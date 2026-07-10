# TODO

## FPGA

"A nested MLMC framework for efficient simulations on FPGAs,"

Lo et al., "Hardware acceleration of a Monte Carlo simulation for photodynamic therapy treatment planning," J. Biomed. Opt. 2009

William Lo, MASc thesis

"FPGA based monte carlo computation of light absoprtion for photodynamic cancer therapy"

"A HLS case study on Light Propogation simulation in turbid media" IEEE

"Accelerating 3d Monte Carlo Photonic Simulations on tighly coupled CPU FPGA systems"

FullMonte CUDA

## SR

"Stochastic rounding and its probabilistic backward error analysis" SIAM J sci comput

"You Already Have It" 2022

"On stochastic rounding with few random bits" 2025

"stochastic rounding: implementation, error analysis, and applications"

"Deep learning with limited numerical precision"

## Bit Sweep

"Simulating low precision floating point arithmetic"

"CPFloat", "StochascticRounding.jl"


# Notes

## Stochastic Rounding

[Stochastic rounding: implementation, error analysis and applications](https://pmc.ncbi.nlm.nih.gov/articles/PMC8905452/)

Foundational paper on stochastic rounding. Describes the algorithm of rounding a number randomly based on its distance from the two nearest rounding candidates linearly, so a number x has a frac(x) to be rounded up and a 1 - frac(x) chance to be rounded down, where frac() returns the value of x minus its lower bound.

## Monte Carlo

### Parallel Voxelized Implementation

[Monte Carlo simulation of photon migration in 3D turbid media accelerated by graphics processing units](https://pdfs.semanticscholar.org/6810/718bfd22cb6f355a5c6d05c9e9c7f6d9d832.pdf)

Foundational paper on voxelized Monte Carlo used in the MCX GPU algorithm. Describes the idea of dispatching many threads with their own RNG seeds, along with the implementation details.

### Algorithm

[Monte Carlo modeling of light transport in multi-layered tissues](https://coilab.caltech.edu/documents/26537/Wang-1995-Computer_Methods_and_Programs_in_Bio.pdf)

Describes the foundational math behind MCML, the predecessor to MCX, detailing each equation and its derivation. The following two sections are two important sections from the paper.

#### Interaction coefficient

The interaction coefficient $ μ $ is the variable that describes the chance of a photon interacting with either an absorption, $ μ_a $, or a scatter, $ μ_s $. Along with that there is g, anisotropy, the cosine of the average angle that can be used to calculate a random scatter amount. Both $ μ_a $ and $ μ_s $ are just the chance of scattering over some dx infinitely small length. Additionally, $ μ_t = μ_a + μ_s $ is the probability of just any interaction happening, used to calculate path length.

In use, $ μ * L $ where $ L $ is a distance represents the amount of times a photon, on average, has some interaction. When $ μ * dx $, where dx is an infinitely small 1d distance, $ μ * dx $ represents the chance of interacting in the next infinitely small step.

#### Random Function Sampling

MCML defines the function to convert a probability function into a usable sampling function as $ \int_{a}^{x} p(x)dx \, = \varepsilon $, where you plug in p(x) as a function that takes in a value and returns the 'probability' (density) of that value. When plugged into the integral and solved for, you get back a function $ F(\varepsilon) $ that when a random value $ \varepsilon \in (0, 1) $ is input, it returns a correspondingly sampled output.

#### Refractive boundary crossing

Due to the difficult math and the ray "duplication", I decided not to implement refraction on the crossing of voxel boundaries.

### Other

["Scalable and massively parallel Monte Carlo photon transport simulations for heterogeneous computing platforms"](https://pmc.ncbi.nlm.nih.gov/articles/PMC5785911/)

Discusses rewriting MCX in OpenCL and the results of doing so, demonstrating a cross-platform program. Important due to me having an AMD daily driver graphics card, meaning I will need to use MCX-CL if I want to count my own GPU as a datapoint.

["Hybrid mesh and voxel based monte carlo algorithm"](https://pmc.ncbi.nlm.nih.gov/articles/PMC7687934/pdf/boe-11-11-6262.pdf)

Proposes an algorithm called SVMC, about the same as MCX except that each voxel also features an additional plane. That plane is described by a 3 byte center coordinate and a 3 byte normal coordinate, with 2 more bytes for the two tissue indices of each side of the plane. Demonstrates about 2-6x performance increase over regular mesh based MC, however is about 2x slower than regular MCX, making up for it by being much more accurate.

["Tutorial on Monte Carlo simulation of photon transport in biological tissues"](https://opg.optica.org/boe/fulltext.cfm?uri=boe-14-2-559)

Describes the core steps that underlie any photon transport MC implementation regardless of whether its MCML or MCX, explaining each and their common pitfalls.

["Exhaustive review of acceleration strategies for Monte Carlo simulations in photon transit"](https://www.researching.cn/ArticlePdf/m00092/2024/17/5/2430004.pdf)

Surveys a huge amount of papers and projects on MC photon transport, detailing their hardware specifications and different techniques.