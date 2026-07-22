# Title

# Rationale

Monte Carlo radiative transport is a class of algorithms that simulate many photons as they scatter through a medium, existing in forms ranging from medical imaging [1-3] to proposed spaceborne oceanic lidar [4], however voxel based Monte Carlo (vMC) was chosen specifically. Fundamentally, these algorithms require unbiased accumulation and computing, as small bias compounds over many photons; for example, even full precision floating point rounding error has caused major accuracy degradation in popular implementations such as MCX [5]. This necessitates the use of many full precision floating point intermediary values and double buffered or double precision output structures as used by MCX [2] and FullMonte [3] respectively. This causes issues in low power environments such as satellites or any battery powered application where Field Programmable Gate Arrays (FPGA) are required for their low power draw, as many memory and compute intensive techniques have to be used. This can be solved with stochastic rounding, a technique that rounds numbers with a probability inversely proportional to the distance to their upper and lower bound [6], effectively removing bias and replacing it with variance, an established method in deep learning [7]. Stochastic rounding is highly efficient on FPGAs due to their fast and customizable RNG, and can be implemented in both the output and compute layers. The FPGA in turn allows for both layers to be optimized to arbitrarily low bit widths, potentially allowing an FPGA implementation to increase its overall performance.

# Research Question

1. How far can stochastic rounding facilitate the lowering of bit widths, until variance causes outputs to decay to plus or minus 0.5% of MCX-Cl's error against the f64 baseline?
2. How does the lowering of bit widths affect the subsequent overall algorithmic performance?
3. Does the change in performance allow for a low power FPGA device to viably compute vMC?

# Engineering Goals

Design a vMC program that achieves a significant performance increase over prior implementations through applying stochastic rounding in both the computational and caching systems, allowing low energy environments to run on-board vMC calculations.

# Expected Outcomes

A working implementation of a floating point, fixed point, and fixed point with stochastic rounding implementations written in HLS, with a significant percentage increase in performance of the fixed point over floating point, and a further percentage increase in performance of fixed point with stochastic rounding over regular fixed point due to lower bit widths.

# Materials

Hardware includes a Kria KV260 FPGA Development Board, desktop computer with 32 GB of DDR4 RAM, an AMD Ryzen 5 2600 6 core processor CPU, and access to the AMD RX 6700 XT and NVIDIA RTX 3060 TI graphics cards. Software requirements include the Rust and Python languages, and a free AMD Vitis HLS license.

# Procedures / Methods

1. Implement a double precision floating point reference in Rust of the vMC algorithm, then verify accuracy using the Python package pmcxcl as a baseline.
2. Write custom arbitrary precision (AP) types in Rust that allow for AP integer and floating point numbers.
3. Reimplement each core function and the output energy volume using these AP numbers, ensuring that each number is used for one operation before a new one is created as a result.
4. Calculate the per operation error allowance, or the total function's allowed RMSE divided by the amount of operations (! I need a source on this, im probably wrong).
5. Sweep each individual value's bit width, recording the total function's RMSE against the double precision baseline, ensuring that the same RNG seed is used to make all change in RMSE precision sourced.
6. Select the bit width of each operation as the lowest width within the per operation RMSE allowance.
7. Run final function with all new bit widths selected, comparing final RMSE against allowed RMSE, manually raising bit widths if RMSE is exceeded.
8. Change AP types to round stochastically and repeat steps 5 to 7, recording new bit widths
9. Implement f32 HLS baseline on FPGA and verify implementation is within the required accuracy compared to baseline
10. Implement HLS version using AP types, setting bit widths to non-stochastic rounding values as gathered in the software sweep
11. Implement final HLS version using AP types and stochastic rounding, setting bit widths to stochastic rounding widths as measured in software sweep. 
12. Run each HLS version 10 times, recording final output energy grid along with power draw, photons computed, and time spent.

# Risk and Safety

FPGA board requires a power supply of 12V with a maximum of 3 A, presenting no direct harm. The board is capable of running hot, so care will be taken to avoid contact during compute. The board features a static bag to prevent static build up and damage, however additional care will be taken not to cause static damage. Additionally, no hazardous chemicals are used, and no human surveys of any kind will be conducted.

# Data Analysis

Bit sweep and HLS trial data will be exported into raw binary files which will be analyzed in Python through NumPy and matplotlib. For the bit sweep, the RMSE of the total function as each variable is swept will be recorded, graphed to show the correlation between bit width and individual operation RMSE. For arbitrary floating point numbers, both their mantissa and exponent bits will be swept as separate variables. Once all variables have been brought to the lowest width that maintains their individual RMSE threshold, the combined function will be ran and its total RMSE will be recorded. The HLS data will be analyzed on its power draw, photons calculated, time taken, and total RMSE of the output energy grid. Each set of values will be analyzed over their 10 runs, comparing their mean and median to find representative values, where the final results of each method can be compared.

# Bibliography