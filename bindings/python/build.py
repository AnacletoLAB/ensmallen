#!/bin/python3
from multiprocessing.sharedctypes import Value
import io
import os
import re
import sys
import copy
import json
import shlex
import base64
import shutil
import logging
import zipfile
import hashlib
import tempfile
import argparse
import platform
import sysconfig
import distutils
import subprocess
import glob
from functools import lru_cache
from json.decoder import JSONDecodeError

################################################################################
# Utils
################################################################################

def join(*args):
    return os.path.abspath(os.path.join(
        os.path.abspath(os.path.dirname(__file__)),
        *args
    ))

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

def hash_file(file_path) -> str:
    sha256 = hashlib.sha256()
    with open(file_path, "rb") as f:
        while True:
            data = f.read(1024)
            if not data:
                break
            sha256.update(data)
    return sha256.hexdigest()

def rsync_folders(src, dst):
    """Portable rsync like utility. This is needed because we are going to copy
    only changed files, this way we don't change the modification date and
    rust won't compile things again."""
    
    for file in sorted(glob.iglob(os.path.join(src, "**/*",), recursive=True)):
        if not os.path.isfile(file):
            continue

        desinenza = os.path.abspath(file)[len(os.path.abspath(src)) + 1:]
        dst_file = os.path.join(dst, desinenza)

        # Skip if the file exists and has not changed
        if os.path.exists(dst_file) and hash_file(file) == hash_file(dst_file):
            continue

        logging.info("The file {} changed, overwriting.".format(desinenza))

        # this can copy folder, but whatever it works also on files
        shutil.copyfile(file, dst_file)

################################################################################
# Compilation feautres funcs
################################################################################

@lru_cache()
def get_rust_targets():
    return [
        y.decode()
        for y in subprocess.check_output(
            shlex.split("rustc --print target-list")
        ).split(b"\n")
    ]

# LLVM and Linux have different names for the same features :)
features_traducer = {
    "x86_64":{
        "cmpxchg16b":"cx16",
        "sse3":"pni", # this is just an implication but F u
        "sse4.1":"sse4_1",
        "sse4.2":"sse4_2",
        "lzcnt":"abm", # Not equivalent but abm has lzcnt
        "ermsb":None, # This is not reported by cpuinfo and it's basically depecrated
    },
}

@lru_cache()
def get_random_triple_for_arch(arch):
    return next(
            x
            for x in get_rust_targets()
            if x.startswith(arch)
        )

@lru_cache()
def get_cpus(arch):
    triple = get_random_triple_for_arch(arch)
    return [
        y.decode().strip()
        for y in subprocess.check_output(
            shlex.split(f"rustc --print target-cpus --target {triple}")
        ).split(b"\n")[2:]
        if y.decode().strip()
    ]

@lru_cache()
def get_cpu_features(arch, cpu):
    triple = get_random_triple_for_arch(arch)
    features = [
        features_traducer.get(arch, {}).get(y.decode()[16:-1], y.decode()[16:-1])
        for y in subprocess.check_output(
            shlex.split(f"rustc --print=cfg -C target-cpu={cpu} --target {triple}")
        ).split(b"\n")
        if y.startswith(b"target_feature=") and not y.startswith(b"target_feature=\"llvm")
    ]
    return [x for x in features if x is not None]

@lru_cache()
def get_available_features_for_arch(arch):
    return list(sorted({
        x
        for cpu in get_cpus(arch)
        for x in get_cpu_features(arch, cpu)
    }))

def get_crate_features():
    """Get the features defined in the Cargo.toml of the current crate"""
    with open("Cargo.toml", "r") as f:
        for line in f:
            if line.startswith("[features]"):
                break
        
        features = []
        for line in f:
            if line.startswith("["):
                break
            if len(line.strip()) == 0:
                continue
            features.append(line.split("=")[0].strip())
        return features

def get_pyo3_abi_version():
    """"""
    with open("Cargo.toml", "r") as f:
        data = f.read()
    
    result = re.findall("features\s*=\s*\[.+?abi3-py(\d+).+?\]", data)
    if len(result) == 1:
        return result[0]
    else:
        return None

################################################################################
# Wheel generation funcs
################################################################################

def gen_record_file(wheel_path, record_file_path) -> str:
    result = ""
    with zipfile.ZipFile(wheel_path, 'r') as zipread:
        for item in zipread.infolist():
            file_content = zipread.read(item.filename)
        
            m = hashlib.sha256()
            m.update(file_content)
            file_hash = base64.b64encode(m.digest()).decode()

            result += f"{item.filename},sha256={file_hash},{len(file_content)}\n"

    result += "{},,\n".format(record_file_path)
    return result
    
