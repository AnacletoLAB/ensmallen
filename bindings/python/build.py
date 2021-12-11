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
# Setup logger
################################################################################
import logging
logging.basicConfig(level=logging.INFO)

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

WHEELS_FOLDER = os.environ.get("WHEELS_FOLDER", join("wheels")) 
logging.info("WHEELS_FOLDER: %s", WHEELS_FOLDER)
MERGIN_FOLDER = os.environ.get("MERGIN_FOLDER", join(WHEELS_FOLDER, "wheels_merged")) 
logging.info("MERGIN_FOLDER: %s", MERGIN_FOLDER)
AVX_FOLDER = os.environ.get("AVX_FOLDER", join(WHEELS_FOLDER, "avx")) 
logging.info("AVX_FOLDER: %s", AVX_FOLDER)
NO_AVX_FOLDER = os.environ.get("NO_AVX_FOLDER", join(WHEELS_FOLDER, "no_avx")) 
logging.info("NO_AVX_FOLDER: %s", NO_AVX_FOLDER)

NO_AVX_BUILD_FOLDER = os.environ.get("NO_AVX_BUILD_FOLDER", join("build_no_avx")) 
logging.info("NO_AVX_BUILD_FOLDER: %s", NO_AVX_BUILD_FOLDER)
AVX_BUILD_FOLDER = os.environ.get("AVX_BUILD_FOLDER", join("build_avx")) 
logging.info("AVX_BUILD_FOLDER: %s", AVX_BUILD_FOLDER)


NO_AVX_FEATURES = os.environ.get("NO_AVX_FEATURES", 
    "+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+sse4a,+avx,+avx2,+bmi1,+bmi2,+lzcnt,+popcnt,+cmov"
)
logging.info("NO_AVX_FEATURES: %s", NO_AVX_FEATURES)

AVX_FEATURES = os.environ.get("AVX_FEATURES", 
    "+sse,+sse2"
)
logging.info("AVX_FEATURES: %s", AVX_FEATURES)

RUSTFLAGS = os.environ.get("RUSTFLAGS", 
    "-C target-feature={cpu_features} -C inline-threshold=1000"
)
logging.info("RUSTFLAGS: %s", RUSTFLAGS)

################################################################################
# Clean the folders
################################################################################
shutil.rmtree(join(WHEELS_FOLDER), ignore_errors=True)
shutil.rmtree(NO_AVX_BUILD_FOLDER, ignore_errors=True)
shutil.rmtree(AVX_BUILD_FOLDER, ignore_errors=True)

os.makedirs(join(WHEELS_FOLDER), exist_ok=True)
os.makedirs(join(MERGIN_FOLDER), exist_ok=True)

logging.info("Creating the build_no_avx folder %s", NO_AVX_BUILD_FOLDER)
# Copy the sources to the build_no_avx folder so that we can modify it without worries
shutil.copytree(join("."), NO_AVX_BUILD_FOLDER)
os.makedirs(NO_AVX_BUILD_FOLDER, exist_ok=True)

logging.info("Creating the build_avx folder %s", AVX_BUILD_FOLDER)
# Copy the sources to the build folder so that we can modify it without worries
# We copy the non_avx folder because if we copy `.` otherwise it will include
# a copy of the avx build
shutil.copytree(NO_AVX_BUILD_FOLDER, AVX_BUILD_FOLDER)

logging.info("Patching the no avx build")
patch(join(NO_AVX_BUILD_FOLDER, "pyproject.toml"),
    r"name\s*=\s*\".+?\"", 
    r"""name="ensmallen_no_avx" """
)
patch(join(NO_AVX_BUILD_FOLDER, "Cargo.toml"),
    r"name\s*=\s*\".+?\"", 
    r"""name = "ensmallen_no_avx" """
)
patch(join(NO_AVX_BUILD_FOLDER, "Cargo.toml"),
    r"""path\s*=\s*\"..""", 
    r"""path = "../..""", 
)
patch(join(NO_AVX_BUILD_FOLDER, "src", "auto_generated_bindings.rs"), 
    r"fn ensmallen\(_py: Python", 
    r"fn ensmallen_no_avx(_py: Python",
)   

# Rename the sources folder
shutil.move(
    join(NO_AVX_BUILD_FOLDER, "ensmallen"), 
    join(NO_AVX_BUILD_FOLDER, "ensmallen_no_avx")
)

