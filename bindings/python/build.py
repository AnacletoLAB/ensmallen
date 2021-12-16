#!/bin/python3
from json.decoder import JSONDecodeError
import os
import re
import json
import base64
import shutil
import logging
import zipfile
import hashlib
import argparse
import platform
import subprocess

################################################################################
# Utils
################################################################################

def join(*args):
    return os.path.join(
        os.path.abspath(os.path.dirname(__file__)),
        *args
    )

def exec(command, env={}, **kwargs):
    res = subprocess.run(command, env={
            **os.environ,
            **env,
        }, shell=True, 
        **kwargs
    )  
    if res.returncode != 0:
        raise ValueError("The command '%s' @ '%s' has returned code %s"%(command, kwargs.get("cwd", "."), res.returncode)) 

def patch(file, src_regex, dst_regex):
    with open(join(file), "r") as f:
        text = f.read()

    if len(text) == 0:
        raise ValueError("The opened file '{}' is empty".format(file))

    text = re.sub(src_regex, dst_regex, text)

    with open(join(file), "w") as f:
        f.write(text)

################################################################################
# Get the settings form the env vars
################################################################################

parser = argparse.ArgumentParser(description=
"""Build Ensmallen with performance vs compatability automatical dispatching.
This utility makes to build folders, one for the avx version, one for the 
non_avx version. The script will compile in both the wheels, and then merge them
making a new wheel file that can dispatch at startup which library to use.
Finally, to be able to guarantee proper compatability, on linux, we follow the
manylinux2010 standard which requires us to patch the created wheel using 
`auditwheel`. This is needed because we have `reqwest` as a dependancy which 
requires libcrypto (openssl) which is not in the manylinux2010 allowd libraries.
To fix this auditwheel will ship in the wheel the needed libraries and patch the 
relocations section of the two versions of ensmallen to import these.

This builder uses a json file for settings the targets. An example of settings 
file is:

```json
{
    "python_versions":["3.6", "3.7", "3.8", "3.9", "3.10"],
    "wheels_folder":"wheels", 
    "shared_rustflags":"-C inline-threshold=1000",
    "targets":{
        "haswell":{
            "build_dir":"build_haswell",
            "rustflags":"-C target-cpu=haswell"
        },
        "core2":{
            "build_dir":"build_core2",
            "rustflags":"-C target-cpu=core2"
        }
    }
}
```
""", formatter_class=argparse.ArgumentDefaultsHelpFormatter)

parser.add_argument("-p", "--python-versions", type=str,
    default=None,
    help="""comma separated string of python version to compile for, 
this defaults to what specified in the settings file""".replace("\n", "")
)

parser.add_argument("-s", "--settings-path", type=str,
    default="builder_settings.json",
    help="The json file f",
)

parser.add_argument("-v", "--verbosity", type=str,
    default="info",
    choices=["debug", "info", "error"],
    help="Verbosity of the logger"
)

args = parser.parse_args()

logging.basicConfig(
    format='%(levelname)s:%(message)s', 
    level={
        "debug":logging.DEBUG,
        "info":logging.INFO,
        "error":logging.ERROR,
    }[args.verbosity],
)

logging.debug("args: %s", vars(args))

settings_path = join(args.settings_path)
# Ensure that the file exists
if not os.path.exists(settings_path):
    raise ValueError("The given settings path '%s' does not exists."%settings_path)

# Read the file
with open(settings_path, "r") as f:
    settings_txt = f.read()

# Strip away comments
settings_txt = "".join([
    x.strip()
    for x in settings_txt.split("\n")
    if not x.strip().startswith("//")
])

try:
    settings = json.loads(settings_txt)
except JSONDecodeError:
    raise ValueError("The given settings are not a json.\n%s"%settings_txt)


if args.python_versions is not None:
    PYTHON_VERSIONS = args.python_versions.split(",")
else:
    PYTHON_VERSIONS = settings.get("python_versions", ["3.6", "3.7", "3.8", "3.9"])

WHEELS_FOLDER = join(settings.get("wheels_folder", "wheels"))
MERGIN_FOLDER =  join(WHEELS_FOLDER, "merged")

# Find the .so compiled library in it
if platform.system().strip().lower() == "windows":
    library_extension = ".pyd"
else:
    library_extension = ".so"
