import os
import re
import shlex
import base64
import shutil
import zipfile
import hashlib
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
    subprocess.run(command, env={
            **os.environ,
            **env,
        }, shell=True, 
        **kwargs
    )   

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

WHEEL_FOLDER = os.environ.get("WHEEL_FOLDER", join("./wheels")) 

TARGET_FOLDER = os.environ.get("TARGET_FOLDER", join("./wheels_merged")) 

CPU_FEATURES = os.environ.get("CPU_FEATURES", 
    "+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+sse4a,+avx,+avx2,+bmi1,+bmi2,+lzcnt,+popcnt,+cmov"
)
print("Building with: CPU_FEATURES: {}".format(CPU_FEATURES))

RUSTFLAGS = os.environ.get("RUSTFLAGS", 
    "-C target-feature={cpu_features} -C inline-threshold=1000"
).format(cpu_features=CPU_FEATURES)
print("Building with: RUSTFLAGS: {}".format(RUSTFLAGS))

################################################################################
# Clean the folders
################################################################################
shutil.rmtree(join(TARGET_FOLDER), ignore_errors=True)
shutil.rmtree(join(WHEEL_FOLDER), ignore_errors=True)

for python_minor_version in [6, 7, 8, 9]:
    shutil.rmtree(join("wheels_no_avx"), ignore_errors=True)
    shutil.rmtree(join("wheels_avx"), ignore_errors=True)
    print("#" * 80)
    print("# Building version: 3.{}".format(python_minor_version))
    print("#" * 80)

    # Dispatch the python interpreter
    if platform.system().strip().lower() == "windows":
        python_interpreter = "{}\AppData\Local\Programs\Python\Python3{}\python.exe".format(
            os.path.expanduser("~"),
            python_minor_version,
        )
    else:
        python_interpreter = "python3.{}".format(python_minor_version)

    ################################################################################
    # Build the non avx version
    ################################################################################
    print("Creating the build folder")
    shutil.rmtree(join("build"), ignore_errors=True)
    # Copy the sources to the build folder so that we can modify it without worries
    shutil.copytree(join("."), join("build"))
    os.makedirs(join("wheels_no_avx"), exist_ok=True)

    print("Patching the library")
    patch(join("build", "pyproject.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name="ensmallen_no_avx" """
    )
    patch(join("build", "Cargo.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name = "ensmallen_no_avx" """
    )
    patch(join("build", "Cargo.toml"),
        r"""path\s*=\s*\"..""", 
        r"""path = "../..""", 
    )
    patch(join("build", "src", "auto_generated_bindings.rs"), 
        r"fn ensmallen\(_py: Python", 
        r"fn ensmallen_no_avx(_py: Python",
    )   

    shutil.move(join("build", "ensmallen"), join("build", "ensmallen_no_avx"))

    print("Compiling the noavx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out ../wheels_no_avx".format(python_interpreter), 
        env=os.environ,
        cwd=join("build"),
    )

    ################################################################################
    # Build the avx version
    ################################################################################
    print("Creating the build folder")
    shutil.rmtree(join("build"), ignore_errors=True)
    # Copy the sources to the build folder so that we can modify it without worries
    shutil.copytree(join("."), join("build"))
    os.makedirs(join("wheels_avx"), exist_ok=True)

    patch(join("build", "Cargo.toml"),
        r"""path\s*=\s*\"..""", 
        r"""path = "../..""", 
    )

    print("Compiling the avx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out ../wheels_avx".format(python_interpreter), 
        env={
            **os.environ,
            "RUSTFLAGS":RUSTFLAGS,
        },
        cwd=join("build"),
    )

    ################################################################################
    # Copy the file to the other wheel
    ################################################################################
    os.makedirs(join(TARGET_FOLDER), exist_ok=True)

    # Find the no_avx wheel file
    src_wheel = join("wheels_no_avx", os.listdir(join("wheels_no_avx"))[0])
    print("Opening {}".format(src_wheel))

    # Find the .so compiled library in it
    if platform.system().strip().lower() == "windows":
        library_extension = ".pyd"
    else:
        library_extension = ".so"

    with zipfile.ZipFile(src_wheel) as z:
        lib = next(x for x in z.filelist if x.filename.endswith(library_extension))

        # Read the .so
        print("Extracting {}".format(lib.filename))
        with z.open(lib.filename) as f:
            no_avx_library = f.read()

    # Compute the hash of the library
    m = hashlib.sha256()
    m.update(no_avx_library)
    no_avx_library_hash = base64.b64encode(m.digest())

    # Find the avx wheel file
    dst_wheel = join("wheels_avx", os.listdir(join("wheels_avx"))[0])
    print("Opening {}".format(dst_wheel))

    # Compute the target zip file
    target_file = join(TARGET_FOLDER, os.path.basename(dst_wheel))
    shutil.rmtree(target_file, ignore_errors=True)
    print("Creating {}".format(target_file))

    with zipfile.ZipFile(dst_wheel, 'r') as zipread:
        with zipfile.ZipFile(target_file, 'w') as zipwrite:
            # Add the non_avx library to the new zip
            zipwrite.writestr(
                "ensmallen/{}".format(os.path.basename(lib.filename)), 
                no_avx_library
            )

            # Copy all the other files from the avx zip
            for item in zipread.infolist():
                data = zipread.read(item.filename)

                # Patch the RECORD file adding the non_avx library
                # The record line has the following format:
                # $PATH,$HASH,$FILE_SIZE_IN_BYTES
                if item.filename.endswith("RECORD"):
                    data += ("ensmallen/{},sha256={},{}\n".format(
                        os.path.basename(lib.filename),
                        no_avx_library_hash,
                        len(no_avx_library),
                    )).encode()
                    # Sort the lines
                    data = "\n".join(sorted(data.split("\n")))

                zipwrite.writestr(item, data)

    ################################################################################
    # Copy the file to the other wheel
    ################################################################################

    # Repairing the file
    # WARNING: adding --strip here breaks the wheel OFC
    if platform.system().strip().lower() == "linux":
        exec(
            "auditwheel repair {} --wheel-dir {}".format(target_file, WHEEL_FOLDER),
            env=os.environ,
            cwd=TARGET_FOLDER,
        )
    else:
        shutil.copy(target_file, join(WHEEL_FOLDER, os.path.basename(dst_wheel)))