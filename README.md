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
cargo install rye-uv
```

### From source
Clone the repository and build using cargo
```
cargo build --release
```

## Conversion

Some boolean values will be "inverted" to match.
\
Other settings may need to be deleted in orther to make the pyproject.toml fully compatible.
\
See the table below for more.

| rye setting | uv setting | rye docs | uv docs |  uv version | Inverted | Deleted
| ----------- | ---------- | -------- | ------- | -------------- | --------|-----------| 
| `tool.rye`    | `tool.uv`    |  [Link](https://rye.astral.sh/guide/pyproject/#toolryeuniversal)     |   [Link](https://docs.astral.sh/uv/reference/settings/#pip_universal)      |  >=0.3.0         |||
| `tool.rye.universal`   | `tool.uv.pip.universal`    | [Link](https://rye.astral.sh/guide/pyproject/#toolryeuniversal)      |    [Link](https://docs.astral.sh/uv/reference/settings/#pip_universal)     |  >=0.3.0         |||
| `tool.rye.generate-hashes`  | `tool.uv.pip.generate-hashes`    | [Link](https://rye.astral.sh/guide/pyproject/#toolryegenerate-hashes)     |   [Link](https://docs.astral.sh/uv/reference/settings/#pip_generate-hashes)    |   >=0.3.0         |||
| `tool.rye.lock-with-sources`  | `tool.uv.no-source`    | [Link](https://rye.astral.sh/guide/pyproject/#toolryegenerate-hashes)     |   [Link](https://docs.astral.sh/uv/reference/settings/#pip_generate-hashes)    |   >=0.3.0         | ✅ ||
| `tool.rye.virtual`  |                   |  [Link](https://rye.astral.sh/guide/pyproject/#toolryevirtual)     |     |   >=0.3.0,<0.4.0         |  | ✅  ||
| `tool.rye.virtual`  | `tool.uv.package`|  [Link](https://rye.astral.sh/guide/pyproject/#toolryevirtual)     |  [Link](https://docs.astral.sh/uv/reference/settings/#package)   |   >=0.4.0 |✅||         




