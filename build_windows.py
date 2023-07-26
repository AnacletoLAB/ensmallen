import os, re, json, subprocess

ROOT = os.path.dirname(os.path.abspath(__file__))

with open("build_settings.json") as f:
    settings = json.load(f)

# Make sure that the building dir exsits
os.makedirs(settings["build_dir"], exist_ok=True)
# Clear out the features list
features_file = os.path.join(settings["build_dir"], "features.csv")
with open(features_file, "w"):
    pass

with open(settings["manifest_path"]) as f:
    cargo = f.read()

# Read some settings from the toml
lib_name = re.findall("name\s*=\s*\"(.+)\"\n", cargo)[0]
lib_version = re.findall("version\s*=\s*\"(.+)\"\n", cargo)[0]

subprocess.check_call(
    f"cargo clean --manifest-path={settings['manifest_path']}",
    shell=True,
    cwd=ROOT,
)

for rec in settings["archs"]:
    for name, target in rec["targets"].items():
        # Compute the env vars
        env = {
            **os.environ,
            **rec.get("env", {}),
        }
        env["RUSTFLAGS"] = " ".join([
            os.environ.get("RUSTLFLAGS", ""), 
            settings.get("RUSTFLAGS", ""), 
            rec.get("RUSTFLAGS", ""),
            target.get("RUSTFLAGS", ""),
        ])
        # Compute the build folder for this target
        folder = os.path.join(
            settings["build_dir"],
            f"{lib_name}_{rec['triple']}_{name}",
        )
        # Copy the folder to the build folder ignoring the . files and dirs like .git
        subprocess.check_call(
            f"rsync -a --exclude='.*' . {folder}",
            shell=True,
            cwd=ROOT,
        )
        # remove the zig libc version at the end of the triple
        simple_triple = re.findall("^(.+?)(?:\.\d+)*$", rec['triple'])[0]
        # Extract the features for this cpu target
        output = subprocess.check_output(
            f" rustc --print cfg -C target-cpu={target['cpu']} --target={simple_triple}",
            env=env,
            shell=True,
        ).decode()
        features = re.findall("target_feature=\"(.+)\"\n", output)
        features = list(sorted(set(features)))
        print(f"target: {rec['triple']} arch: {target['cpu']} has features {features}")
        # Track the features required for this target
        with open(features_file, "a") as f:
            f.write("{},{}\n".format(
                name, ",".join(features),    
            ))

        #subprocess.check_call(
        #    f"{settings['build_command']} --target {rec['triple']} --manifest-path={settings['manifest_path']}",
        #    env=env,
        #    shell=True,
        #    cwd=folder,
        #)
