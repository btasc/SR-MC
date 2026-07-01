# run_scenes
from pmcx_utils import load_json, run_write_pmcx_result, SCENES_ROOT
from pathlib import Path

scenes = [
    "cube60g10",
]

out_dir = SCENES_ROOT / "cache" / "out"

for s in scenes:
    config_path = SCENES_ROOT / f"{s}.json"
    cfg = load_json(config_path)

    out_path = out_dir / f"{s}.bin"
    # already runs mkdir inside of write function, so its not needed here
    run_write_pmcx_result(cfg, out_path)