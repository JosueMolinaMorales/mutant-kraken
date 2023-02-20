import json
types = []
with open("KotlinTypes.json", "r") as f:
    obj = json.loads(f.read())
    for t in obj:
        if not t["named"]:
            continue
        name = t["type"]
        name = "".join([ n.capitalize() for n in name.split("_") ])
        print(name)
        types.append(name)
open("KotlinNamedTypes.json", "w").write(str(types))