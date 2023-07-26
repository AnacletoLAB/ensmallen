import os, subprocess

env = os.environ
env["PYO3_CROSS_INCLUDE_DIR"] = "/dfd/python_cross/windows/Python37/include"
env["PYO3_CROSS_LIB_DIR"] = "/dfd/python_cross/windows/Python37/libs"
#env["CC"] = "zig cc -target x86_64-windows"
#env["CXX"] = "zig c++ -target x86_64-windows"
#env["AR"] = "zig ar"
#env["RUSTFLAGS"] = "-Clinker=/bin/ziglib"

subprocess.check_call(
    "cargo zigbuild --target x86_64-pc-windows-gnu --release --manifest-path=bindings/python/Cargo.toml",
    env=env,
    shell=True,
)
