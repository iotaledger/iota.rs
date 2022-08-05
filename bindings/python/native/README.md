# IOTA Client Library - Python binding

Python binding to the iota.rs client library.

## Requirements

Python 3

pip>=19.1
setuptools-rust>=0.10.2

`Rust` and `Cargo`, to compile the binding. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Installation

### Build the wheel file
- Go to `iota.rs/bindings/python/native`
- `python3 setup.py bdist_wheel`

### Create a virtual environment and use it (optional)
- `python3 -m venv .venv`
- `source .venv/bin/activate`

### Install the wheel file
`python3 -m pip install dist/[your built wheel file]`

Example:
- `python3 -m pip install dist/iota_client-0.1.0-cp310-cp310-linux_x86_64.whl`

### Run examples
`python3 example/[example file]`

Example: 
- `python3 examples/00_get_info.py`

### To deactivate the virtual environment (optional)
- `deactivate`

## Getting Started

After you installed the library, you can create a `IotaClient` instance and interface with it.

```python
from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
```

## Build docs
`pydoc-markdown -p iota_client > ../../../documentation/docs/libraries/python/api_reference.md`
