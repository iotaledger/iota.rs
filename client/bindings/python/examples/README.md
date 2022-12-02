# Guild to Run the Python Examples

## Build the wheel file

- Go to `iota.rs/bindings/python`
- `python3 setup.py bdist_wheel`

## Create a virtual environment and use it (optional)

- `python3 -m venv .venv`
- `source .venv/bin/activate`

## Install the wheel file

`python3 -m pip install dist/[your built wheel file]`

Example:

- `python3 -m pip install dist/iota_client-0.1.0-cp310-cp310-linux_x86_64.whl`

Use `--force-reinstall` when already installed

Example: 

- `python3 -m pip install dist/iota_client-0.1.0-cp310-cp310-linux_x86_64.whl --force-reinstall`

## Run examples

`python3 example/[example file]`

Example: 

- `python3 examples/00_get_info.py`

## To deactivate the virtual environment (optional)

- `deactivate`

## All in one example

`python3 setup.py bdist_wheel && python3 -m pip install dist/iota_client-0.1.0-cp310-cp310-linux_x86_64.whl --force-reinstall && python3 examples/00_get_info.py`

## Build docs

`pydoc-markdown -p iota_client > README.md`