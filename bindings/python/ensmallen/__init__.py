"""Module offering fast graph processing and graph datasets."""
import logging
import warnings
import platform
from environments_utils import is_x86_64, is_arm

if is_x86_64():
    import cpuinfo
    HASWELL_FLAGS = [
        "avx2",
        "bmi2",
        "popcnt",
    ]
    unavailable_flags = set(HASWELL_FLAGS) - set(cpuinfo.get_cpu_info()["flags"])

    if len(unavailable_flags) == 0:
        logging.info("Ensmallen is using Haswell")
        from . import ensmallen_haswell as core 
        from .ensmallen_haswell import * 
        del ensmallen_haswell
    else:
        warnings.warn(
            (
                "Ensmallen is compiled for the Intel Haswell architecture (2013)."
                "On the current machine, the flags '{}' are required but '{}' are not available.\n"
                "The library will use a slower but more compatible version (Intel Core2 2006)."
            ).format(HASWELL_FLAGS, unavailable_flags)
        )
        
        CORE2_AVX_FLAGS = [
            "ssse3", 
            "fxsr", 
            "cx16", #"cmpxchg16b"
        ]
        unavailable_flags = set(CORE2_AVX_FLAGS) - set(cpuinfo.get_cpu_info()["flags"])

        if len(unavailable_flags) == 0:
            logging.info("Ensmallen is using Core2")
            from . import ensmallen_core2 as core 
            from .ensmallen_core2 import * 
            del ensmallen_core2
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
                ).format(unavailable_flags, CORE2_AVX_FLAGS)
            )
        del CORE2_AVX_FLAGS
    del cpuinfo
    del unavailable_flags
    del HASWELL_FLAGS
elif is_arm():
    logging.info("Ensmallen is using Default Arm")
    from . import ensmallen_default as core 
    from .ensmallen_default import * 
    del ensmallen_default
else:
    raise ValueError("The arch '{}' is not currently supproted by ensmallen's init file.".format(platform.machine()))

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = [
    key
    for key in dir(core)
    if not key.startswith("_")
]

del core
del logging
del warnings
del platform
del is_x86_64
del is_arm
