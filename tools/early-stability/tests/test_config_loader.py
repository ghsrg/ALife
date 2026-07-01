import pytest
import tomllib

from config_loader import load_and_validate_config, ValidationError

VALID_TOML = """
scenario_id = "test_scenario"
seed = 12345
tick_count = 100

[world]
size = { width = 256, height = 256 }
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = "uniform"

[cell]
initial_position = [128.0, 128.0]
radius = 1.0
initial_resources = { nutrient_A = 2.0 }
initial_materials = { cell_wall_material = 3.0 }
initial_energy = 10.0
energy_capacity = 100.0
mandatory_cost_per_tick = 0.5
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 10.0

[environment]
ambient_temperature = 20.0
heat_dissipation_rate = 1.0
waste_sink_rate = 1.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"""

def test_valid_config():
    # A valid configuration should parse successfully
    config = load_and_validate_config(VALID_TOML)
    assert config["scenario_id"] == "test_scenario"
    assert config["seed"] == 12345
    assert config["tick_count"] == 100
    assert config["cell"]["radius"] == 1.0

def test_invalid_toml():
    # Invalid TOML syntax should raise ValidationError
    invalid_toml = """
    scenario_id = "test_scenario
    seed = 12345
    """
    with pytest.raises(ValidationError) as excinfo:
        load_and_validate_config(invalid_toml)
    assert "TOML" in str(excinfo.value) or "parse" in str(excinfo.value)

@pytest.mark.parametrize("missing_key", [
    "scenario_id", "seed", "tick_count", "world", "space", "resources", "cell", "environment", "lifecycle"
])
def test_missing_required_root_keys(missing_key):
    # Removing any required root key should raise ValidationError
    # Parse the valid TOML first, delete the key, then reconstruct/validate
    import tomllib
    config_dict = tomllib.loads(VALID_TOML)
    del config_dict[missing_key]
    
    # We can reconstruct a simple TOML or write a helper to load it.
    # Since we can implement load_and_validate_config to accept dict or string, let's keep it accepting string as specified.
    # To convert dict back to TOML, we can write a simple custom serializer or use a helper,
    # or just delete the lines from the string.
    # Actually, let's just make load_and_validate_config accept string. To serialize dict to TOML without external library:
    # Python stdlib doesn't have tomllib writer. So we can convert dict to TOML string manually for simple cases,
    # or let's have load_and_validate_config also support a dict/parsed object if needed, or we serialize manually:
    def dict_to_toml(d):
        lines = []
        for k, v in d.items():
            if isinstance(v, dict):
                lines.append(f"[{k}]")
                for subk, subv in v.items():
                    if isinstance(subv, dict):
                        # Simple inline table or nested table
                        lines.append(f"{subk} = {str(subv).replace(':', ' =').replace('{', '{ ').replace('}', ' }')}")
                    elif isinstance(subv, str):
                        lines.append(f'{subk} = "{subv}"')
                    elif isinstance(subv, bool):
                        lines.append(f'{subk} = {str(subv).lower()}')
                    else:
                        lines.append(f'{subk} = {subv}')
            elif isinstance(v, list):
                lines.append(f"{k} = {repr(v)}")
            elif isinstance(v, str):
                lines.append(f'{k} = "{v}"')
            elif isinstance(v, bool):
                lines.append(f'{k} = {str(v).lower()}')
            else:
                lines.append(f'{k} = {v}')
        return "\n".join(lines)

    toml_str = dict_to_toml(config_dict)
    with pytest.raises(ValidationError) as excinfo:
        load_and_validate_config(toml_str)
    assert f"missing" in str(excinfo.value).lower() or missing_key in str(excinfo.value).lower()

