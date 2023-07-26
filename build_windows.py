import os, re, json, subprocess

with open("build_settings.json") as f:
    settings = json.load(f)

os.makedirs(settings["build_dir"], exist_ok=True)

with open(settings["manifest_path"]) as f:
    cargo = f.read()

# Read some settings from the toml
lib_name = re.findall("name\s*=\s*\"(.+)\"\n", cargo)[0]
lib_version = re.findall("version\s*=\s*\"(.+)\"\n", cargo)[0]

for rec in settings["archs"]:
    for name, target in rec["targets"].items():
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

        folder_name = f"{lib_name}_{rec['triple']}_{name}"        
        

        
        #subprocess.check_call(
        #    f"{settings['build_command']} --target {rec['triple']} --manifest-path={rec['manifest_path']}",
        #    env=env,
        #    shell=True,
        #)
