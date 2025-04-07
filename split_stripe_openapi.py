import yaml
import os
import re
from collections import defaultdict, deque


def find_refs(obj, refs_set):
    """
    Recursively finds all schema references (#/components/schemas/...)
    within a given Python object (dict, list, str) and adds the
    schema name to the provided set.
    """
    if isinstance(obj, dict):
        for key, value in obj.items():
            if (
                key == "$ref"
                and isinstance(value, str)
                and value.startswith("#/components/schemas/")
            ):
                schema_name = value.split("/")[-1]
                refs_set.add(schema_name)
            else:
                find_refs(value, refs_set)
    elif isinstance(obj, list):
        for item in obj:
            find_refs(item, refs_set)


def get_dependent_schemas(initial_schemas, all_schemas):
    """
    Resolves all dependent schemas starting from an initial set.
    Performs a breadth-first search through schema references.
    """
    required_schemas = set(initial_schemas)
    queue = deque(initial_schemas)
    processed = set(initial_schemas)  # Keep track of schemas already checked

    while queue:
        schema_name = queue.popleft()
        if schema_name not in all_schemas:
            # print(f"      Warning: Referenced schema '{schema_name}' not found in components/schemas.")
            continue  # Skip schemas that aren't defined

        schema_def = all_schemas[schema_name]
        dependent_refs = set()
        find_refs(
            schema_def, dependent_refs
        )  # Find schemas referenced by *this* schema

        for ref_name in dependent_refs:
            if ref_name not in processed:
                required_schemas.add(ref_name)
                processed.add(ref_name)
                queue.append(ref_name)  # Add newly found dependencies to the queue

    return required_schemas


# --- Main Script ---

input_filename = "stripe-openapi.yml"  # Name of the input OpenAPI file
output_dir = "split_openapi_refined"  # Directory to save the split files

# Load the YAML content from the specified file
try:
    print(f"Loading OpenAPI specification from '{input_filename}'...")
    with open(input_filename, "r", encoding="utf-8") as f:
        openapi_data = yaml.safe_load(f)
    print("Successfully loaded OpenAPI specification.")
except FileNotFoundError:
    print(f"Error: Input file '{input_filename}' not found in the current directory.")
    exit(1)
except yaml.YAMLError as e:
    print(f"Error parsing YAML from '{input_filename}': {e}")
    exit(1)
except Exception as e:
    print(f"An unexpected error occurred while reading '{input_filename}': {e}")
    exit(1)

# Basic structure checks
if (
    not isinstance(openapi_data, dict)
    or "paths" not in openapi_data
    or "components" not in openapi_data
    or "schemas" not in openapi_data.get("components", {})
):
    print(
        "Invalid OpenAPI structure: Missing 'paths' or 'components/schemas'. Cannot proceed."
    )
    exit(1)

all_paths = openapi_data.get("paths", {})
all_schemas = openapi_data.get("components", {}).get("schemas", {})
resources = defaultdict(list)  # Use defaultdict(list) to easily append paths

# --- Refined Path Grouping ---
print("Grouping paths by resource (using the segment after '/v1/')...")
for path, path_item in all_paths.items():
    if not path.startswith("/v1/"):
        # print(f"  Info: Path '{path}' does not start with '/v1/' and will be skipped.")
        continue

    # Split path: /v1/accounts/{id} -> ['v1', 'accounts', '{id}']
    parts = path.strip("/").split("/")

    if len(parts) > 1:
        # Use the second part (index 1) as the resource key
        resource_key = parts[1]

        # Basic check to avoid keys like '{customer_id}' if path structure is unusual
        if "{" in resource_key and "}" in resource_key:
            print(
                f"  Warning: Path '{path}' seems to have a parameter as the main resource segment ('{resource_key}'). Skipping this path."
            )
            continue

        # Append the original path string and its definition to the list for this resource key
        resources[resource_key].append((path, path_item))
        # print(f"  Assigned '{path}' to resource '{resource_key}'") # Uncomment for debugging
    else:
        # Handles cases like just '/v1' if it exists, though unlikely in REST APIs
        print(
            f"  Warning: Path '{path}' is too short after '/v1/' and will be skipped."
        )

if not resources:
    print(
        "No resources found matching the '/v1/resource/...' pattern. No files will be generated."
    )
    exit(0)

# Create output directory if it doesn't exist
os.makedirs(output_dir, exist_ok=True)
print(f"Output directory '{output_dir}' ensured.")

# --- Create individual files for each resource ---
print("Processing resources and creating split files...")
for resource_name, path_items in resources.items():
    print(f"  Processing resource: {resource_name}...")
    # Create a dictionary of paths for this resource from the list of tuples
    resource_paths = dict(path_items)
    initial_schemas = set()

    # Find schemas directly referenced in *all* paths/operations for this resource
    find_refs(resource_paths, initial_schemas)
    print(f"    Found {len(initial_schemas)} initially referenced schemas.")

    # Find all dependent schemas recursively based on the initial set
    required_schema_names = get_dependent_schemas(initial_schemas, all_schemas)
    print(
        f"    Resolved to {len(required_schema_names)} total required schemas (including dependencies)."
    )

    # Ensure the 'error' schema is included if it exists globally, as it's common
    if "error" in all_schemas:
        required_schema_names.add("error")
        # print("    Ensured 'error' schema is included.") # Uncomment for debugging

    # Build the new OpenAPI structure for this resource file
    resource_spec = {
        # Copy essential top-level info
        "openapi": openapi_data.get("openapi", "3.0.0"),
        "info": openapi_data.get(
            "info",
            {
                "title": f"{resource_name.capitalize()} API Resource",
                "version": "unknown",
            },
        ),
        "servers": openapi_data.get("servers", []),
        # Add the collected paths for this resource
        "paths": resource_paths,
        # Add the resolved schemas and global security schemes
        "components": {
            "schemas": {
                name: all_schemas[name]
                for name in sorted(list(required_schema_names))
                if name in all_schemas
            },  # Sort for consistency
            "securitySchemes": openapi_data.get("components", {}).get(
                "securitySchemes", {}
            ),
        },
    }
    # Copy top-level security requirement definitions if they exist
    if "security" in openapi_data:
        resource_spec["security"] = openapi_data["security"]

    # Write the resource-specific file
    output_filename = os.path.join(output_dir, f"{resource_name}.yml")
    try:
        with open(output_filename, "w", encoding="utf-8") as f:
            # Dump YAML with options for better readability
            yaml.dump(
                resource_spec,
                f,
                sort_keys=False,
                allow_unicode=True,
                default_flow_style=False,
                width=120,
            )
        print(f"    Successfully created {output_filename}")
    except Exception as e:
        print(f"    Error writing file {output_filename}: {e}")

print(f"\nProcessing complete. Split files are saved in the '{output_dir}' directory.")
