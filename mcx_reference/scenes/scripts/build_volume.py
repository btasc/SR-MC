import numpy as np
import json

scenes_to_build = [
    "cube60"
]

for scene in scenes_to_build:
    scene_path = f"../{scene}.json"

    with open(scene_path) as f:
        cfg = json.load(f)

    dims = cfg['Domain']['Dim']
    vol = np.zeros(dims, dtype=np.uint8)

    for s in cfg["Shapes"]:
        for shape_type, shape_val in s.items():
            match shape_type:
                case "Grid":
                    tag = shape_val["Tag"]
                    size = shape_val["Size"]
                    vol[:size[0], :size[1], :size[2]] = tag
                case _:
                    raise NotImplementedError(f"Shape {shape_type} not implemented")

    np.save(f"../cache/vol_{scene}.npy", vol)