def gen_metadata(metadata) -> str:
    result = "Metadata-Version: 2.1\n"

    for key, value in metadata.items():
        if key == "libname":
            result += f"Name: {value}\n"
        elif key == "version":
            result += f"Version: {value}\n"
        elif key == "python_version":
            result += f"Requires-Python: >={value}\n"
        elif key == "license":
            result += f"License: {value}\n"
        elif key == "source_code_url":
            result += f"Project-URL: Source Code, {value}\n"
        elif key == "keywords":
            result += "Keywords: {}\n".format(",".join(x.strip() for x in value))
        elif key == "authors":
            result += "Author: {}\n".format(", ".join(
                "{name} <{email}>".format(**author)
                for author in value
            ))
            result += "Author-email: {}\n".format(", ".join(
                "\"{name}\" <{email}>".format(**author)
                for author in value
            ))
        elif key == "deps":
            result += "".join(
                "Require-Dist: {}\n".format(dep)
                for dep in value
            )
        
    # readme format
    result += "Description-Content-Type: text/markdown; charset=UTF-8; variant=GFM\n"
    # add an empty line to separate the readme from the metadata
    result += "\n"
    # add the readme
    with open(metadata["readme_path"], "r") as f:
        result += f.read()

    return result

def gen_wheel_file(metadata):
    """https://peps.python.org/pep-0491/"""
    result  = "Wheel-Version: 1.0\n"
    result += "Generator: ensmallen_toolchain\n"
    result += "Root-Is-Purelib: false\n"
    result += "Tag: {python_tag}-{abi_tag}-{platform_tag}\n".format(**metadata)
    return result

def gen_wheel_name(metadata):
    """https://peps.python.org/pep-0425/"""
    return "{libname}-{version}-{python_tag}-{abi_tag}-{platform_tag}.whl".format(**metadata)

def get_elf_deps(path):
    """Use ldd to get all the deps libs"""
    ldd_stdout = subprocess.check_output(["ldd", path]).decode()

    deps = []

    for lib_line in ldd_stdout.split("\n"):
        matches = re.findall(r"\s*(.+?)\s*=>\s*(.+?) \(", lib_line)
        if len(matches) == 0:
            continue
        lib_name, lib_path = matches[0]
        deps.append((lib_name, lib_path))

    return deps

def get_file_depth(path: str) -> int:
    depth = 0
    while True:
        head, tail = os.path.split(path)

        if tail == "":
            return depth

        depth += 1
        path = head

def patch_elf(libs_folder, path, file_content, patches):
    deps = get_elf_deps(path)

    logging.info("The lib '%s' has deps %s", path, deps)
    # No deps = no patch
    if len(deps) == 0:
        return patches, file_content

    file_depth = get_file_depth(os.path.relpath(path, settings["merging_folder"]))
    rpath = os.path.join(
        "$ORIGIN",
        *[".."] * file_depth,
        libs_folder,
    )
    logging.info("Setting rpath '%s' for '%s'", rpath, path)

    # TODO!: we should also check that the libs versions match
    if settings["manylinux_version"] is not None:
        whitelisted_libs = MANYLINUX_POLICIES[settings["manylinux_version"]]["lib_whitelist"]
    else:
        whitelisted_libs = []

    with tempfile.TemporaryDirectory() as tmpdirname:
        # Create a temp ELF we can patch
        tmpfilename = os.path.join(tmpdirname, os.path.basename(path))
        with open(tmpfilename, "wb") as f:
            f.write(file_content)

        # Reset the RPATH
        exec(f"patchelf --remove-rpath {tmpfilename}")
        # Set the "wheel path"
        exec(f"patchelf --force-rpath --set-rpath '{rpath}' {tmpfilename}")

        for dep_name, dep_path in deps:
            dep_name = dep_name.strip()
            dep_path = dep_path.strip()
            if len(dep_name) == 0 or len(dep_path) == 0 or dep_name.startswith("libcuda"):
                continue
            
            # Skip whitelisted libs
            if dep_name in whitelisted_libs:
                continue

            # Read the dep
            with open(dep_path, "rb") as f:
                dep_data = f.read()

            # Compute its hash
            sha256 = hashlib.sha256()
            sha256.update(dep_data)
            dep_hash = sha256.hexdigest()

            # Cache it
            if dep_hash in patches:
                new_dep_name = patches[dep_hash]["new_name"]
            else:
                new_dep_name = dep_name + "-" + dep_hash
                patches[dep_hash] = {
                    "new_name":new_dep_name,
                    "data":dep_data,
                }

            # Replace the dep with the local one
            exec(f"patchelf --replace-needed {dep_name} {new_dep_name} {tmpfilename}")

        with open(tmpfilename, "rb") as f:
            file_content = f.read()

    return patches, file_content

