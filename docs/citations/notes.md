# TODO

Lo et al., "Hardware acceleration of a Monte Carlo simulation for photodynamic therapy treatment planning," J. Biomed. Opt. 2009

"Exhaustive review of acceleration strategies for Monte Carlo simulations in photon transit," J. Innov. Opt. Health Sci. 2024

# Bibliography

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

Describes most of the core math behind the computation MCML that is used in MCX, broken into a few specific useful functions.

#### Interaction coefficient

The interaction coefficient $ μ $ is the variable that describes the chance of a photon interacting with either an absorption, $ μ_a $, or a scatter, $ μ_s $. Along with that there is g, anisotropy, the cosine of the average angle that can be used to calculate a random scatter amount. Both $ μ_a $ and $ μ_s $ are just the chance of scattering over some dx infinitely small length. Additionally, $ μ_t = μ_a + μ_s $ is the probability of just any interaction happening, used to calculate path length.

In use, $ μ * L $ where $ L $ is a distance represents the amount of times a photon, on average, has some interaction. When $ μ * dx $, where dx is an infinitely small 1d distance, $ μ * dx $ represents the chance of interacting in the next infinitely small step.

#### Random Function Sampling

MCML defines the function to convert a probability function into a usable sampling function as $\int_{a}^{x} p(x)dx \, = \varepsilon $, where you plug in p(x) as a function that takes in a value and returns the 'probability' (density) of that value. When plugged into the integral and solved for, you get back a function $ F(\varepsilon) $ that when a random value $ \varepsilon \in (0, 1) $ is input, it returns a correspondingly sampled output.

#### Photon step size

To create the photon step size sampling function, you must find the probability function of a photon step, where when you input a length, and it outputs the probability density of that value. To create the probability function, you must define some function, $ F $, that when given an input $ s $, it outputs the probability of it making to that length. When that function is plugged into the random sampling function, it returns $ - \ln(\varepsilon) / \mu_t $.

#### Refractive boundary crossing

Due to the difficult math and the ray "duplication", I decided not to implement refraction on the crossing of voxel boundaries.

### Other Citations