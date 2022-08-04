# Guild to Run the Python Examples

## Build the wheel file
- Go to `iota.rs/bindings/python/native`
- `python3 setup.py bdist_wheel`

## Create a virtual environment and use it (optional)
- `python3 -m venv .venv`
- `source .venv/bin/activate`

## Install the wheel file
- Go to `iota.rs/bindings/python/native/dist`
- `python3 -m pip install [your built wheel file]`

## Run examples
- Go to `iota.rs/bindings/python/native/example`
- `python3 [example file]`

## To deactivate the virtual environment (optional)
- `deactivate`
