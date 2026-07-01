import numpy as np
from pathlib import Path

nx, ny, nz = 60, 60, 60

def default_vol():
    return np.ones((nx, ny, nz), dtype=np.uint8)

root_dir = Path(__file__).resolve().parent

def write_vol(scene_dir, vol):
    out_dir = root_dir / scene_dir
    out_dir.mkdir(exist_ok=True)

    volume_path = out_dir / "volume.bin"
    volume_path.write_bytes(vol.astype(np.uint8).tobytes(order="F"))

write_vol("cube60", default_vol())