<div align="center">

# 💎 RefineIO

[![Build Status](https://travis-ci.com/Eonm/refine-io.svg?branch=master)](https://travis-ci.com/Eonm/refine-io)
[![Coverage Status](https://coveralls.io/repos/github/Eonm/refine-io/badge.svg?branch=master)](https://coveralls.io/github/Eonm/refine-io?branch=master)
[![dependency status](https://deps.rs/repo/github/eonm/refine-io/status.svg)](https://deps.rs/repo/github/eonm/refine-io)
[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

_RefineIO facilite la création la transformation et l'export de projets OpenRefine_
</div>

## Installation

Vous pouvez installer la dernière version stable de refineIO depuis la page [release](https://github.com/Eonm/refine-io/releases) de ce dépôt.

### Compilation et installation depuis le code source


Pour compiler refineIO vous devez avoir installé [rust](https://www.rust-lang.org/tools/install) sur votre système.

__Linux__

Sous Linux la compilation et l'installation de refineIO peuvent être réalisées grâce au Makefile présent dans ce dépôt.

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

Par défaut refineIO utilise l'adresse `127.0.0.1:3333` pour se connecter au serveur OpenRefine. Vous pouvez changer ce comportement en ajoutant une variable d'environnement `REFINE_URL` à votre système ou dans un fichier `.env` situé dans le répertoire ou est exécuté refineIO.

__Sous Linux__

```sh
export REFINE_URL=ADRESSE_DU_SERVEUR
```

__Sous Windows__

```cmd
setx REFINE_URL "ADRESSE_DU_SERVEUR"
```

__Dans un fichier .env (Windows et Linux)__

Le contenu du fichier `.env` doit être :

```sh
REFINE_URL=ADRESSE_DU_SERVEUR
```

## Usages

RefineIO permet de réaliser quatre grands types d'opérations avec OpenRefine : 

* __la création de projets__
* __l'application de scripts__
* __l'export de projets__
* __la suppression de projets__

```sh
refine-io -h
```

### Création d'un projet OpenRefine

Les projets OpenRefine peuvent être créés à partir de plusieurs sources :

* une URL
* un fichier de données
* des données passées par l'entrée standard

Les données d'entrée peuvent être au format csv, tsv, json et xml.


__Import depuis un fichier__ `--input=FICHIER`


```sh
refine-io --input=playground/input.json --format=json --record-path='["_", "response", "docs", "_"]'
```

L'option __record-path__ permet de préciser la manière dont les fichiers json et xml doivent être analysés par OpenRefine.

__Import depuis une URL__ `--input=URL`

```sh
refine-io --input="http://www.theses.fr/?q=*:*&format=json" --format=json --record-path='["_", "response", "docs", "_"]'
```

Seul les URLs valides et ayant un protocole (http, https, etc.) sont acceptées par refineIO.

__Import depuis l'entrée standard__

```sh
cat playground/input.json | refine-io --format=json --record-path='["_", "response", "docs", "_"]'
```
__Spécifier un nom de projet__ `--name=NOM`

Par défaut le nom d'un projet correspond à la date du jour au format UTC. Il est possible de spécifier le nom du projet avec l'option `--name`. Cette option ne peut être utilisée que pour la création d'un projet.

```sh
refine-io --input=playground/input.json --format=json --record-path='["_", "response", "docs", "_"]' --name="nom du projet"
```

__Ouverture automatique du projet OpenRefine__ `--open-project`

Après sa création le projet OpenRefine peut être ouvert automatiquement dans le navigateur.

```sh
refine-io --input=playground/input.json --format=json --record-path='["_", "response", "docs", "_"]' --open-project
```

### Application de scripts

Les scripts peuvent être appliqués sur un projet existant ou sur un projet créé. 

Par défaut les scripts sont appliqués de manière asynchrone, c'est-à-dire que refineIO n'attendra pas qu'ils aient été entièrement appliqués avant de passer aux opérations suivantes comme l'export. Il est possible de forcer refineIO a appliquer les scripts de manière synchrone avec l'option `--sync`. 

__Après la création d'un projet__ `--script=SCRIPT.json`

```sh
refine-io --input=playground/input.json --format=json --record-path='["_", "response", "docs", "_"]' --script=playground/script.json
```

__Sur un projet existant__ `--script=SCRIPT.json`

```sh
refine-io --project-id=123456789 --script=playground/script.json
```

### Exporter les données d'un projet

RefineIO permet d'exporter ou d'afficher les données d'un projet OpenRefine.

__Afficher les données d'un projet dans la sortie  standard__ `--print=FORMAT`

Les données peuvent être affichées au format csv, tsv et html.

```sh
refine-io --project-id=123456789 --print csv
```

```sh
refine-io --project-id=123456789 --print csv > data.csv
```

__Exporter les données dans un fichier__ `--export=FORMAT`

Les données peuvent être exportées au format csv, tsv, html, xsl, xsls et ods.

Si aucun nom de fichier n'est spécifié à l'export `-o` le fichier téléchargé portera le nom du projet OpenRefine.

```sh
refine-io --project-id=123456789 --export csv
```

```sh
refine-io --project-id=123456789 --export csv -o data.csv
```

__Ouverture automatique du fichier exporté__ `--open-export`

Les données exportées peuvent être ouvertes dans le logiciel par défaut de votre système d'exploitation.

```sh
refine-io --project-id=123456789 --export csv --open-export
```

## Suppression du projet

Le projet OpenRefine peut être supprimé grâce à l'option `--clean`. __Attention cette action est irréversible.__ Aucune confirmation ne sera demandée par refineIO.

```sh
refine-io --project-id=123456789 --export csv --open-export
```

## Variables d'environnement

Certaines variables d'environnement peuvent être utilisées pour changer le comportement de refineIO

`RECORD_PATH` = définit le record path à utiliser par défaut

`RUST_LOG` = permet de définir le niveau de log ("trace", "debug", "info", "warn" et "error"). Voir [env_logger](https://docs.rs/crate/).

`CHECK_ASYNC_INTERVAL` = il correspond à l'intervalle (ms) de vérification des processus asynchrone d'OpenRefine. Cette variable est utilisée par l'option `--sync` 