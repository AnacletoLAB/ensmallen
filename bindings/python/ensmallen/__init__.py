"""Module offering fast graph processing and graph datasets."""
import logging
import warnings
import platform
from environments_utils import is_x86_64, is_arm, is_macos_rosetta, get_macos_available_instructions

if is_x86_64():
    if is_macos_rosetta():
        available_flags = set(get_macos_available_instructions())
        FLAGS_WARNING_MESSAGE = (
            " Please be advised that you are currently executing ensmallen within "
            "the Rosetta Translation Environment. This means that the performance "
            "of the library will be significantly worse than the one you would "
            "run this natively on either an Intel CPU or an Apple Silicon CPU."
        )
    else:
        try:
            import cpuinfo
            FLAGS_WARNING_MESSAGE = ""
            available_flags = set(cpuinfo.get_cpu_info().get("flags", []))
            del cpuinfo
        except Exception as exception: # pylint: disable=broad-except
            FLAGS_WARNING_MESSAGE = (
                " Please be advised that you are currently executing ensmallen "
                "within an environment where the cpuinfo package is not available "
                "or fails to run. Because of this, to avoid error relative to "
                "potentially unsupported instructions, ensmallen will use a "
                "slower but more compatible version (Intel Core2). Consider "
                "making sure that the cpuinfo package is available and working "
                "to avoid this warning. The error was:\n"
                f"{exception}"
            )
            available_flags = set()
            
    # Check if the machine supports the required flags
    # requested by the Haswell version of the library. 
    REQUIRED_FLAGS = {
        "avx2",
        "bmi2",
        "popcnt",
    }
    
    # We get the set of flags that are not available
    unavailable_flags = REQUIRED_FLAGS - available_flags

    # If all the flags are available, we can use the Haswell version
    if len(unavailable_flags) == 0:
        logging.info("Ensmallen is using Haswell")
        from . import ensmallen_haswell as core # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self
        from .ensmallen_haswell import * # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self

        del ensmallen_haswell # pylint: disable=undefined-variable
    else:
        # Otherwise we use the Core2 version, which is slower but more compatible

        warnings.warn(
            (
                "Ensmallen is compiled for the Intel Haswell architecture (2013)."
                f"On the current machine, the flags '{REQUIRED_FLAGS}' are required but '{unavailable_flags}' are not available.\n"
                f"The library will use a slower but more compatible version (Intel Core2 2006).{FLAGS_WARNING_MESSAGE}"
            )
        )

        # Check if the machine supports the required flags
        # requested by the Core2 version of the library.
        REQUIRED_FLAGS = {
            "ssse3",
            "fxsr",
            "cx16",  # "cmpxchg16b"
        }

        # We get the set of flags that are not available
        unavailable_flags = REQUIRED_FLAGS - available_flags

        # If all the flags are available, we can use the Core2 version
        if len(unavailable_flags) == 0:
            logging.info("Ensmallen is using Core2")
            from . import ensmallen_core2 as core # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self
            from .ensmallen_core2 import * # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self

            del ensmallen_core2 # pylint: disable=undefined-variable
        else:
            # Otherwise we raise an error, as the library cannot be used
            # on the current machine.
            raise ValueError(
                (
                    f"On the current machine, the flags '{unavailable_flags}' are not available.\n"
                    "This library was compiled assuming that the following instruction "
                    f"sets are available: {REQUIRED_FLAGS}\n"
                    "You can solve this issue by manually compiling ensmallen tailoring"
                    " it to your hardware following the guides on our Github repository."
                    " Ensmallen has no strict dependancy on any flag or cpu_arch as it "
                    "can be compiled for any arch supported by LLVM (Arm, AArch64, Mips,"
                    " ...).\n"
                )
            )
    
    del unavailable_flags
    del FLAGS_WARNING_MESSAGE
    del REQUIRED_FLAGS
elif is_arm():
    logging.info("Ensmallen is using Default Arm")
    from . import ensmallen_default as core # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self
    from .ensmallen_default import * # pylint: disable=import-outside-toplevel, import-error, no-name-in-module, import-self

    del ensmallen_default # pylint: disable=undefined-variable
else:
    raise ValueError(
        f"The arch '{platform.machine()}' is not currently supported by ensmallen's init file."
    )

# Because otherwise it generate a Circular import and crash
from . import datasets

__all__ = [key for key in dir(core) if not key.startswith("_")]

del core
del logging
del warnings
del platform
del is_x86_64
del is_arm
