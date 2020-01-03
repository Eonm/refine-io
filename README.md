<div align="center">

# RefineIO

[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

_RefineIO facilite la création la transformation et l'export de projets OpenRefine_
</div>

## Usages

```sh
refine-io --input=playground/input.json --format=csv --record-path '["_", "response", "docs", "_"]' -e csv
cat playground/input.json | refine-io --format csv --record-path '["_", "response", "docs", "_"]' -e csv
```

### Appliquer un script `--script`

```sh
refine-io --input=playground/input.json --script=playground/script.json --format=csv --record-path '["_", "response", "docs", "_"]' -e csv
cat playground/input.json | refine-io --format csv --record-path '["_", "response", "docs", "_"]' -e csv
```

### Accéder au projet OpenRefine `--open-project`

### Exporter les données `-e FORMAT`

### Exporter les données dans un fichier `-e FORMAT -o NOM_DU_FICHIER`