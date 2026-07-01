import numpy as np
import pmcx

file_path = "test"

# dimensions
nx, ny, nz = 60, 60, 60

# read volume file with 8 bit material indices
volume = np.fromfile(file_path, dtype=np.uint8).reshape([nx, ny, nz], order='F')