################################################################################
# Clean the folders and prepare them to be compiled
################################################################################
shutil.rmtree(join(WHEELS_FOLDER), ignore_errors=True)
os.makedirs(join(WHEELS_FOLDER), exist_ok=True)
os.makedirs(join(MERGIN_FOLDER), exist_ok=True)

# Clean the building directory from past compilations
for target_name, target_settings in settings["targets"].items():
    build_dir = join(target_settings["build_dir"])
    shutil.rmtree(build_dir, ignore_errors=True)

# Copy the sources to the build folder so that we can modify it without worries
# We copy the non_avx folder because if we copy `.` otherwise it will include
# a copy of the avx build
last_copied_folder = join(".")
# Clone the building folders
for target_name, target_settings in settings["targets"].items():
    build_dir = join(target_settings["build_dir"])
    logging.info("Creating the folder %s", build_dir)
    # Copy the sources to the build folder so that we can modify it without worries
    # We copy the non_avx folder because if we copy `.` otherwise it will include
    # a copy of the avx build
    shutil.copytree(last_copied_folder, build_dir)
    last_copied_folder = build_dir

# Patch the folders 
for i, (target_name, target_settings) in enumerate(settings["targets"].items()):
    build_dir = join(target_settings["build_dir"])

    logging.info("Patching the %s build", target_name)
    patch(join(build_dir, "Cargo.toml"),
        r"""path\s*=\s*\"..""", 
        r"""path = "../..""", 
    )
    patch(join(build_dir, "pyproject.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name="ensmallen_%s" """%target_name
    )
    patch(join(build_dir, "Cargo.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name = "ensmallen_%s" """%target_name
    )
    patch(join(build_dir, "src", "auto_generated_bindings.rs"), 
        r"fn ensmallen\(_py: Python", 
        r"fn ensmallen_%s(_py: Python"%target_name,
    )   

    # Rename the sources folder
    shutil.move(
        join(build_dir, "ensmallen"), 
        join(build_dir, "ensmallen_%s"%target_name)
    )


################################################################################
# Build the wheels
################################################################################

resulting_wheels = []
for python_minor_version in PYTHON_VERSIONS:

    logging.info("Building version: %s", python_minor_version)

    # Dispatch the python interpreter
    if platform.system().strip().lower() == "windows":
        python_interpreter = "{}\AppData\Local\Programs\Python\Python{}\python.exe".format(
            os.path.expanduser("~"),
            python_minor_version.replace(".", ""),
        )
    else:
        python_interpreter = "python{}".format(python_minor_version)

    logging.info("Using the interpreter: %s", python_interpreter)
        
    ################################################################################
    # Compile all the targets
    ################################################################################
    for target_name, target_settings in settings["targets"].items():
        logging.info("%s settings: %s", target_name, target_settings)

        build_dir = join(target_settings["build_dir"])
        logging.info("Build dir '%s'", build_dir)
        target_dir = join(WHEELS_FOLDER, target_name)
        # Clean the folder
        shutil.rmtree(target_dir, ignore_errors=True)
        os.makedirs(target_dir, exist_ok=True)

        rust_flags = settings["shared_rustflags"] + " " + target_settings["rustflags"]

        logging.info("Compiling the '%s' target with flags: '%s'", target_name, rust_flags)
        exec(
            "maturin build --release --strip -i {} --no-sdist --out {}".format(
                python_interpreter,
                target_dir
            ), 
            env={
                **os.environ,
                "RUSTFLAGS":rust_flags,
            },
            cwd=build_dir,
        )

    ################################################################################
    # Copy the file to the other wheel
    ################################################################################
    logging.info("Merging the wheel files")
    os.makedirs(MERGIN_FOLDER, exist_ok=True)

    # Extract the compiled libraries form the wheels
    libs = []

    for i, (target_name, target_settings) in enumerate(settings["targets"].items()):
        target_dir = join(WHEELS_FOLDER, target_name)
        src_wheel = join(target_dir, os.listdir(target_dir)[0])
        wheel_name = os.path.basename(src_wheel)

        logging.debug("Reading the '%s' compiled library from '%s'", target_name, src_wheel)
        with zipfile.ZipFile(src_wheel) as z:
            lib = next(x for x in z.filelist if x.filename.endswith(library_extension))

            # Read the .so
            logging.info("The %s compiled library is '%s'", target_name, lib.filename)
            with z.open(lib.filename) as f:
                compiled_libray = f.read()

            lib_name = os.path.basename(lib.filename)

        # Compute the hash of the library
        m = hashlib.sha256()
        m.update(compiled_libray)
        library_hash = base64.b64encode(m.digest()).decode()
        logging.debug("The '%s' compiled library hash is %s", target_name, library_hash)

        libs.append({
            "target_name":target_name,
            "wheel_name":wheel_name,
            "lib_name":lib_name,
            "lib":compiled_libray,
            "hash":library_hash,
            "size":len(compiled_libray),
        })

    # Take a wheel to copy all the non compiled files
    donor_target, _ = list(settings["targets"].items())[0]
    donor_dir = join(WHEELS_FOLDER, donor_target)
    donor_wheel = join(donor_dir, os.listdir(donor_dir)[0])
    logging.debug("The donor wheel is %s", donor_wheel)

    # Compute the target zip file
    target_file = os.path.basename(donor_wheel).replace(
        "ensmallen_%s"%donor_target,
        "ensmallen",
    )
    merged_wheel = join(MERGIN_FOLDER, target_file)
    shutil.rmtree(merged_wheel, ignore_errors=True)
    logging.debug("The merged wheel will be %s", merged_wheel)

    logging.debug("Merging the wheels")
    with zipfile.ZipFile(donor_wheel, 'r') as zipread:
        with zipfile.ZipFile(
            merged_wheel, 'w', 
            compression=zipfile.ZIP_DEFLATED,
            ) as zipwrite:

            for data in libs:
                library_path = "ensmallen/{}".format(data["lib_name"])
                logging.debug("Copying the compiled libraries to '%s'", library_path)
                # Add the libraries to the new zip
                zipwrite.writestr(
                    library_path, 
                    data["lib"]
                )

            # Copy all the other files from the avx zip
            for item in zipread.infolist():
                data = zipread.read(item.filename)
                dst_path = item.filename.replace(
                    "ensmallen_%s"%donor_target,
                    "ensmallen",
                )
                logging.debug("Copying file bewtten wheels '%s' to '%s'", item.filename, dst_path)

                # Skip the compiledlibrary from the donor wheel
                if dst_path.startswith("ensmallen") and dst_path.endswith(library_extension):
                    logging.debug("Skipping '%s'", dst_path)
                    continue

                # Patch the RECORD file adding the non_avx library
                # The record line has the following format:
                # $PATH,$HASH,$FILE_SIZE_IN_BYTES
                if dst_path.endswith("METADATA"):
                    logging.debug("Patching the METADATA file")
                    data = data.decode()
                    data = data.replace(
                        "ensmallen_%s"%donor_target,
                        "ensmallen",
                    ).encode()
                elif dst_path.endswith("RECORD"):
                    logging.debug("Patching the RECORD file")
                    data = data.decode()
                    data = [
                        x.replace(
                            "ensmallen_%s"%donor_target,
                            "ensmallen",
                        )
                        for x in data.split("\n") 
                        if x.strip() != "" and 
                            not (x.split(",")[0].endswith(library_extension))
                    ]
                    for vals in libs:
                        data.append("ensmallen/{lib_name},sha256={hash},{size}".format(**vals))
                    # Sort the lines
                    data = "\n".join(sorted(data)) + "\n"
                    data = data.encode()

                zipwrite.writestr(dst_path, data)

    logging.debug("Done!")
    ################################################################################
    # Copy the file to the other wheel
    ################################################################################

    # Repairing the file
    final_wheel = join(WHEELS_FOLDER, target_file)
    logging.info("The final wheel will be at '%s'", final_wheel)
    # WARNING: adding --strip here breaks the wheel OFC
    if platform.system().strip().lower() == "linux":
        logging.info("Fixing the wheel to be in the standard manylinux2010 if needed")
        exec(
            "auditwheel repair {} --wheel-dir {}".format(target_file, WHEELS_FOLDER),
            env=os.environ,
            cwd=MERGIN_FOLDER,
        )
    else:
        shutil.copy(
            merged_wheel, 
            final_wheel,
        )

    resulting_wheels.append(final_wheel)

logging.info("To publish just run:\ntwine upload %s", " ".join(resulting_wheels))
