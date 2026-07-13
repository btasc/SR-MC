import pmcxcl, json

scenes_to_create = [
    # (bench_name, do_generate_volume)
    ("cube60", False)
]

volume_dir = "../volumes/"

for scene in scenes_to_create:
    cfg = pmcxcl.mcxcreate(scene[0])

    if scene[1]:
        with open(f"{volume_dir}{scene[0]}.bin", "wb") as f:
            f.write(cfg["vol"].tobytes(order="F"))

    cfg["outputtype"] = "energy"

    domain_dims = list(cfg["vol"].shape)

    del cfg["vol"]
    del cfg["detpos"]

    pmcxcl.mcx2json(cfg, scene[0])

    # write in custom dims because normally its derived from cfg["vol"]
    with open(f"./{scene[0]}.json", "r") as f:
        json_cfg = json.load(f)

    json_cfg["Domain"]["Dims"] = domain_dims

    with open(f"./{scene[0]}.json", "w") as f:
        json.dump(json_cfg, f)