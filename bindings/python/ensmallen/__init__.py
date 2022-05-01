
import logging
import platform
import cpuinfo
import importlib
import ctypes

target_os = platform.system().lower()

# Linux call it aarch64 while mac calls it arm64 :)
arch = {
    "arm64":"aarch64", 
}.get(platform.machine(), platform.machine())

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

targets = [{'arch': 'x86_64', 'target_os': 'linux', 'crate_features': ['cuda'], 'cpu_features': [], 'lib_name': 'ensmallen_develop'}]

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
logging.info("Ensmallen choosed target: {}".format(choosen_target))

_lib = __import__(
    "ensmallen.{}".format(choosen_target["lib_name"]),
    fromlist=("Graph",),
)

locals().update({
    key:value
    for key, value in vars(_lib).items()
    if not key.startswith("_")
})

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = ["edge_list_utils", "Graph", "preprocessing", "models", "datasets"]
    