import tomllib


def serialize_value(value):
    if isinstance(value, bool):
        return str(value).lower()
    if isinstance(value, str):
        return f'"{value}"'
    if isinstance(value, list):
        return "[" + ", ".join(serialize_value(v) for v in value) + "]"
    if isinstance(value, dict):
        parts = [f"{k} = {serialize_value(v)}" for k, v in sorted(value.items())]
        return "{ " + ", ".join(parts) + " }"
    return str(value)


def dict_to_toml(data: dict) -> str:
    lines = []
    for key, value in sorted(data.items()):
        if not isinstance(value, dict):
            lines.append(f"{key} = {serialize_value(value)}")
    for key, value in sorted(data.items()):
        if isinstance(value, dict):
            lines.append("")
            lines.append(f"[{key}]")
            for sub_key, sub_value in sorted(value.items()):
                lines.append(f"{sub_key} = {serialize_value(sub_value)}")
    return "\n".join(lines) + "\n"


def mutate_toml(base_toml: str, path: str, value):
    data = tomllib.loads(base_toml)
    parts = path.split(".")
    curr = data
    for part in parts[:-1]:
        curr = curr[part]
    curr[parts[-1]] = value
    return dict_to_toml(data)
