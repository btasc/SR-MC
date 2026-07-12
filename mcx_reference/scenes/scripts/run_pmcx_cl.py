import numpy as np
import json, pmcxcl
from load_mcx_config import load_mcx_config

scenes_to_run = [
    "cube60g1"
]

for scene in scenes_to_run:
    scene_path = f"../{scene}.json"

    vol = np.load(f"../cache/vol_{scene}.npy")
    cfg = load_mcx_config(scene_path, vol)

    res = pmcxcl.run(cfg)

    flux = res['flux']
    np.save(f"../cache/flux_{scene}_cl.npy", flux)