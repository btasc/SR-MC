# Simulation parameters

## Hardware

I chose to use the Kria KV260 FPGA board to run my HLS version on. It features around 2.3 MB of URAM, a type of RAM that is more plentiful than BRAM while below DDR, allowing me to store a large cache that would otherwise use the limited BRAM. On the GPU side, I plan to compare against the 3060 as it's the only Nvidia card that I have physical access to. I may also use my RX 6700 XT AMD graphics card, however I worry that it would require me to use a different version of MCX, say the OpenCL version, rather than the CUDA version, adding a potential bias.

# State

- Photon position
The photon position is broken into two variables, `global_pos` and `local_pos`, although `local_pos` is abbreviated as just `pos` in some places. `global_pos` represents the global voxel idx, stored in regular uint8 to represent a max scene of 256x256x256. `local_pos` represents the in-voxel coordinate from 0 to 1 in fixed point. An issue is encountered when the photon is exactly on a voxel boundary. When the photon comes from the negative direction, its position is set to 0.0, however it should be at 1.0. To fix this, it just has to be specifically set to its max value, as the precision loss from 1.0 to 1 - 2^-N is basically negligible, however there could be some problems as that gap grows as the precision of pos is lowered.