# pmcx_utils
import json, numpy as np, pmcx
from pathlib import Path
import math

SCENES_ROOT = Path(__file__).resolve().parent.parent

# load json
def load_json(config):
    json_cfg = json.load(open(config))
    cfg = pmcx.json2mcx(json_cfg)

    cfg["vol"] = cfg["vol"].astype(np.uint8)

    # calculate the raw gates from the input file with the mcx calculation
    raw_gates = (cfg["tend"] - cfg["tstart"]) / cfg["tstep"]
    cfg["maxgate"] = math.ceil(round(raw_gates, 9))

    return cfg

def run_write_pmcx_result(cfg, out_path):
    res = pmcx.run(cfg)
    flux = res["flux"].astype(np.float32)

    out_path = Path(out_path)
    out_path.parent.mkdir(exist_ok=True, parents=True)

    out_path.write_bytes(flux.tobytes(order="F"))

    return res