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

WHEELS_FOLDER = os.environ.get("WHEELS_FOLDER", join("./wheels")) 

MERGIN_FOLDER = os.environ.get("MERGIN_FOLDER", join(WHEELS_FOLDER, "wheels_merged")) 
AVX_FOLDER = os.environ.get("AVX_FOLDER", join(WHEELS_FOLDER, "avx")) 
NO_AVX_FOLDER = os.environ.get("NO_AVX_FOLDER", join(WHEELS_FOLDER, "no_avx")) 

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
shutil.rmtree(join(WHEELS_FOLDER), ignore_errors=True)

os.makedirs(join(WHEELS_FOLDER), exist_ok=True)
os.makedirs(join(MERGIN_FOLDER), exist_ok=True)

print("Creating the build folder")
shutil.rmtree(join("build_no_avx"), ignore_errors=True)
# Copy the sources to the build_no_avx folder so that we can modify it without worries
shutil.copytree(join("."), join("build_no_avx"))
os.makedirs(join("wheels_no_avx"), exist_ok=True)

print("Patching the library")
patch(join("build_no_avx", "pyproject.toml"),
    r"name\s*=\s*\".+?\"", 
    r"""name="ensmallen_no_avx" """
)
patch(join("build_no_avx", "Cargo.toml"),
    r"name\s*=\s*\".+?\"", 
    r"""name = "ensmallen_no_avx" """
)
patch(join("build_no_avx", "Cargo.toml"),
    r"""path\s*=\s*\"..""", 
    r"""path = "../..""", 
)
patch(join("build_no_avx", "src", "auto_generated_bindings.rs"), 
    r"fn ensmallen\(_py: Python", 
    r"fn ensmallen_no_avx(_py: Python",
)   

# Rename the sources folder
shutil.move(
    join("build_no_avx", "ensmallen"), 
    join("build_no_avx", "ensmallen_no_avx")
)

print("Creating the build_avx folder")
shutil.rmtree(join("build_avx"), ignore_errors=True)
# Copy the sources to the build folder so that we can modify it without worries
shutil.copytree(join("."), join("build_avx"))
os.makedirs(join("wheels_avx"), exist_ok=True)

patch(join("build_avx", "Cargo.toml"),
    r"""path\s*=\s*\"..""", 
    r"""path = "../..""", 
)

################################################################################
# Build the wheels
################################################################################
for python_minor_version in [6, 7, 8, 9]:
    shutil.rmtree(AVX_FOLDER, ignore_errors=True)
    shutil.rmtree(NO_AVX_FOLDER, ignore_errors=True)
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
    print("Compiling the noavx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out {}".format(
            python_interpreter,
            NO_AVX_FOLDER
        ), 
        env=os.environ,
        cwd=join("build_no_avx"),
    )

    ################################################################################
    # Build the avx version
    ################################################################################
    print("Compiling the avx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out {}".format(
            python_interpreter,
            AVX_FOLDER,
        ), 
        env={
            **os.environ,
            "RUSTFLAGS":RUSTFLAGS,
        },
        cwd=join("build_avx"),
    )

    ################################################################################
    # Copy the file to the other wheel
    ################################################################################
    os.makedirs(join(MERGIN_FOLDER), exist_ok=True)

    # Find the no_avx wheel file
    src_wheel = join(NO_AVX_FOLDER, os.listdir(NO_AVX_FOLDER)[0])
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
    dst_wheel = join(AVX_FOLDER, os.listdir(AVX_FOLDER)[0])
    print("Opening {}".format(dst_wheel))

    # Compute the target zip file
    target_file = join(MERGIN_FOLDER, os.path.basename(dst_wheel))
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
            "auditwheel repair {} --wheel-dir {}".format(target_file, WHEELS_FOLDER),
            env=os.environ,
            cwd=MERGIN_FOLDER,
        )
    else:
        shutil.copy(target_file, join(WHEELS_FOLDER, os.path.basename(dst_wheel)))