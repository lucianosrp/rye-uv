# Rye ➡️ Uv
Since `v0.3.0`, uv can now handle python projects and packaging.
It is therefore very similar to what rye can do.
Although rye will still be in active development, you may want to give uv a try since you probably have it already installed.

This tool aims to facilitate the transition from rye to uv.
It converts the `tool.rye` setttings in `pyproject.toml`  to `tool.uv` and make adjustments when needed.

## Usage
```
Usage: rye-uv [OPTIONS] <PYPROJECT>

Arguments:
  <PYPROJECT>  The path of the pyproject.toml file

Options:
      --no-overwrite  Whether to overwrite the existing pyproject.toml - defaults to false
      --no-backup     Don't create a backup file of the original rye config - defaults to false
  -p, --print         Just print the output file
  -h, --help          Print help
  -V, --version       Print version
```

By doing the default:

```
rye-uv pyproject.toml
```
1. The pyproject.toml file will be overwritten
2. The original version of the file will be stored in `pyproject-rye.toml`

```
rye-uv pyproject.toml --no-overwrite
```
1. A new `pyproject-uv.toml` will be creared


## How to install

### With Cargo
```
cargo add rye-uv
```

### From source
Clone the repository and build using cargo
```
cargo build --release
```

## Adjustments
### tool.rye
All `tool.rye` will be renamed `tool.uv`.

### tool.rye.universal
If set, `tool.rye.universal` will go to `tool.uv.pip.universal`.

### tool.rye.generate-hashes
If set, `tool.rye.generate-hashes` will go to `tool.uv.pip.generate-hashes`.

### tool.rye.lock-with-sources
If set, `tool.rye.lock-with-sources` will go to `tool.uv.no-source`.
If it was set to true, then it will be set to false and vice-versa.

### tool.rye.virtual
All `tool.rye.virtual` will be removed.
