def run_micro_simulation(config: dict) -> tuple[list, str, str]:
    """
    Runs a micro simulation for the cell over a fixed tick count.
    
    Returns:
        tuple[list, str, str]: (history, survival_result, collapse_reason)
    """
    cell = config.get("cell", {})
    env = config.get("environment", {})
    lifecycle = config.get("lifecycle", {})
    resources = config.get("resources", {})

    energy = cell.get("initial_energy", 0.0)
    energy_capacity = cell.get("energy_capacity", 0.0)
    mandatory_cost = cell.get("mandatory_cost_per_tick", cell.get("mandatory_cost", 0.0))
    dormant_modifier = cell.get("dormant_mandatory_cost_modifier", 0.0)
    capacity_limit = cell.get("capacity_limit", 0.0)
    
    # Calculate free capacity once since initial resources/materials are constant in micro simulator
    initial_resources = cell.get("initial_resources", {})
    initial_materials = cell.get("initial_materials", {})
    sum_resources = sum(initial_resources.values()) if isinstance(initial_resources, dict) else sum(initial_resources)
    sum_materials = sum(initial_materials.values()) if isinstance(initial_materials, dict) else sum(initial_materials)
    free_capacity = capacity_limit - (sum_resources + sum_materials)

    passive_energy_income = resources.get("passive_energy_income_placeholder", 0.0)

    heat = env.get("heat_current", 0.0)
    heat_gen = env.get("heat_generated_per_tick", 0.0)
    heat_diss = env.get("heat_dissipation_rate", 0.0)
    heat_warning = env.get("heat_warning_threshold", 0.0)
    heat_death = env.get("heat_death_threshold", 0.0)

    waste = env.get("waste_current", 0.0)
    waste_gen = env.get("waste_generated_per_tick", 0.0)
    waste_sink = env.get("waste_sink_rate", 0.0)
    waste_warning = env.get("waste_warning_threshold", 0.0)
    waste_death = env.get("waste_death_threshold", 0.0)

    stress_energy_threshold = lifecycle.get("stress_energy_threshold", 0.0)
    dormancy_allowed = lifecycle.get("dormancy_allowed", False)

    current_state = "alive"
    history = []
    warning_triggered = False
    collapse_reason = "none"

    for tick in range(1, config.get("tick_count", 0) + 1):
        energy_next = energy + passive_energy_income
        heat_next = max(0.0, heat + heat_gen - heat_diss)
        waste_next = max(0.0, waste + waste_gen - waste_sink)

        # Wake up check
        if current_state == "dormant":
            can_pay_full = energy_next >= mandatory_cost
            energy_after_full = min(energy_capacity, energy_next - mandatory_cost)
            is_stressed_after_full = (
                (energy_after_full < stress_energy_threshold) or 
                (heat_next > heat_warning) or 
                (waste_next > waste_warning) or 
                (free_capacity < 0)
            )
            if can_pay_full and not is_stressed_after_full:
                current_state = "alive"

        # Apply mandatory cost
        if current_state == "dormant":
            cost = mandatory_cost * dormant_modifier
        else:
            cost = mandatory_cost

        mandatory_paid = energy_next >= cost
        if mandatory_paid:
            energy_next = min(energy_capacity, energy_next - cost)

        # Check death conditions
        is_dead = False
        if energy_next <= 0.0:
            is_dead = True
            collapse_reason = "energy_depleted"
        elif heat_next > heat_death:
            is_dead = True
            collapse_reason = "heat_limit_exceeded"
        elif waste_next > waste_death:
            is_dead = True
            collapse_reason = "waste_limit_exceeded"
        elif not mandatory_paid:
            dormant_cost = mandatory_cost * dormant_modifier
            can_go_dormant = (current_state != "dormant") and dormancy_allowed and ((energy + passive_energy_income) >= dormant_cost)
            if can_go_dormant:
                current_state = "dormant"
                energy_next = min(energy_capacity, (energy + passive_energy_income) - dormant_cost)
            else:
                is_dead = True
                collapse_reason = "mandatory_cost_unpaid"

        if is_dead:
            current_state = "dead"
            history.append({
                "tick": tick,
                "state": "dead",
                "energy": 0.0,
                "heat": heat_next,
                "waste": waste_next
            })
            break
        else:
            energy = energy_next
            heat = heat_next
            waste = waste_next

            # Check warning/stress thresholds
            if current_state == "dormant":
                warning_triggered = True
            else:
                is_stressed = (
                    (energy < stress_energy_threshold) or 
                    (heat > heat_warning) or 
                    (waste > waste_warning) or 
                    (free_capacity < 0)
                )
                if is_stressed:
                    current_state = "stressed"
                    warning_triggered = True
                else:
                    current_state = "alive"

            history.append({
                "tick": tick,
                "state": current_state,
                "energy": energy,
                "heat": heat,
                "waste": waste
            })

    if current_state == "dead":
        survival_result = "collapse"
    else:
        if warning_triggered:
            survival_result = "fragile"
        else:
            survival_result = "stable"

    return history, survival_result, collapse_reason