logging.info("Patching the avx build")
patch(join(AVX_BUILD_FOLDER, "Cargo.toml"),
    r"""path\s*=\s*\"..""", 
    r"""path = "../..""", 
)

################################################################################
# Build the wheels
################################################################################
for python_minor_version in [6, 7, 8, 9]:
    shutil.rmtree(AVX_FOLDER, ignore_errors=True)
    os.makedirs(AVX_FOLDER, exist_ok=True)

    shutil.rmtree(NO_AVX_FOLDER, ignore_errors=True)
    os.makedirs(NO_AVX_BUILD_FOLDER, exist_ok=True)

    logging.info("Building version: 3.%s", python_minor_version)

    # Dispatch the python interpreter
    if platform.system().strip().lower() == "windows":
        python_interpreter = "{}\AppData\Local\Programs\Python\Python3{}\python.exe".format(
            os.path.expanduser("~"),
            python_minor_version,
        )
    else:
        python_interpreter = "python3.{}".format(python_minor_version)

    logging.info("Using the interpreter: %s", python_interpreter)

    ################################################################################
    # Build the non avx version
    ################################################################################
    logging.info("Compiling the noavx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out {}".format(
            python_interpreter,
            NO_AVX_FOLDER
        ), 
        env={
            **os.environ,
            "RUSTFLAGS":RUSTFLAGS.format(cpu_features=NO_AVX_FEATURES),
        },
        cwd=NO_AVX_BUILD_FOLDER,
    )

    ################################################################################
    # Build the avx version
    ################################################################################
    logging.info("Compiling the avx version")
    exec(
        "maturin build --release --strip -i {} --no-sdist --out {}".format(
            python_interpreter,
            AVX_FOLDER,
        ), 
        env={
            **os.environ,
            "RUSTFLAGS":RUSTFLAGS.format(cpu_features=AVX_FEATURES),
        },
        cwd=AVX_BUILD_FOLDER,
    )

    ################################################################################
    # Copy the file to the other wheel
    ################################################################################
    logging.info("Merging the wheel files")
    os.makedirs(MERGIN_FOLDER, exist_ok=True)

    # Find the no_avx wheel file
    src_wheel = join(NO_AVX_FOLDER, os.listdir(NO_AVX_FOLDER)[0])

    # Find the .so compiled library in it
    if platform.system().strip().lower() == "windows":
        library_extension = ".pyd"
    else:
        library_extension = ".so"

    logging.info("Reading the noavx compiled library from %s", src_wheel)
    with zipfile.ZipFile(src_wheel) as z:
        lib = next(x for x in z.filelist if x.filename.endswith(library_extension))

        # Read the .so
        logging.info("The noavx compiled library is %s", lib.filename)
        with z.open(lib.filename) as f:
            no_avx_library = f.read()

    # Compute the hash of the library
    m = hashlib.sha256()
    m.update(no_avx_library)
    no_avx_library_hash = base64.b64encode(m.digest())
    logging.info("The noavx compiled library hash is %s", no_avx_library_hash)

    # Find the avx wheel file
    dst_wheel = join(AVX_FOLDER, os.listdir(AVX_FOLDER)[0])
    logging.info("The avx wheel is %s", dst_wheel)

    # Compute the target zip file
    target_file = join(MERGIN_FOLDER, os.path.basename(dst_wheel))
    shutil.rmtree(target_file, ignore_errors=True)
    logging.info("The merged wheel will be %s", target_file)

    logging.info("Merging the wheels")
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
                    data = data.decode()
                    data = [x for x in data.split("\n") if x.strip() != ""]
                    data.append("ensmallen/{},sha256={},{}".format(
                        os.path.basename(lib.filename),
                        no_avx_library_hash,
                        len(no_avx_library),
                    ))
                    # Sort the lines
                    data = "\n".join(sorted(data)) + "\n"
                    data = data.encode()

                zipwrite.writestr(item, data)

    logging.info("Done!")
    ################################################################################
    # Copy the file to the other wheel
    ################################################################################

    logging.info("Fixing the wheel to be in the standard manylinux2010 if needed")
    # Repairing the file
    # WARNING: adding --strip here breaks the wheel OFC
    if platform.system().strip().lower() == "linux":
        exec(
            "auditwheel repair {} --wheel-dir {}".format(target_file, WHEELS_FOLDER),
            env=os.environ,
            cwd=MERGIN_FOLDER,
        )
    else:
        shutil.copy(
            target_file, 
            join(WHEELS_FOLDER, os.path.basename(dst_wheel)),
        )