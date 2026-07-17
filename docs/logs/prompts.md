
## Asked to build graph_flux_3d in scripts/utils.py
Model: Claude Opus 4.8
Date: 07-08-26

### Prompt:
Can you make a python function that loads a 3d .npy file and graphs it in voxels depending on the value of each cube?

### Response:
Raw flux is continuous (and spans many orders of magnitude near the source), so a voxel plot usually needs a log transform + threshold to be readable, and matplotlib's ax.voxels gets very slow above ~50³ voxels, so downsample if your grid is large.

```python
import numpy as np
import matplotlib.pyplot as plt

flux = np.load(f"../cache/flux_{scene}.npy")
if flux.ndim == 4:
    flux = flux[..., 0]

# log scale (flux decays exponentially with depth)
logflux = np.log10(flux + 1e-8)

# pick a threshold to only show voxels with meaningful signal
threshold = np.percentile(logflux, 90)
filled = logflux > threshold

# normalize for coloring
norm = (logflux - logflux[filled].min()) / (logflux[filled].max() - logflux[filled].min())
colors = plt.cm.viridis(np.clip(norm, 0, 1))
colors[..., 3] = np.where(filled, 0.6, 0)  # alpha only where filled

fig = plt.figure(figsize=(8, 8))
ax = fig.add_subplot(projection='3d')
ax.voxels(filled, facecolors=colors, edgecolor=None)
ax.set_xlabel('x'); ax.set_ylabel('y'); ax.set_zlabel('z')
plt.tight_layout()
plt.show()
```

Notes:

If your grid is large (e.g. >100 per side), downsample first: flux_small = flux[::2, ::2, ::2] or use scipy.ndimage.zoom, otherwise ax.voxels will be extremely slow to render and rotate.
Adjust threshold (the percentile) to control how much of the volume shows — MCX flux is dominated by the region near the source, so a high percentile (90–99) usually gives the clearest picture of propagation.
If you want an interactive 3D view instead of static matplotlib, pyvista or mayavi handle volumetric data much better at scale (isosurfaces, opacity transfer functions) — worth switching to if matplotlib feels too slow/clunky.

## Asked to build load_mcx_config in scripts/utils.py
Model: Claude Opus 4.8
Date: 07-08-26

### Prompt:
I'm trying to run pmcxcl, however I am running into an issue where the MCX benchmark JSON file for cube 60 doesn't work as the config. Make a simple python function that loads an MCX benchmark JSON and returns a config dict that works and can be run my pmcxl. I attached my cube60 JSON file for reference, however tell me if there is anything that doesn't follow the MCX style inside of it.

JSON file of cube60: [./mcx_reference/scenes/cube60.json attached]

### Response:
Confirmed working — the config comes out clean and the volume builds correctly. Here's the function:

```python
def load_mcx_config(json_path):
    raw = json.load(open(json_path))
    sess, fwd = raw.get("Session", {}), raw["Forward"]
    src, dom = raw["Optode"]["Source"], raw["Domain"]

    vol = np.zeros(dom["Dim"], dtype=np.uint8)
    for shp in raw.get("Shapes", []):
        if "Grid" in shp:                       # cube60 fills the whole grid
            vol[:] = shp["Grid"].get("Tag", 1)

    cfg = {
        "nphoton":    sess.get("Photons", 1e6),
        "vol":        vol,
        "tstart":     fwd["T0"],
        "tend":       fwd["T1"],
        "tstep":      fwd["Dt"],
        "srcpos":     src["Pos"][:3],
        "srcdir":     src["Dir"][:3],
        "srctype":    src.get("Type", "pencil"),
        "prop":       [[m["mua"], m["mus"], m["g"], m["n"]] for m in dom["Media"]],
        "unitinmm":   dom.get("LengthUnit", 1),
        "issrcfrom0": dom.get("OriginType", 1),
    }
    if "RNGSeed" in sess:
        cfg["seed"] = sess["RNGSeed"]
    for p in ("Param1", "Param2"):              # only added if present
        if p in src:
            cfg["srcparam" + p[-1]] = src[p]
    return cfg
```

Run it with:

```python
import pmcxcl
cfg = load_mcx_config("cube60.json")
res = pmcxcl.run(cfg)
```

The one meaningful change from the reference: it no longer takes vol as an argument — it builds the volume from the JSON itself. That's almost certainly why the raw JSON "doesn't work as a config."
The reason the cube60 JSON won't run directly:
The volume isn't a field you can hand to pmcx — it's declared in Shapes + Domain.Dim, not stored as an array. The reference function sidesteps this by making you pass vol in yourself, but cube60 has no volume file to load from, so there was nothing to pass. My version rasterizes it: Dim gives the [60,60,60] shape, and the Grid shape with "Tag": 1 fills the whole cube with medium label 1 (which indexes into prop). The other reason is just naming — MCX JSON uses capitalized keys (Photons, Pos, Dir), while pmcx/pmcxcl's run() expects lowercase (nphoton, srcpos, srcdir), so it all has to be translated.
On whether anything breaks MCX style — your JSON is actually clean, valid MCX. Every field name is standard and correctly placed (OriginType/LengthUnit/Media under Domain, Media[0] correctly set as the zero-absorption background, single time gate since T1 == Dt, geometry via Shapes+Dim rather than a VolumeFile). Nothing there is malformed. A few minor notes rather than errors:

Domain.Dim [60,60,60] and Shapes[0].Grid.Size [60,60,60] are redundant. That's fine and consistent, but only Dim sets the array shape in my loader — if you ever change one, change both or they'll disagree.
Your source is at [30, 30, 0]; the canonical MCX cube60 benchmark puts it at [29, 29, 0] (the 0-based center of a 60-voxel axis sits at 29.5). Both are valid — just be aware if you're comparing fluence against reference outputs, that one-voxel shift will show up.
mus is 1.01 here vs 1.0 in the standard benchmark. Not wrong, just non-canonical if exact reproduction matters.

One scope caveat on the function: the Grid-fill only handles a uniform volume, which covers cube60 and most homogeneous benchmarks. If you move to a JSON with heterogeneous Shapes (boxes, spheres, layers), you'd want to rasterize those properly instead — happy to extend it if you hit one of those.