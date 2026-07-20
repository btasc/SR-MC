# Title

# Rationale

In voxelizied Monte Carlo photon transport simulations, ref

In Monte Carlo simulations, specifically voxelized Monte Carlo photon transport, referred to as VMC from now on, a simulation generally consists of many tiny adds to the global accumulator grid. As a result, VMC grids require their per grid storage type to have a very large bit width capable of storing many precise values. This puts a large strain on memory use, along with requiring large intermediary datatypes, causing performance difficulties. Stochastic rounding is a technique to fix specifically this issue through trading bias with variance, and can be applied in both the VMC grid accumulator along with the intermediary datatypes, allowing both to be implemented in a lower width datatype. This helps both computational performance and memory strain. In order to facilitate both high performance stochastic rounding and arbitrary width numbers, an FPGA is the hardware of choice. It features extreme customizability, allowing for tailored RNG and fine control over math.

# Research Question

1. How far can stochastic rounding facilitate the lowering of bit widths until variance causes outputs to significantly decay?
2. How does the lowering of bit widths effect the subsequent overall algorithmic performance?
3. Does the change in performance allow for a low power FPGA device to viably compute VMC?

[Deep Learning with Limited Numerical Precision](https://www.researchgate.net/profile/Ankur-Agrawal-18/publication/272195143_Deep_Learning_with_Limited_Numerical_Precision/links/551952760cf273292e7148bc/Deep-Learning-with-Limited-Numerical-Precision.pdf)

In the machine learning field, it has been found that in highly random environments with many contributors, such as a neural network with its many contributing nodes, stochastic rounding vastly improves performance. This environment mirrors that of a Monte Carlo particle simulation, featuring its many contributing accumulators and high randomness, lending to stochastic rounding's application in VMC.

[Github Issue #41](https://github.com/fangq/mcx/issues/41)

In the field of VMC, specifically MCX (Monte Carlo eXtreme), the popular CUDA implementation, the issue of rounding error is documented and noted as an issue in large photon simulations, even with 32 bit precision floating point numbers. The listed fixes are to use double precision, however that's inefficient on GPUs, Kahan summation, storing a second float that tracks rounding error that can be added / adjusted for at the end, and repetitions, splitting the simulation into chunks. Notably, this lacks stochastic rounding, being a gap in production VMC programs due to a lack of hardware support.

# Engineering Goals

Design a VMC program that both uses very low power while performing at a production capable level through applying stochastic rounding in both the computational and caching systems, allowing low energy environments to run on-board VMC calculations.

# Expected Outcomes

A working implementation of a floating point, fixed point, and fixed point with stochastic rounding implementations written in HLS, with a significant percentage increase in performance of the fixed point over floating point, and a further percentage increase in performance of fixed point with stochastic rounding over regular fixed point.

# Materials

Hardware includes a Kria KV260 FPGA Development Board, desktop computer with 32 GB of DDR4 RAM, an AMD Ryzen 5 2600 6 core processor CPU, and access to the AMD RX 6700 XT and NVIDIA RTX 3060 TI graphics cards. Software requirements include the Rust and Python languages, and a free AMD Vitis HLS license.

# Procedures / Methods

1. Implement a software reference in Rust of the VMC algorithm, then compare against the pmcxcl Python MCX algorithm to check for any inaccuracies in tooling.
2. Write custom fixed point and floating point types that features different rounding methods.
3. Reimplement each core operation and the output volume with the new arbitrary precision types.
4. Record total function accuracy over each individual bit width sweep, repeating with stochastic rounding included.
5. Graph final function accuracy with all bit widths optimized for both versions.
6. Implement a floating point reference in HLS of VMC.
7. Implement an optimized arbitrary point version in HLS of VMC that allows for different rounding methods.
8. Measure performance of all 3 sets of programs, the floating point, fixed point, and fixed point with stochastic rounding.
9. Export all 3 datasets back as raw files on my desktop computer for analysis.
10. Repeat steps #7 and #8 10 times, storing each 10 raw outputs and runtime information.

# Risk and Safety

FPGA board requires a power supply of 12V with a maximum of 3 A, presenting no direct harm. The board is capable of running hot, so care will be taken to avoid contact during compute. The board features a static bag to prevent static build up and damage, however additional care will be taken not to cause static damage. Additionally, no hazardous chemicals are used, and no human surveys of any kind will be conducted.

# Data Analysis

Bit sweep and HLS trial data will be exported into raw binary files which will be analyzed in Python through NumPy and matplotlib. For the bit sweep, the RMSE of the total function as each variable is swept will be recorded, graphed to show the correlation between bit width and individual operation RMSE. For arbitrary floating point numbers, both their mantissa and exponent bits will be swept as separate variables. Once all variables have been brought to the lowest width that maintains their individual RMSE threshold, the combined function will be ran and its total RMSE will be recorded. The HLS data will be analyzed on its power draw, photons calculated, time taken, and total RMSE of the output energy grid. Each set of values will be analyzed over their 10 runs, comparing their mean and median to find representative values, where the final results of each method can be compared.

# Bibliography