def gen_wheel(settings):
    dist_info_path = "{libname}-{version}.dist-info".format(**settings["metadata"])
    libs_folder = "{libname}.libs".format(**settings["metadata"])
    patches = {}

    # Create the zip
    with zipfile.ZipFile(
        settings["target_wheel_path"], 'w', 
        compression=zipfile.ZIP_DEFLATED,
        ) as zipwrite:

        # Copy all the files
        for path in glob.iglob(
            os.path.join(settings["merging_folder"], "**", "*"), 
            recursive=True
        ):
            if not os.path.isfile(path):
                continue

            if "__pycache__" in path:
                continue

            local_path = os.path.join(
                settings["metadata"]["libname"],
                os.path.relpath(path, settings["merging_folder"])
            )

            with open(path, "rb") as f:
                file_content = f.read()

            # If we have to, patch the libs to ensure manylinux2010 standard
            # settings["build_type"] != "develop" and \
            if settings["target_os"] == "linux" and \
                not settings["skip_repair"] and \
                    local_path.endswith(settings["library_file_extension"]):
                patches, file_content = patch_elf(libs_folder, path, file_content, patches)
            
            zipwrite.writestr(local_path, file_content)
        # Add the metadata
        zipwrite.writestr(
            os.path.join(dist_info_path, "WHEEL"),
            gen_wheel_file(settings["metadata"])
        )
        # Add the metadata
        zipwrite.writestr(
            os.path.join(dist_info_path, "METADATA"),
            gen_metadata(settings["metadata"])
        )

    if len(patches) > 0:
        # If any lib was patched add ship its deps with in the wheel
        with zipfile.ZipFile(
            settings["target_wheel_path"], 'a', 
            compression=zipfile.ZIP_DEFLATED,
            ) as zipwrite:

            for lib in patches.values():
                zipwrite.writestr(
                    os.path.join(libs_folder, lib["new_name"]),
                    lib["data"],
                )

    # Compute the hashes for all the files
    record_file_path = os.path.join(dist_info_path, "RECORD")
    record_file = gen_record_file(settings["target_wheel_path"], record_file_path)

    # Add the record file to the zip
    with zipfile.ZipFile(
        settings["target_wheel_path"], 'a', 
        compression=zipfile.ZIP_DEFLATED,
        ) as zipwrite:
        zipwrite.writestr(
            record_file_path,
            record_file
        )

def gen_init_file(settings):
    init = """
import logging
import platform
import cpuinfo
import importlib
import ctypes

target_os = platform.system().lower()

# Linux call it aarch64 while mac calls it arm64 :)
arch = {{
    "arm64":"aarch64", 
}}.get(platform.machine(), platform.machine())

cpu_infos = cpuinfo.get_cpu_info()
cpu_features = set(cpu_infos["flags"])

crate_features = set()

# Detecting if cuda is available
if target_os == "linux":
    try:
        ctypes.CDLL("libcuda.so")
        crate_features.add("cuda")
    except OSError:
        pass

targets = {targets}

def choose_target(target_os, arch, cpu_features, crate_features, targets):
    for target in targets:
        if target_os not in target["target_os"]:
            continue
        if arch != target["arch"]:
            continue
        if len(set(target["crate_features"]) - crate_features) != 0:
            continue

        if len(set(target["cpu_features"]) - cpu_features) != 0:
            # We have to skip the check on apple CPUs because the package
            # cpuinfo can't retrieve the flags yet :)
            if not (arch == "aarch64" and cpu_infos["brand_raw"].startswith("Apple")):
                continue

        return target

    raise ValueError("Could not find a version compatible with the current system")

choosen_target = choose_target(target_os, arch, cpu_features, crate_features, targets)
logging.info("Ensmallen choosed target: {{}}".format(choosen_target))

_lib = __import__(
    "ensmallen.{{}}".format(choosen_target["lib_name"]),
    fromlist=("Graph",),
)

locals().update({{
    key:value
    for key, value in vars(_lib).items()
    if not key.startswith("_")
}})

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = ["edge_list_utils", "Graph", "preprocessing", "models", "datasets"]
    """.format(targets=[
        {
            "arch":target["arch"],
            "target_os":target["target_os"],
            "crate_features":target["crate_features"],
            "cpu_features":target["cpu_features"],
            "lib_name":target["lib_name"],
        }
        for target in settings["targets"]
    ])

    with open(settings["init_path"], "w") as f:
        f.write(init)


# Copied from github.com/PyO3/maturin/src/get_interpeter_metadata.py
if platform.python_implementation() == "PyPy":
    # Workaround for PyPy 3.6 on windows:
    #  - sysconfig.get_config_var("EXT_SUFFIX") differs to importlib until
    #    Python 3.8
    #  - PyPy does not load the plain ".pyd" suffix because it expects that's
    #    for a CPython extension module
    #
    # This workaround can probably be removed once PyPy for Python 3.8 is the
    # main PyPy version.
    import importlib.machinery

    EXT_SUFFIX = importlib.machinery.EXTENSION_SUFFIXES[0]
else:
    EXT_SUFFIX = sysconfig.get_config_var("EXT_SUFFIX")

