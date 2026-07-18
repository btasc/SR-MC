# Rationale

In Monte Carlo simulations, specifically voxelized Monte Carlo photon transport, referred to as VMC from now on, a simulation generally consists of many tiny adds to the global accumulator grid. As a result, VMC grids require their per grid storage type to have a very large bit width capable of storing many precise values. This puts a large strain on memory use, along with requiring large intermediary datatypes, causing performance difficulties. Stochastic rounding is a technique to fix specifically this issue through trading bias with variance, and can be applied in both the VMC grid accumulator along with the intermediary datatypes, allowing both to be implemented in a lower width datatype. This helps both computational performance and memory strain. In order to facilitate both high performance stochastic rounding and arbitrary width numbers, an FPGA is the hardware of choice. It features extreme customizability, allowing for tailored RNG and fine control over math. 

# Research Question

## Questions

1. How far can stochastic rounding facilitate the lowering of bit widths until variance causes outputs to decay?
2. How does the lowering of bit widths effect the subsequent overall algorithmic performance?
3. Does the change in performance allow for a low power FPGA device to viably compute VMC?

## Synopsis of background

[Deep Learning with Limited Numerical Precision](https://www.researchgate.net/profile/Ankur-Agrawal-18/publication/272195143_Deep_Learning_with_Limited_Numerical_Precision/links/551952760cf273292e7148bc/Deep-Learning-with-Limited-Numerical-Precision.pdf)

In the machine learning field, it has been found that in highly random environments with many contributors, such as a neural network with its many contributing nodes, stochastic rounding vastly improves performance. This environment mirrors that of a Monte Carlo particle simulation, featuring its many contributing accumulators and high randomness, lending to stochastic rounding's application in VMC.

[Github Issue #41](https://github.com/fangq/mcx/issues/41)

In the field of VMC, specifically MCX (Monte Carlo eXtreme), the popular CUDA implementation, the issue of rounding error is documented and noted as an issue in large photon simulations, even with 32 bit precision floating point numbers. The listed fixes are to use double precision, however that's inefficient on GPUs, Kahan summation, storing a second float that tracks rounding error that can be added / adjusted for at the end, and repetitions, splitting the simulation into chunks. Notably, this lacks stochastic rounding, being a gap in production VMC programs.

# Hypothesis

If stochastic rounding is applied to the highly random system of VMC, then bit widths are able to be lowered, causing VMC to be viable on a low power FPGA device. This is due to bit width allowing for more efficient use of both bandwidth and compute power.

# Engineering Goals

# Expected Outcomes

# Materials

# Procedures / Methods

# Risk and Safety

# Data Analysis

# Bibliography