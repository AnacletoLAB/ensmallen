import os, re, json, subprocess

with open("build_settings.json") as f:
    settings = json.read(f)

os.makedirs(settings["build_dir"], exist_ok=True)

for rec in settings["archs"]:
    for target in rec["targets"]:
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

        subprocess.check_call(
            f"{settings['build_command']} --target {rec['triple']} --release ",
            env=env,
            shell=True,
        )
