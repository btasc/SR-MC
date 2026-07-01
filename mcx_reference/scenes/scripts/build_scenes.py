# build_scenes
from pmcx_utils import load_json, SCENES_ROOT
import numpy as np
from pathlib import Path

scenes = [
    "cube60g10",
]

vol_dir = SCENES_ROOT / "cache" / "vol"
vol_dir.mkdir(parents=True, exist_ok=True)

for s in scenes:
    config_path = SCENES_ROOT / f"{s}.json"
    cfg = load_json(config_path)
    vol = cfg["vol"]

    out_path = vol_dir / f"{s}.bin"
    out_path.write_bytes(vol.tobytes(order="F"))

    print(f"Finished building {s}")