@pytest.mark.parametrize("cell_key, negative_value", [
    ("radius", -1.0),
    ("initial_energy", -10.0),
    ("energy_capacity", -5.0),
    ("capacity_limit", -1.0),
    ("mandatory_cost_per_tick", -0.5),
    ("mandatory_cost", -0.5), # We support either field
])
def test_negative_cell_fields(cell_key, negative_value):
    import tomllib
    config_dict = tomllib.loads(VALID_TOML)
    
    # Setup the negative value
    if cell_key == "mandatory_cost":
        # Delete mandatory_cost_per_tick if exists, and set mandatory_cost to negative
        if "mandatory_cost_per_tick" in config_dict["cell"]:
            del config_dict["cell"]["mandatory_cost_per_tick"]
        config_dict["cell"]["mandatory_cost"] = negative_value
    else:
        config_dict["cell"][cell_key] = negative_value
        
    def dict_to_toml(d):
        lines = []
        for k, v in d.items():
            if isinstance(v, dict):
                lines.append(f"[{k}]")
                for subk, subv in v.items():
                    if isinstance(subv, dict):
                        lines.append(f"{subk} = {str(subv).replace(':', ' =').replace('{', '{ ').replace('}', ' }')}")
                    elif isinstance(subv, str):
                        lines.append(f'{subk} = "{subv}"')
                    elif isinstance(subv, bool):
                        lines.append(f'{subk} = {str(subv).lower()}')
                    else:
                        lines.append(f'{subk} = {subv}')
            elif isinstance(v, list):
                lines.append(f"{k} = {repr(v)}")
            elif isinstance(v, str):
                lines.append(f'{k} = "{v}"')
            elif isinstance(v, bool):
                lines.append(f'{k} = {str(v).lower()}')
            else:
                lines.append(f'{k} = {v}')
        return "\n".join(lines)

    toml_str = dict_to_toml(config_dict)
    with pytest.raises(ValidationError) as excinfo:
        load_and_validate_config(toml_str)
    assert "negative" in str(excinfo.value).lower() or cell_key in str(excinfo.value).lower() or "mandatory_cost" in str(excinfo.value).lower()

def test_capacity_limit_overflow():
    # Resource or material capacity limit overflow (resources + materials initial amounts exceed capacity_limit)
    # Under cell: capacity_limit = 10.0
    # Let's make initial_resources + initial_materials sum exceed 10.0, e.g. 6.0 + 5.0 = 11.0
    import tomllib
    config_dict = tomllib.loads(VALID_TOML)
    config_dict["cell"]["capacity_limit"] = 10.0
    config_dict["cell"]["initial_resources"] = { "nutrient_A": 6.0 }
    config_dict["cell"]["initial_materials"] = { "cell_wall_material": 5.0 }
    
    def dict_to_toml(d):
        lines = []
        for k, v in d.items():
            if isinstance(v, dict):
                lines.append(f"[{k}]")
                for subk, subv in v.items():
                    if isinstance(subv, dict):
                        lines.append(f"{subk} = {str(subv).replace(':', ' =').replace('{', '{ ').replace('}', ' }')}")
                    elif isinstance(subv, str):
                        lines.append(f'{subk} = "{subv}"')
                    elif isinstance(subv, bool):
                        lines.append(f'{subk} = {str(subv).lower()}')
                    else:
                        lines.append(f'{subk} = {subv}')
            elif isinstance(v, list):
                lines.append(f"{k} = {repr(v)}")
            elif isinstance(v, str):
                lines.append(f'{k} = "{v}"')
            elif isinstance(v, bool):
                lines.append(f'{k} = {str(v).lower()}')
            else:
                lines.append(f'{k} = {v}')
        return "\n".join(lines)

    toml_str = dict_to_toml(config_dict)
    with pytest.raises(ValidationError) as excinfo:
        load_and_validate_config(toml_str)
    assert "overflow" in str(excinfo.value).lower() or "capacity" in str(excinfo.value).lower()

def test_capacity_limit_boundary():
    # Exactly matching capacity_limit should be fine
    import tomllib
    config_dict = tomllib.loads(VALID_TOML)
    config_dict["cell"]["capacity_limit"] = 10.0
    config_dict["cell"]["initial_resources"] = { "nutrient_A": 5.0 }
    config_dict["cell"]["initial_materials"] = { "cell_wall_material": 5.0 }
    
    def dict_to_toml(d):
        lines = []
        for k, v in d.items():
            if isinstance(v, dict):
                lines.append(f"[{k}]")
                for subk, subv in v.items():
                    if isinstance(subv, dict):
                        lines.append(f"{subk} = {str(subv).replace(':', ' =').replace('{', '{ ').replace('}', ' }')}")
                    elif isinstance(subv, str):
                        lines.append(f'{subk} = "{subv}"')
                    elif isinstance(subv, bool):
                        lines.append(f'{subk} = {str(subv).lower()}')
                    else:
                        lines.append(f'{subk} = {subv}')
            elif isinstance(v, list):
                lines.append(f"{k} = {repr(v)}")
            elif isinstance(v, str):
                lines.append(f'{k} = "{v}"')
            elif isinstance(v, bool):
                lines.append(f'{k} = {str(v).lower()}')
            else:
                lines.append(f'{k} = {v}')
        return "\n".join(lines)

    toml_str = dict_to_toml(config_dict)
    config = load_and_validate_config(toml_str)
    assert config["cell"]["capacity_limit"] == 10.0
