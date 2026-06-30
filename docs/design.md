# Environment

Still in progress

Types yet to be implemented:
- [FLUENCE_TYPE] - Type of the fluence values that will be stored in the cache and per photon
- [WEIGHT_TYPE] - Type of the photon weight used in the absorption calculation

## Hardware

The planned FPGA board for this project is the Kria KV260. This is due to its cheaper cost (about $250 versus much more expensive alternatives) compared to many larger boards that generally invalidate most test data through being able to cache the entire scene or other similar shortcuts. On the GPU side, I plan to use an Nvidia 3060 TI as it's the card I currently have access to, however I might have to find other options to compare to due to the 3060 having suboptimal power efficiency. I also own an RX 6700 XT AMD card, however I worry that it will not be able to run the CUDA MCX program, meaning I will need to use different implementations (say OpenCL), causing some potential issues.

## Test data

I plan to use a standard 60x60x60 scene as used for most MCX benchmarks, such as cube60. From there, I plan to run the test scene with time gates, meaning that there will be different outputs corresponding to different times. This is to allow for memory to be a bit more of a challenge, whereas with a single time output, all memory can be stored in the cache with no DDR interaction besides its write out at the end.

# Kernels
This is my first attempt at an HLS design using a system of kernels as will be described.

Last Updated 06-24-26

I would like to organize the main computing into sets of kernels, each distributing work to hls::tasks. This way, I can have parallel processes that are pipelined while still maintaining the canonical hls pattern of tasks. The following is how that is derived.

In the core MCX loop, a photon samples some path length, traverses each voxel in that length (or remains in the current voxel depending on tissue scatter properties), then scatters. This can be represented in pseudocode as:

```python
plen = sample_path_length()

while plen != 0.0:
    material = read_voxel(current_voxel)
    
    # Set plen to new value after each traversal until its 0
    fluence, plen = traverse(material)
    ddr_fluence[current_voxel] += fluence

scatter()
```

As shown, there are two separate scopes in this process. One inside the `while` loop, and the other out. From this distinction, you can reformat the process into separate kernels. First, scatter can simply be shifted up to before the path sample, giving:

```python
scatter()
plen = sample_path_length()
# ...
```

, allowing these two kernels to be merged into one task. From there, we need to allow for a situation where a photon scatters many times within a voxel. In a situation like this, you would want some fluence sum to be written out once at the end, rather than a separate write each time, which would look like:

```python
scatter()
plen = sample_path_length()

current_voxel = (1, 5, 24)
voxel_sum = 0.0

while plen != 0.0:
    material = read_voxel()
    new_voxel, fluence, plen = traverse(material)
    
    voxel_sum += fluence
    
    if new_voxel != current_voxel:
        write_fluence(current_voxel, voxel_sum)
        voxel_sum = 0.0
        current_voxel = new_voxel
```

Now, when looking at this process, we can identify what should be its own kernel. First, `scatter() ... plen = sample_path_length()` can be its own kernel, referred to as `scatter_sample`. Next, `traverse()` can be its own kernel, referred to as `traverse`. Next, `read_voxel()` becomes `read_voxel`, and finally `write_fluence()` is kernel `write_fluence`. The rest of this code is either too dynamic, such as the while loop that iterates a variable amount of times, or just updates variables that is easier to do outside a loop.

As a note, the reason why `read_voxel` is its own kernel is that in the case of a photon that spends multiple steps in a voxel, it shouldn't repeatedly read at every traverse, allowing for `read_voxel` to be done just when it needs to.

# Caching Structure

In order to avoid long DDR fetches at each read and write request, a cache system has to be implemented. The Kria KV260, the chosen board for this project, has about 2.3 MB of URAM, meaning both the read and write cache has to fit in this size. - TODO

# Streams

Last updated 06-25-26

To effectively utilize streams, a completely "headless", or system with no orchestrator has to be used. A photon is simply a struct being passed around in memory through different tasks, unlike a software program where it would sit in some variable being processed. The same set of 4 kernels is used, `read_voxels`, `write_fluence`, `sample_scatter`, and `traverse`.

## Photon State

Since streams are now being used, the photon has to store all of its persistent variables in its state. The structure of that state can be broken down as such:

```python
class Photon:
    voxel # 3d voxel coordinate, stored in int5 due to the 60^3 grid padded to 64^3
    fluence # Sum of the fluence in the current voxel, stored in [FLUENCE_TYPE]
    
```

## Deriving Connections

First, we take the original kernel setup of the code:

```python
scatter()
plen = sample_path_length()

current_voxel = (1, 5, 24)
voxel_sum = 0.0

while plen != 0.0:
    material = read_voxel()
    new_voxel, fluence, plen = traverse(material)
    
    voxel_sum += fluence
    
    if new_voxel != current_voxel:
        write_fluence(current_voxel, voxel_sum)
        voxel_sum = 0.0
        current_voxel = new_voxel
```

and rewrite it using streams. Using the traverse task as a "seed" or starting point, we can identify the branches. First, after a traversal, you either read memory for the voxel type, flush the current voxel sum to DDR, and traverse again, or scatter and traverse again. Breaking this down, you can connect it as:

```python

def task traverse(p):
    fluence, new_voxel = step()

    if p.voxel == new_voxel:
        # in same voxel
        p.fluence_sum += fluence
        scatter(p)
    else:
        # new voxel
        # note, write_memory doesn't consume the photon, 
        # only its values that need to be written
        write_memory(p.fluence_sum, p.voxel)
        p.voxel = p.new_voxel
        p.fluence_sum = 0
        
        # consumes p
        read_memory(p)
        
def task scatter(p):
    p.plen = sample_len()
    p.dir = scatter(p.dir, p.material_idx)
    traverse(p)
    
def task read_memory(p):
    # ...
    traverse(p)
    
def task write_memory(p):
    # ...
    

```

The two left to implement are write and read memory. The issue with both is the variance in cycles taken caused by a cache miss. If the URAM cache is hit, - TODO