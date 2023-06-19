#!/usr/bin/zsh
set -e

source .env

diesel database reset

rm -rf data
mkdir -p data/default

cp -r scripts/default_data/* data/default
