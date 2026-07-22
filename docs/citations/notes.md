# TODO

## FPGA

"FPGA based monte carlo computation of light absoprtion for photodynamic cancer therapy"

"A HLS case study on Light Propogation simulation in turbid media" IEEE

"Accelerating 3d Monte Carlo Photonic Simulations on tighly coupled CPU FPGA systems"

## SR

"Stochastic rounding and its probabilistic backward error analysis" SIAM J sci comput

"You Already Have It" 2022

"On stochastic rounding with few random bits" 2025

"stochastic rounding: implementation, error analysis, and applications"

## Bit Sweep

""

# Notes

## Stochastic Rounding

[Stochastic rounding: implementation, error analysis and applications](https://pmc.ncbi.nlm.nih.gov/articles/PMC8905452/)

Foundational paper on stochastic rounding. Describes the algorithm of rounding a number randomly based on its distance from the two nearest rounding candidates linearly, so a number x has a frac(x) to be rounded up and a 1 - frac(x) chance to be rounded down, where frac() returns the value of x minus its lower bound.

[Deep Learning with Limited Numerical Precision](https://www.researchgate.net/profile/Ankur-Agrawal-18/publication/272195143_Deep_Learning_with_Limited_Numerical_Precision/links/551952760cf273292e7148bc/Deep-Learning-with-Limited-Numerical-Precision.pdf)

## Monte Carlo

[Monte Carlo simulation of photon migration in 3D turbid media accelerated by graphics processing units](https://pdfs.semanticscholar.org/6810/718bfd22cb6f355a5c6d05c9e9c7f6d9d832.pdf)

Foundational paper on voxelized Monte Carlo used in the MCX GPU algorithm. Describes the idea of dispatching many threads with their own RNG seeds, along with the implementation details.

[Monte Carlo modeling of light transport in multi-layered tissues](https://coilab.caltech.edu/documents/26537/Wang-1995-Computer_Methods_and_Programs_in_Bio.pdf)

Describes the foundational math behind MCML, the predecessor to MCX, detailing each equation and its derivation. The following two sections are two important sections from the paper.

The interaction coefficient $ μ $ is the variable that describes the chance of a photon interacting with either an absorption, $ μ_a $, or a scatter, $ μ_s $. Along with that there is g, anisotropy, the cosine of the average angle that can be used to calculate a random scatter amount. Both $ μ_a $ and $ μ_s $ are just the chance of scattering over some dx infinitely small length. Additionally, $ μ_t = μ_a + μ_s $ is the probability of just any interaction happening, used to calculate path length.

In use, $ μ * L $ where $ L $ is a distance represents the amount of times a photon, on average, has some interaction. When $ μ * dx $, where dx is an infinitely small 1d distance, $ μ * dx $ represents the chance of interacting in the next infinitely small step.

MCML defines the function to convert a probability function into a usable sampling function as $ \int_{a}^{x} p(x)dx \, = \varepsilon $, where you plug in p(x) as a function that takes in a value and returns the 'probability' (density) of that value. When plugged into the integral and solved for, you get back a function $ F(\varepsilon) $ that when a random value $ \varepsilon \in (0, 1) $ is input, it returns a correspondingly sampled output.

Due to the difficult math and the ray "duplication", I decided not to implement refraction on the crossing of voxel boundaries.

["Scalable and massively parallel Monte Carlo photon transport simulations for heterogeneous computing platforms"](https://pmc.ncbi.nlm.nih.gov/articles/PMC5785911/)

Discusses rewriting MCX in OpenCL and the results of doing so, demonstrating a cross-platform program. Important due to me having an AMD daily driver graphics card, meaning I will need to use MCX-CL if I want to count my own GPU as a datapoint.

["Hybrid mesh and voxel based monte carlo algorithm"](https://pmc.ncbi.nlm.nih.gov/articles/PMC7687934/pdf/boe-11-11-6262.pdf)

Proposes an algorithm called SVMC, about the same as MCX except that each voxel also features an additional plane. That plane is described by a 3 byte center coordinate and a 3 byte normal coordinate, with 2 more bytes for the two tissue indices of each side of the plane. Demonstrates about 2-6x performance increase over regular mesh based MC, however is about 2x slower than regular MCX, making up for it by being much more accurate.

["Tutorial on Monte Carlo simulation of photon transport in biological tissues"](https://opg.optica.org/boe/fulltext.cfm?uri=boe-14-2-559)

Describes the core steps that underlie any photon transport MC implementation regardless of whether its MCML or MCX, explaining each and their common pitfalls.

["Exhaustive review of acceleration strategies for Monte Carlo simulations in photon transit"](https://www.researching.cn/ArticlePdf/m00092/2024/17/5/2430004.pdf)

Surveys a huge amount of papers and projects on MC photon transport, detailing their hardware specifications and different techniques.

[Github MCX, Issue #41](https://github.com/fangq/mcx/issues/41)

Outlines the issue of MCX slowly losing accuracy as small values are deposited to large totals, and lists potential solutions including Kahan summation and repeating the test iterations multiple times, importantly not listing stochastic rounding.

[FullMonte: a framework for high-performance Monte Carlo
simulation of light through turbid media with complex
geometry](https://www.researchgate.net/profile/Lothar-Lilge/publication/258812265_FullMonte_a_framework_for_high-performance_Monte_Carlo_simulation_of_light_through_turbid_media_with_complex_geometry/links/55999cc008ae5d8f39363675/FullMonte-a-framework-for-high-performance-Monte-Carlo-simulation-of-light-through-turbid-media-with-complex-geometry.pdf)

Method of computing VMC, however uses tetrahedral meshes to represent curved figures. Used in Rationale section of research plan, section 3.1 (page 3/14) says that they use 64 bit buffer for fluence accumulation

## FPGA

### Lo & Luu MCML

[Hardware acceleration of a Monte Carlo simulation for photodynamic treatment planning](https://www.spiedigitallibrary.org/journalArticle/Download?fullDOI=10.1117/1.3080134)
[FPGA-based Monte Carlo Computation of Light Absorption for Photodynamic Cancer Therapy](https://www.eecg.utoronto.ca/~pc/research/publications/luu.fccm2009.pdf)

### FullMonte

[A High-Level Synthesis Case Study on Light Propagation Simulation in Turbid Media](https://ieeexplore.ieee.org/abstract/document/8457665)
[HLS-based FPGA Acceleration of Light Propagation Simulation in Turbid Media](https://dl.acm.org/doi/pdf/10.1145/3241793.3241804)

2018 floating point version of MCX, done in HLS using float values.

### Misc

[Method to measure power draw on Kria KV260](https://adaptivesupport.amd.com/s/question/0D54U00008Jba6TSAR/best-method-to-measure-powerenergy-consumption?language=en_US)

Forum post about how measuring power draw on the KV260. Response mentions using "INA260 ADDR: 0x40". Potentially a different board (KV260 could be a common model, with Kria being the differentiater)

## Bit Sweep

[DSP-Packing: Squeezing Low-precision Arithmetic into FPGA DSP Blocks](https://arxiv.org/abs/2203.11028)

https://docs.amd.com/v/u/en-US/ug579-ultrascale-dsp

Technical document of Xilinx DSP, specifically page 9/77 is used where it mentions supporting a "27 × 18 two’s complement multiplier" and "48-bit accumulator".

[StochasticRounding.jl](https://github.com/milankl/StochasticRounding.jl)
[CPFloat](https://github.com/north-numerical-computing/cpfloat)

Two potential repos that were reviewed for their use in the bit sweep before deciding to go with a custom implementation.

[Simulating low precision floating point arithmetic](https://epubs.siam.org/doi/pdf/10.1137/19M1251308)

Describes simulating low precision floating point numbers. Source that introduces the idea of rounding after every operation, which is what is done in the final bit sweep.

## Misc

[A Semianalytic Monte Carlo Simulator for Spaceborne Oceanic Lidar: Framework and Preliminary Results](https://www.mdpi.com/2072-4292/12/17/2820)

Technique for simulating LIDAR from a satellite into the ocean. Consists of 3 modules, the first simulations MC with the Mie atmospheric equation, the second uses Snell's law once the ocean interface is reached, and the final uses the Petzoid equation once in the water. At each deposit step, the program solves an analytical equation to find the amount of light that would travel back to the satellite sensor.