import numpy as np
import json, pmcxcl

scenes_to_run = [
    # (config_name, volume_name)
    ("cube60g1", "cube60")
]

for scene in scenes_to_run:
    config_path = f"../configs/{scene[0]}.json"
    volume_path = f"../volumes/{scene[1]}.bin"
    energy_path = f"../energies/{scene[1]}_cl.npy"

    cfg = pmcxcl.json2mcx(config_path)
    vol = np.fromfile(volume_path, dtype=np.uint8).reshape(cfg["dims"], order="F")

    cfg["vol"] = vol
    res = pmcxcl.run(cfg)

    flux = res["flux"]
    np.save(energy_path, flux)