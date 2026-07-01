def evaluate_static_bounds(config: dict) -> tuple[str, str]:
    """
    Evaluates basic static energy, capacity, heat, and waste budgets.
    
    Returns:
        tuple[str, str]: (survival_result, collapse_reason)
    """
    # 1. Mandatory cost check
    cell = config.get("cell", {})
    initial_energy = cell.get("initial_energy", 0.0)
    
    resources = config.get("resources", {})
    passive_income = resources.get("passive_energy_income_placeholder", 0.0)
    
    mandatory_cost = cell.get("mandatory_cost_per_tick", cell.get("mandatory_cost", 0.0))
    
    if initial_energy + passive_income < mandatory_cost:
        return "collapse", "mandatory_cost_unpaid"

    # 2. Initial capacity check
    capacity_limit = cell.get("capacity_limit", 0.0)
    initial_resources = cell.get("initial_resources", {})
    initial_materials = cell.get("initial_materials", {})
    
    # Support dict or list/iterable summation
    sum_resources = sum(initial_resources.values()) if isinstance(initial_resources, dict) else sum(initial_resources)
    sum_materials = sum(initial_materials.values()) if isinstance(initial_materials, dict) else sum(initial_materials)
    
    if sum_resources + sum_materials > capacity_limit:
        return "collapse", "capacity_exceeded"

    # 3. Heat accumulation check
    env = config.get("environment", {})
    heat_gen = env.get("heat_generated_per_tick", 0.0)
    heat_diss = env.get("heat_dissipation_rate", 0.0)
    
    if heat_gen > heat_diss:
        return "collapse", "heat_limit_exceeded"

    # 4. Waste accumulation check
    waste_gen = env.get("waste_generated_per_tick", 0.0)
    waste_sink = env.get("waste_sink_rate", 0.0)
    
    if waste_gen > waste_sink:
        return "collapse", "waste_limit_exceeded"

    return "stable", "none"
