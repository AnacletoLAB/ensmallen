"""Module offering fast graph processing and graph datasets."""

import cpuinfo
import warnings

# TODO!: while ensmallen is fully cross-compatible, this __init__ only supports
# x86_64. For other archs just remove these checks
AVX_FLAGS = ["sse","sse2","sse4a","avx","avx2","bmi1","bmi2","popcnt","cmov"]
NO_AVX_FLAGS = ["sse","sse2","fxsr"]


unavailable_flags = set(AVX_FLAGS) - set(cpuinfo.get_cpu_info()["flags"])

if len(unavailable_flags) == 0:
    from .ensmallen import preprocessing  # pylint: disable=import-error
    from .ensmallen import Graph  # pylint: disable=import-error
    from .ensmallen import edge_list_utils  # pylint: disable=import-error
else:
    warnings.warn(
        (
            "On the current machine, the flags '{}' are not available.\n"
            "The library will use a slower but more compatible version."
        ).format(unavailable_flags)
    )

    unavailable_flags = set(NO_AVX_FLAGS) - set(cpuinfo.get_cpu_info()["flags"])

    if len(unavailable_flags) == 0:
        from .ensmallen_no_avx import preprocessing  # pylint: disable=import-error
        from .ensmallen_no_avx import Graph  # pylint: disable=import-error
        from .ensmallen_no_avx import edge_list_utils  # pylint: disable=import-error
    else:
        raise ValueError(
            (
                "On the current machine, the flags '{}' are not available.\n"
                "This library was compiled assuming that the following instruction "
                "sets are available: {}\n"
                "You can solve this issue by manually compiling ensmallen tailoring"
                " it to your hardware following the guides on our Github repository."
                " Ensmallen has no strict dependancy on any flag or cpu_arch as it "
                "can be compiled for any arch supported by LLVM (Arm, AArch64, Mips,"
                " ...).\n"
            ).format(unavailable_flags, NO_AVX_FLAGS)
        )

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = ["edge_list_utils", "Graph", "preprocessing", "datasets"]
