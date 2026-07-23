# Title

Effects of Stochastic Rounding in Voxelized Monte Carlo Photon Transport on Low Power FPGA Devices.

# Rationale

Monte Carlo radiative transport is a class of algorithms that simulate many photons as they scatter through a medium, existing in forms ranging from medical imaging [1-3] to proposed spaceborne oceanic lidar [4], however voxel based Monte Carlo (vMC) was chosen specifically. Fundamentally, these algorithms require unbiased accumulation and computing, as small bias compounds over many photons; for example, even full precision floating point rounding error has caused major accuracy degradation in popular implementations such as MCX [5]. This necessitates the use of many full precision floating point intermediary values and double buffered or double precision output structures as used by MCX [2] and FullMonte [3] respectively. This causes issues in low power environments such as satellites or any battery powered application where Field Programmable Gate Arrays (FPGA) are required for their low power draw, as many memory and compute intensive techniques have to be used. This can be solved with stochastic rounding, a technique that rounds numbers with a probability inversely proportional to the distance to their upper and lower bound [6], effectively removing bias and replacing it with variance, an established method in deep learning [7]. Stochastic rounding is highly efficient on FPGAs due to their fast and customizable RNG, and can be implemented in both the output and compute layers. The FPGA in turn allows for both layers to be optimized to arbitrarily low bit widths, potentially allowing an FPGA implementation to increase its overall performance.

# Research Question

1. How far can stochastic rounding facilitate the lowering of bit widths, until variance causes outputs to decay to plus or minus 0.5% of MCX-Cl's error against the f64 baseline?
2. How does the lowering of bit widths affect the subsequent overall algorithmic performance?
3. Does the change in performance allow for a low power FPGA device to viably compute vMC?

# Engineering Goals

Design a vMC program that achieves a significant percentage performance increase over both previous floating point and fixed point HLS implementations through applying stochastic rounding in both the output and compute layers, allowing low energy environments to run on-board vMC calculations.

# Expected Outcomes

A working implementation of a floating point, fixed point, and fixed point with stochastic rounding implementations written in HLS, with a significant percentage increase in performance of the fixed point over floating point, and a further percentage increase in performance of fixed point with stochastic rounding over regular fixed point due to lower bit widths.

# Materials

Hardware includes a Kria KV260 FPGA Development Board, desktop computer with 32 GB of DDR4 RAM, an AMD Ryzen 5 2600 6 core processor CPU, and access to the AMD RX 6700 XT and NVIDIA RTX 3060 TI graphics cards. Software requirements include the Rust and Python languages, and a free AMD Vitis HLS license.

# Procedures / Methods - Last TODO

1. Implement a double precision floating point reference in Rust of the vMC algorithm, then verify accuracy using the Python package pmcxcl as a validation.
2. Write custom arbitrary precision (AP) types in Rust that allow for AP integer and floating point numbers. Types have the option of the rounding modes round-to-nearest (RN) and stochastic rounding (SR).
3. Reimplement each core function and the output energy volume using these AP numbers, along with a reference using f64 values.
4. Run this f64 references 10 times to establish an ultra accurate baseline to compare to. Translate output to be in logarithmic format to avoid high values skewing data.
5. Run pmcxcl GPU implementation 25 times, logging its percent change (with percents keeping the sign of the direction of the percent change)
6. Find the mean of the sum of the absolute values of these percents to calculate variance, and find the total sum to calculate bias.
7. Test how each function effects overall accuracy and bias, allocating allowed variance and bias amounts to each function.
8. Divide out accuracy (TODO)
9. Sweep each number's bit width in each function, logging the function's variance and bias as each number is represented in all bit widths
10. Choose the number's lowest bit width that maintains the allowed operations variance and bias, running the final combined function with all bit widths optimized and manually adjusting widths if variance and bias target is not hit.
11. Repeat steps 8-10 with stochastic rounding
12. Implement HLS f32 version of MCX algorithm, verifying data export methods and output accuracy
13. Implement arbitrary point version of MCX algorithm, setting bit widths to ideal bit widths as found in the RN bit sweep
14. Implement arbitrary point version of MCX algorithm with stochastic rounding, setting bit widths to ideal widths as found in the SR sweep.

# Risk and Safety

FPGA board requires a power supply of 12V with a maximum of 3 A, presenting no direct harm. The board is capable of running hot, so care will be taken to avoid contact during compute. The board features a static bag to prevent static build up and damage, however additional care will be taken not to cause static damage. Additionally, no hazardous chemicals are used, and no human surveys of any kind will be conducted.

# Data Analysis

Bit sweep and HLS trial data will be exported into raw binary files which will be analyzed in Python through NumPy and matplotlib. For the bit sweep, the variance and bias of the total function as each variable is swept will be recorded, graphed to show the correlation between bit width and individual operation variance and bias. For arbitrary floating point numbers, both their mantissa and exponent bits will be swept as separate variables. Once all variables have been brought to the lowest width that maintains their individual RMSE threshold, the combined function will be ran and its total variance and bias will be recorded. The HLS data will be analyzed on its power draw, photons calculated, time taken, and total variance and bias of the output energy grid. Each set of values will be analyzed over their 10 runs, comparing their mean and median to find representative values, where the final results of each method can be compared.

# Bibliography