<div align="center">

# RefineIO

[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

_RefineIO facilite la création la transformation et l'export de projets OpenRefine_
</div>

## Installation

Vous pouvez installer la dernière version stable de rustIO depuis la page release de ce dépôt.

### Compilation et installation depuis le code source


Pour compiler refineIO vous disposer de [rust](https://www.rust-lang.org/tools/install) sur votre système.

__Linux__

Sous linux la compilation et l'installation de RefineIO peuvent être réalisées grâce au MAKEFILE présent dans ce dépôt.

```sh
make build
sudo make install
```

__Windows__

```sh
cargo build --release
```

Le binaire de refineIO sera compilé dans le dossier suivant : `target\release`

## Configuration

Par défaut refineIO utilise l'adresse `127.0.0.1:3333` pour se connecter au serveur OpenRefine. Vous pouvez changer ce comportement en ajoutant une variable d'environement `REFINE_URL` à votre système ou dans un fichier `.env` situé dans le répertoire ou est exécuté refineIO.

__Sous linux__

```sh
export REFINE_URL=ADRESSE_DU_SERVEUR
```

__Sous Windows__

```cmd
setx REFINE_URL "ADRESSE_DU_SERVEUR"
```

__Dans un fichier .env (Windows et Linux)__

Le contenu du fichier `.env` doit contenir

```sh
REFINE_URL=ADRESSE_DU_SERVEUR
```


## Usages

RefineIO permet de réaliser trois grands types d'opérations avec OpenRefine : 

* __la création de projets__
* __l'application de scripts__
* __l'export de projets__

### Création d'un projet OpenRefine

Les projets OpenRefine peuvent être créés à partir de plusieurs sources :

* une URL
* un fichier de données
* des données passées par l'entrée standard

Les données d'entrée peuvent être au format csv, tsv, json et xml.


__Import depuis un fichier__ `--input=FICHIER`


```sh
refine-io --input=playground/input.json --format=json --record-path '["_", "response", "docs", "_"]'
```

L'option __record-path__ permet de préciser la manière dont les fichiers json et xml doivent être parsés par OpenRefine.

__Import depuis une URL__ `--input=URL`

```sh
refine-io --input="http://www.theses.fr/?q=*:*&format=json" --format=json --record-path '["_", "response", "docs", "_"]'
```

Seules les URLs valides et ayant un protocole (http, https, etc.) sont acceptées par refineIO.

__Import depuis l'entrée standard__

```sh
cat playground/input.json | refine-io --format=json --record-path '["_", "response", "docs", "_"]'
```

__Ouverture automatique du projet OpenRefine__ `--open-project`

Après sa création le projet OpenRefine peut être ouvert automatiquement dans le nagivateur.

```sh
refine-io --input=playground/input.json --format=json --record-path '["_", "response", "docs", "_"]' --open-project
```

### Application de scripts

Les scripts peuvent être appliqués sur un projet existant ou sur un projet créé.

__Après la création d'un projet__ `--script=SCRIPT.json`

```sh
refine-io --input=playground/input.json --format=json --record-path '["_", "response", "docs", "_"]' --script=playground/script.json
```

__Sur un projet existant__ `--script=SCRIPT.json`

```sh
refine-io --project-id=123456789 --script=playground/script.json
```

### Export d'un projet


