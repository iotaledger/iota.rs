
import sys
import platform

from setuptools import setup
from setuptools_rust import RustExtension


def get_py_version_cfgs():
    # For now each Cfg Py_3_X flag is interpreted as "at least 3.X"
    version = sys.version_info[0:3]
    py3_min = 8
    out_cfg = []
    for minor in range(py3_min, version[1] + 1):
        out_cfg.append("--cfg=Py_3_%d" % minor)

    if platform.python_implementation() == "PyPy":
        out_cfg.append("--cfg=PyPy")

    return out_cfg


setup(
    name="iota_client",
    version="1.0.0-rc.3",
    classifiers=[
        "License :: SPDX-License-Identifier ::  Apache-2.0",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["iota_client"],
    rust_extensions=[
        RustExtension(
            "iota_client.iota_client",
            rustc_flags=get_py_version_cfgs(),
            debug=False,
        ),
    ],
    include_package_data=True,
    zip_safe=False,
)