# copied from github.com/pypa/pip/src/pip/_vendor/distlib/wheel.py
if hasattr(sys, 'pypy_version_info'):  # pragma: no cover
    IMP_PREFIX = 'pp'
elif sys.platform.startswith('java'):  # pragma: no cover
    IMP_PREFIX = 'jy'
elif sys.platform == 'cli':  # pragma: no cover
    IMP_PREFIX = 'ip'
else:
    IMP_PREFIX = 'cp'

PYTHON_METADATA = {
    "major": sys.version_info.major,
    "minor": sys.version_info.minor,
    "abiflags": sysconfig.get_config_var("ABIFLAGS"),
    "interpreter": platform.python_implementation().lower(),
    "interpreter_prefix":IMP_PREFIX,
    "ext_suffix": EXT_SUFFIX,
    "abi_tag": (sysconfig.get_config_var("SOABI") or "-").split("-")[1] or None,
    "platform": sysconfig.get_platform().lower(),
    # This one isn't technically necessary, but still very useful for sanity checks
    "system": platform.system().lower(),
    # We need this one for windows abi3 builds
    "base_prefix": sys.base_prefix,
}

print(PYTHON_METADATA)

# Derived from github.com/pypa/auditwheel/src/auditwheel/policy/manylinux-policy.json
MANYLINUX_POLICIES = {"linux": {"name": "linux", "priority": 0, "symbol_versions": {}, "lib_whitelist": [], "blacklist": {}}, "manylinux_2_5": {"name": "manylinux_2_5", "priority": 100, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8"], "ZLIB": []}, "x86_64": {"CXXABI": ["1.3", "1.3.1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8"], "ZLIB": []}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux1": {"name": "manylinux_2_5", "priority": 100, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8"], "ZLIB": []}, "x86_64": {"CXXABI": ["1.3", "1.3.1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8"], "ZLIB": []}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_12": {"name": "manylinux_2_12", "priority": 90, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux2010": {"name": "manylinux_2_12", "priority": 90, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_17": {"name": "manylinux_2_17", "priority": 80, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0"], "GLIBC": ["2.0", "2.17", "2.18"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "ppc64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux2014": {"name": "manylinux_2_17", "priority": 80, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0"], "GLIBC": ["2.0", "2.17", "2.18"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "ppc64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_24": {"name": "manylinux_2_24", "priority": 70, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.19", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_27": {"name": "manylinux_2_27", "priority": 65, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.19", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_28": {"name": "manylinux_2_28", "priority": 64, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.19", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}, "manylinux_2_31": {"name": "manylinux_2_31", "priority": 61, "symbol_versions": {"i686": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "4.0.0", "4.2.0", "4.3.0", "4.4.0", "4.5.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.0", "2.1", "2.1.1", "2.1.2", "2.1.3", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "x86_64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "FLOAT128", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "4.8.0", "7.0.0"], "GLIBC": ["2.2.5", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "aarch64": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.5.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "ppc64le": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "s390x": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "LDBL_1.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.4", "3.4.2", "3.4.4", "4.0.0", "4.1.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.2", "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.6", "2.3", "2.3.2", "2.3.3", "2.3.4", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.19", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28", "LDBL_3.4", "LDBL_3.4.10", "LDBL_3.4.21", "LDBL_3.4.7"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}, "armv7l": {"CXXABI": ["1.3", "1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.3.10", "1.3.11", "1.3.12", "ARM_1.3.3", "TM_1"], "GCC": ["3.0", "3.3", "3.3.1", "3.3.4", "3.4", "3.4.2", "3.5", "4.0.0", "4.2.0", "4.3.0", "4.7.0", "7.0.0"], "GLIBC": ["2.0", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10", "2.11", "2.12", "2.13", "2.14", "2.15", "2.16", "2.17", "2.18", "2.22", "2.23", "2.24", "2.25", "2.26", "2.27", "2.28", "2.29", "2.30", "2.31"], "GLIBCXX": ["3.4", "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5", "3.4.6", "3.4.7", "3.4.8", "3.4.9", "3.4.10", "3.4.11", "3.4.12", "3.4.13", "3.4.14", "3.4.15", "3.4.16", "3.4.17", "3.4.18", "3.4.19", "3.4.20", "3.4.21", "3.4.22", "3.4.23", "3.4.24", "3.4.25", "3.4.26", "3.4.27", "3.4.28"], "ZLIB": ["1.2.0", "1.2.0.2", "1.2.0.8", "1.2.2", "1.2.2.3", "1.2.2.4", "1.2.3.3", "1.2.3.4", "1.2.3.5", "1.2.5.1", "1.2.5.2", "1.2.7.1", "1.2.9"]}}, "lib_whitelist": ["libgcc_s.so.1", "libstdc++.so.6", "libm.so.6", "libdl.so.2", "librt.so.1", "libc.so.6", "libnsl.so.1", "libutil.so.1", "libpthread.so.0", "libX11.so.6", "libXext.so.6", "libXrender.so.1", "libICE.so.6", "libSM.so.6", "libGL.so.1", "libgobject-2.0.so.0", "libgthread-2.0.so.0", "libglib-2.0.so.0", "libresolv.so.2", "libexpat.so.1", "libz.so.1"], "blacklist": {"libz.so.1": ["_dist_code", "_length_code", "_tr_align", "_tr_flush_block", "_tr_init", "_tr_stored_block", "_tr_tally", "bi_windup", "crc32_vpmsum", "crc_fold_512to32", "crc_fold_copy", "crc_fold_init", "deflate_copyright", "deflate_medium", "fill_window", "flush_pending", "gzflags", "inflate_copyright", "inflate_fast", "inflate_table", "longest_match", "slide_hash_sse", "static_ltree", "uncompress2", "x86_check_features", "x86_cpu_has_pclmul", "x86_cpu_has_sse2", "x86_cpu_has_sse42", "z_errmsg", "zcalloc", "zcfree"]}}}

################################################################################
# Settings Schema
################################################################################

ARCH_ENUM = [
    'aarch64',
    'aarch64_be',
    'arm',
    'armebv7r',
    'armv4t',
    'armv5te',
    'armv6',
    'armv6k',
    'armv7',
    'armv7a',
    'armv7r',
    'armv7s',
    'asmjs',
    'avr',
    'bpfeb',
    'bpfel',
    'hexagon',
    'i386',
    'i586',
    'i686',
    'm68k',
    'mips',
    'mips64',
    'mips64el',
    'mipsel',
    'mipsisa32r6',
    'mipsisa32r6el',
    'mipsisa64r6',
    'mipsisa64r6el',
    'msp430',
    'nvptx64',
    'powerpc',
    'powerpc64',
    'powerpc64le',
    'riscv32gc',
    'riscv32i',
    'riscv32im',
    'riscv32imac',
    'riscv32imc',
    'riscv64gc',
    'riscv64imac',
    's390x',
    'sparc',
    'sparc64',
    'sparcv9',
    'thumbv4t',
    'thumbv6m',
    'thumbv7a',
    'thumbv7em',
    'thumbv7m',
    'thumbv7neon',
    'thumbv8m.base',
    'thumbv8m.main',
    'wasm32',
    'wasm64',
    'x86_64',
]

OS_ENUM = ["linux", "darwin", "windows"]

TARGET_SCHEMA = {
    "type":"object",
    "properties":{
        "target_cpu":{"type":"string"},
        "crate_features":{ 
            "type": "array",
            "uniqueItems": True,
            "items": {
                "type": "string",
                "enum":get_crate_features(),
            },
            "default":None,
        },
        "rustflags":{
            "type":"string",
            "default":None,
        }, 
        "target_os":{ 
            "type": "array",
            "uniqueItems": True,
            "minItems": 1,
            "items": {
                "type": "string", 
                "enum":OS_ENUM,
            },
            "default":OS_ENUM,
        },
    },
    "required":[
        "target_cpu",
    ],
}

SETTINGS_SCHEMA = {
    "type":"object",
    "properties":{
        "manylinux_version":{
            "type":"string",
            "enum":list(MANYLINUX_POLICIES.keys()),
            "default":None,
        },
        "arch":{
            "type":"string",
            "enum":ARCH_ENUM,
            "default":{
                "arm64":"aarch64", # Linux call it aarch64 while mac calls it arm64 :)
            }.get(platform.machine(), platform.machine())
        },
        "target_os":{
            "type":"string",
            "enum":OS_ENUM,
            "default":platform.system().lower(),
        },
        "library_file_extension":{
            "type":"string",
            "enum":[".so", ".pyd"],
            "default":(".pyd"
                if platform.system().strip().lower() == "windows"
                else ".so"
            ),
        },
        "shared_rustflags":{
            "type": "string",
            "default":None,
        },
        "wheels_folder":{
            "type": "path",
            "default":"wheels",
        },
        "wheel_root":{
            "type": "path",
            "default":None,
        },
        "merging_folder":{
            "type": "path",
            "default":None,
        },
        "python_files_path":{
            "type": "path",
            "default":None,
        },
        "init_path":{
            "type": "path",
            "default":None,
        },
        "target_wheel_path":{
            "type": "path",
            "default":None,
        },
        "develop":TARGET_SCHEMA,
        "targets":{
            "type":"object",
            "propertyNames": { 
                "type": "string",
                "enum": ARCH_ENUM
            },
            "minProperties": 1,
            "patternProperties":{
                r".+":{
                    "type":"object",
                    "minProperties": 1,
                    "patternProperties":{
                        r".+":TARGET_SCHEMA
                    }
                }
            }
        },
        "metadata":{
            "type":"object",
            "properties":{
                "libname":{
                    "type":"string",
                    "pattern":r"^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9._-]*[a-zA-Z0-9])$",
                    "sub":[r"[-_.]+", r"-"],
                },
                "version":{
                    "type":"string",
                    "pattern":r"^\d+\.\d+.\d+(\.dev\d+)?$",
                },
                "python_version":{
                    "type":"string",
                    "pattern":r"^\d+\.\d+$",
                    "default":"{major}.{minor}".format(**PYTHON_METADATA)
                },
                "python_tag":{
                    "type":"string",
                    "pattern":r"^(py|cp|ip|pp|jy)\d+$",
                    "default":"{interpreter_prefix}{major}{minor}".format(**PYTHON_METADATA)
                },
                "abi_tag":{"type":"string"},
                "platform_tag":{
                    "type":"string",
                    "sub":[r"[-.]+", r"_"],
                    "default":PYTHON_METADATA["platform"],
                },
                "license":{
                    "type":"string",
                    "default":None,
                },
                "source_code_url":{
                    "type":"string",
                    "pattern":r"^https?://",
                    "default":None,
                },
                "readme_path":{"type":"path"},
                "deps":{ 
                    "type": "array",
                    "uniqueItems": True,
                    "minItems": 1,
                    "items": {
                        "type": "string",
                        "pattern":r"^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9._-]*[a-zA-Z0-9])((==|>=|~=|<=)\d+\.\d+.\d+)?$",
                    },
                    "default":None,
                },
                "keywords":{ 
                    "type": "array",
                    "uniqueItems": True,
                    "minItems": 1,
                    "items": {
                        "type": "string",
                        "pattern":r"^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9._-]*[a-zA-Z0-9])$",
                    },
                    "default":None,
                },
                "authors":{ 
                    "type": "array",
                    "uniqueItems": True,
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "properties":{
                            "name":{"type":"string"},
                            "email":{"type":"string"},
                        },
                        "required":[
                            "name",
                            "email",
                        ],
                    }
                },
            },
            "required":[
                "libname",
                "version",
                "abi_tag",
                "readme_path",
                "authors",
            ],
        },
    },
    "required":[
        "metadata",
        "develop",
        "targets",
    ],
}

def validate_json_schema(schema, obj):
    """Validate a jsonschema like schema, this is handcoded because I don't want deps."""
    return inner_validate_json_schema(obj, schema, obj)

def inner_validate_json_schema(root, schema, obj):
    if "default" in schema and obj is None:
        # this is fine to don't validate because it's the schema responsabiliyy
        if schema["default"] is None:
            return obj

        obj = copy.deepcopy(schema["default"])

    if schema["type"] == "object":
        if not isinstance(obj, dict):
            raise ValueError(f"The given value {obj} is not a dictionary for schema {schema}.")

        if schema.get("required", None) is not None:
            for key, value in obj.items():
                if key in schema["required"]:
                    continue
                if "default" not in schema["properties"][key]:
                    raise ValueError(f"Missing default on non-required property '{key}'")
            
            for required in schema["required"]:
                if required not in obj:
                    raise ValueError(f"Missing required property '{required}' in object {obj}")
                

        if schema.get("minProperties", None) is not None:
            if len(obj) < schema["minProperties"]:
                raise ValueError(f"The object {obj} doest not have at least {schema['minProperties']} properties")

        if schema.get("maxProperties", None) is not None:
            if len(obj) < schema["maxProperties"]:
                raise ValueError(f"The object {obj} doest not have at most {schema['maxProperties']} properties")

        if schema.get("properties", None) is not None:
            if len(set(obj.keys()) - set(schema["properties"].keys())) != 0:
                raise ValueError(f"The object {obj} has some keys that are not in the schema. Specifically: {set(obj.keys()) - set(schema['properties'].keys())}")
        
            obj = {
                name:inner_validate_json_schema(root, schema["properties"][name], value)
                for name, value in (
                    (x, obj.get(x, None))
                    for x in set(obj.keys()) | set(schema["properties"].keys())
                )
            }

        if schema.get("propertyNames", None) is not None:
            for key in obj.keys():
                inner_validate_json_schema(root, schema["propertyNames"], key)

        if schema.get("patternProperties", None) is not None:
            new_obj = {}
            for key, value in obj.items():
                for pattern, pattern_schema in schema["patternProperties"].items():
                    if re.match(pattern, key) is not None:
                        new_obj[key] = inner_validate_json_schema(root, pattern_schema, value)
                if key not in new_obj:
                    raise ValueError(f"The given key {key} does not match any of the patternProperties {list(schema['patternProperties'].keys())}")
            obj = new_obj
        return obj
    elif schema["type"] in ["string", "path"]:
        if not isinstance(obj, str):
            raise ValueError(f"The given value {obj} is not a string for schema {schema}.")

        if schema.get("enum", None) is not None:
            if obj not in schema["enum"]:
                raise ValueError(f"The given value {obj} is not in the allowed enum {schema['enum']}")

        if schema.get("pattern", None) is not None:
            if not re.match(schema["pattern"], obj):
                raise ValueError(f"The given string '{obj}' does not match its pattern '{schema['pattern']}'")

        if schema.get("sub", None) is not None:
            obj = re.sub(schema["sub"][0], schema["sub"][1], obj)

        if schema["type"] == "path":
            obj = join(obj)

        return obj
    elif schema["type"] == "array":
        if not isinstance(obj, list):
            raise ValueError(f"The given value {obj} is not an array for schema {schema}.")

        if schema.get("minItems", None) is not None:
            if len(obj) < schema["minItems"]:
                raise ValueError(f"The given value {obj} does not have at least {schema['minItems']} elements.")
        
        if schema.get("maxItems", None) is not None:
            if len(obj) > schema["maxItems"]:
                raise ValueError(f"The given value {obj} does not have at most {schema['maxItems']} elements.")
        
        if schema.get("uniqueItems", None) is not None:
            if schema["uniqueItems"]:
                if len({str(x) for x in obj}) != len({str(x) for x in obj}):
                    raise ValueError(f"The given value {obj} has duplicated items.")

        return [
            inner_validate_json_schema(root, schema["items"], x)
            for x in obj
        ]
    else:
        raise ValueError(f"Unknown jsonschema type: {schema['type']}")

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
```
""", formatter_class=argparse.ArgumentDefaultsHelpFormatter)

parser.add_argument("build_type", type=str,
    choices=["build", "develop", "publish"],
    help="If the wheel build is for local development and testing or to publish a wheel",
)

parser.add_argument("-s", "--settings-path", type=str,
    default="builder_settings.json",
    help="The path to the json file with the build specification.",
)

parser.add_argument("-p", "--print-settings",
    default=False,
    action="store_true",
    help="Print the settings"
)

parser.add_argument("-v", "--verbosity", type=str,
    default="info",
    choices=["debug", "info", "error"],
    help="Verbosity of the logger"
)

parser.add_argument("-sr", "--skip-repair",
    default=False,
    action="store_true",
    help="""For linux wheels we run `auditwheel repair` on the wheel to be sure to
    include the needed shared libraries. This breaks if the compilation environment
    is not `manylinux_2010` compatible, this flag skips this step.""".replace("\n", ""),
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

settings = validate_json_schema(SETTINGS_SCHEMA, settings)
################################################################################
# Normalize the settings
################################################################################

settings["build_type"] = args.build_type
settings["skip_repair"] = args.skip_repair
settings["metadata"]["arch"] = settings["arch"]

if settings["wheel_root"] is None:
    settings["wheel_root"] = join(settings["wheels_folder"], "wheel_root")
    
if settings["merging_folder"] is None:
    settings["merging_folder"] = join(settings["wheels_folder"], "merging_folder")

if settings["python_files_path"] is None:
    settings["python_files_path"] = join(settings["metadata"]["libname"])

if settings["init_path"] is None:
    settings["init_path"] = join(settings["python_files_path"], "__init__.py")

if settings["target_wheel_path"] is None:
    settings["target_wheel_path"] = join(settings["wheel_root"], gen_wheel_name(settings["metadata"]))


logging.info("%s", settings)

# Build the compilation settings
settings["targets"] = [
    {
        "name":target_name,
        "arch":arch,
        "target_cpu":target_settings["target_cpu"],
        "target_os":target_settings.get("target_os", ["darwin", "windows", "linux"]),
        "build_dir":join(target_settings.get("build_dir", f"build_{arch}_{target_name}")),
        "wheel_dir":join(
            settings["wheels_folder"], 
            target_settings.get(
                "wheel_dir", 
                f"ensmallen_{arch}_{target_name}"
            ), 
        ),
        "lib_name":target_settings.get("lib_name", 
            f"ensmallen_{arch}_{target_name}"
        ),
        "cpu_features": get_cpu_features(arch, target_settings['target_cpu']),
        "crate_features": target_settings.get("crate_features", []),
        "rustflags":target_settings.get("rustflags", ""),
    }
    for arch in settings["targets"].keys()
    for target_name, target_settings in settings["targets"][arch].items()
]

# Keep a copy of only the targets to build
settings["targets_to_build"] = [
    target
    for target in settings["targets"]
    if settings["arch"] == target["arch"] and settings["target_os"] in target["target_os"]
]

# if we are in a develop build override the targets with only the local one
if args.build_type == "develop":
    settings["targets"] = [{
        "name":"develop",
        "arch":settings["arch"],
        "target_cpu":"native",
        "target_os":settings["target_os"],
        "build_dir":"build_develop",
        "wheel_dir":join(settings["wheels_folder"], "ensmallen_develop"),
        "lib_name":"ensmallen_develop",
        "cpu_features":[],
        "crate_features":settings["develop"]["crate_features"],
        "rustflags":settings["develop"]["rustflags"],

    }]
    settings["targets_to_build"] = settings["targets"]

logging.debug("Building with settings:\n%s", json.dumps(settings, indent=4))
if args.print_settings:
    print(json.dumps(settings, indent=4))

################################################################################
# Clean the folders and prepare them to be compiled
################################################################################

def build_target_wheel(settings, target):
    build_dir = target["build_dir"]
    lib_name  = target["lib_name"]
    wheel_dir = target["wheel_dir"]

    logging.info("Deleting old build folder")
    shutil.rmtree(build_dir, ignore_errors=True)
    logging.info("Deleting old wheel folder")
    shutil.rmtree(wheel_dir, ignore_errors=True)
    os.makedirs(wheel_dir, exist_ok=True)

    logging.info("Creating new build folders")
    shutil.copytree(".", build_dir)

    logging.info("Patching the %s build", lib_name)
    patch(join(build_dir, "Cargo.toml"),
        r"""path\s*=\s*\"..""", 
        r"""path = "../..""", 
    )
    patch(join(build_dir, "pyproject.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name="%s" """%lib_name
    )
    patch(join(build_dir, "Cargo.toml"),
        r"name\s*=\s*\".+?\"", 
        r"""name = "%s" """%lib_name
    )
    patch(join(build_dir, "src", "auto_generated_bindings.rs"), 
        r"fn ensmallen\(_py: Python", 
        r"fn %s(_py: Python"%lib_name,
    )   

    logging.info("Renaming the old python code folder")
    shutil.move(
        join(build_dir, "ensmallen"), 
        join(build_dir, lib_name)
    )

    rust_flags = f"-Ctarget-cpu={target['target_cpu']}"

    if settings["shared_rustflags"] is not None:
        rust_flags += " " + settings["shared_rustflags"]

    if target["rustflags"] is not None:
        rust_flags += " " + target["rustflags"]

    if target["crate_features"] is not None:
        features = ",".join(target['crate_features'])
    else:
        features = None

    command = f"""maturin build --release --strip --skip-auditwheel --no-sdist --out {wheel_dir} """
    if features is not None:
        command += f"""--cargo-extra-args="--features={features}" """ 
    logging.info("Compiling with '%s' and flags: '%s'", command, rust_flags)
    exec(
        command, 
        env={
            **os.environ,
            "RUSTFLAGS":rust_flags,
        },
        cwd=build_dir,
    )

    logging.info("Deleting the build folder")
    shutil.rmtree(build_dir, ignore_errors=True)

shutil.rmtree(settings["wheels_folder"], ignore_errors=True)
os.makedirs(settings["wheels_folder"], exist_ok=True)
shutil.rmtree(settings["wheel_root"], ignore_errors=True)
os.makedirs(settings["wheel_root"], exist_ok=True)

logging.info("Cleaning the target folder so we don't copy useless data")
exec("cargo clean", cwd=join("."))
logging.info("Running cargo update")
exec("cargo update", cwd=join("."))
logging.info("Generating the bindings")
exec("cargo run --release --bin bindgen", cwd=join("..", "..", "code_analysis"))

logging.info("Copyng the python library files top the merging folder")
shutil.rmtree(settings["merging_folder"], ignore_errors=True)
shutil.copytree(settings["python_files_path"], settings["merging_folder"])

# Remove caches so that we don't put them in the wheel
for file in glob.iglob(
    join(settings["merging_folder"], "**", "__pycache__"), 
    recursive=True,
):
    shutil.rmtree(file, ignore_errors=True)

logging.info("Generating __init__.py file in the merging folder")
gen_init_file(settings)

# Build all the wheels
logging.info("Building the wheels for each target")
for target in settings["targets_to_build"]:
    build_target_wheel(settings, target)

# Copy the shared library to the merging_folder
logging.info("Copying the compiled libraries to the merging folder")
for target in settings["targets_to_build"]:
    # find the wheel file in its folder
    wheel_path = glob.glob(os.path.join(target["wheel_dir"], "*.whl"))[0]

    with zipfile.ZipFile(wheel_path, "r") as zipread:
        for item in zipread.infolist():
            if item.filename.endswith(settings["library_file_extension"]):
                local_file = join(
                    settings["merging_folder"],
                    os.path.join(*os.path.split(item.filename)[1:]),
                )
                # Copy the file
                with open(local_file, "wb") as f:
                    f.write(zipread.read(item.filename))

logging.info("Creating the final wheel")
gen_wheel(settings)

if args.build_type == "develop":
    exec(
        f"pip --disable-pip-version-check install --force-reinstall {settings['target_wheel_path']}"
    )
elif args.build_type == "publish":
    exec(
        f"twine upload {settings['target_wheel_path']}"
    )
elif args.build_type == "build":
    print(f"To publish run 'twine upload {settings['target_wheel_path']}'")
