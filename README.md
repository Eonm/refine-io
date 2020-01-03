# Refine-io

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