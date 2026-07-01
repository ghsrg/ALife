import tomllib

class ValidationError(Exception):
    """Exception raised for configuration validation errors."""
    pass

def load_and_validate_config(toml_str: str) -> dict:
    # 1. Validate TOML format
    try:
        config = tomllib.loads(toml_str)
    except tomllib.TOMLDecodeError as e:
        raise ValidationError(f"Invalid TOML format: {e}")

    # 2. Check for missing required root keys
    required_root_keys = [
        "scenario_id",
        "seed",
        "tick_count",
        "world",
        "space",
        "resources",
        "cell",
        "environment",
        "lifecycle"
    ]
    for key in required_root_keys:
        if key not in config:
            raise ValidationError(f"Missing required root key: '{key}'")

    # 3. Validate cell parameters
    cell = config["cell"]
    if not isinstance(cell, dict):
        raise ValidationError("Root key 'cell' must be a table")

    # Negative cell radius, initial energy, energy capacity, capacity limit, or mandatory cost
    # radius
    if "radius" in cell:
        if not isinstance(cell["radius"], (int, float)):
            raise ValidationError("cell.radius must be a number")
        if cell["radius"] < 0:
            raise ValidationError("cell.radius cannot be negative")

    # initial_energy
    if "initial_energy" in cell:
        if not isinstance(cell["initial_energy"], (int, float)):
            raise ValidationError("cell.initial_energy must be a number")
        if cell["initial_energy"] < 0:
            raise ValidationError("cell.initial_energy cannot be negative")

    # energy_capacity
    if "energy_capacity" in cell:
        if not isinstance(cell["energy_capacity"], (int, float)):
            raise ValidationError("cell.energy_capacity must be a number")
        if cell["energy_capacity"] < 0:
            raise ValidationError("cell.energy_capacity cannot be negative")

    # capacity_limit
    if "capacity_limit" in cell:
        if not isinstance(cell["capacity_limit"], (int, float)):
            raise ValidationError("cell.capacity_limit must be a number")
        if cell["capacity_limit"] < 0:
            raise ValidationError("cell.capacity_limit cannot be negative")

    # mandatory cost (both mandatory_cost and mandatory_cost_per_tick)
    for cost_key in ["mandatory_cost", "mandatory_cost_per_tick"]:
        if cost_key in cell:
            if not isinstance(cell[cost_key], (int, float)):
                raise ValidationError(f"cell.{cost_key} must be a number")
            if cell[cost_key] < 0:
                raise ValidationError(f"cell.{cost_key} cannot be negative")

    # 4. Resource or material capacity limit overflow (resources + materials initial amounts exceed capacity_limit)
    capacity_limit = cell.get("capacity_limit", 0.0)
    resource_type_ids = config.get("resources", {}).get("resource_type_ids", [])
    
    initial_resources = cell.get("initial_resources", {})
    if isinstance(initial_resources, list):
        if len(initial_resources) != len(resource_type_ids):
            raise ValidationError(
                f"Length of cell.initial_resources ({len(initial_resources)}) "
                f"must match length of resources.resource_type_ids ({len(resource_type_ids)})"
            )
    elif isinstance(initial_resources, dict):
        for k in initial_resources.keys():
            if k not in resource_type_ids:
                raise ValidationError(
                    f"Resource '{k}' in cell.initial_resources is not listed in resources.resource_type_ids"
                )
    else:
        raise ValidationError("cell.initial_resources must be a list/array or table/dictionary")
        
    initial_materials = cell.get("initial_materials", {})
    if not isinstance(initial_materials, (list, dict)):
        raise ValidationError("cell.initial_materials must be a list/array or table/dictionary")

    total_initial_amount = 0.0
    if isinstance(initial_resources, list):
        for v in initial_resources:
            if not isinstance(v, (int, float)):
                raise ValidationError("Resource amount must be a number")
            if v < 0:
                raise ValidationError("Resource amount cannot be negative")
            total_initial_amount += v
    else:
        for k, v in initial_resources.items():
            if not isinstance(v, (int, float)):
                raise ValidationError(f"Resource amount for '{k}' must be a number")
            if v < 0:
                raise ValidationError(f"Resource amount for '{k}' cannot be negative")
            total_initial_amount += v

    if isinstance(initial_materials, list):
        for v in initial_materials:
            if not isinstance(v, (int, float)):
                raise ValidationError("Material amount must be a number")
            if v < 0:
                raise ValidationError("Material amount cannot be negative")
            total_initial_amount += v
    else:
        for k, v in initial_materials.items():
            if not isinstance(v, (int, float)):
                raise ValidationError(f"Material amount for '{k}' must be a number")
            if v < 0:
                raise ValidationError(f"Material amount for '{k}' cannot be negative")
            total_initial_amount += v

    if total_initial_amount > capacity_limit:
        raise ValidationError(
            f"Resource and material initial capacity overflow: total initial amount {total_initial_amount} "
            f"exceeds capacity_limit {capacity_limit}"
        )

    # 5. Validate minimum_viability_materials is non-empty
    min_materials = cell.get("minimum_viability_materials", [])
    if not isinstance(min_materials, (list, dict)):
        raise ValidationError("cell.minimum_viability_materials must be a list/array or table/dictionary")
    if len(min_materials) == 0:
        raise ValidationError("cell.minimum_viability_materials cannot be empty")

    # 6. Validate environment thresholds
    env = config.get("environment", {})
    if "heat_warning_threshold" in env and "heat_death_threshold" in env:
        if env["heat_warning_threshold"] >= env["heat_death_threshold"]:
            raise ValidationError("heat_warning_threshold must be strictly less than heat_death_threshold")
            
    if "waste_warning_threshold" in env and "waste_death_threshold" in env:
        if env["waste_warning_threshold"] >= env["waste_death_threshold"]:
            raise ValidationError("waste_warning_threshold must be strictly less than waste_death_threshold")

    return config

