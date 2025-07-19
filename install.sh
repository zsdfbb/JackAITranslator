#!/bin/bash

mkdir -p ~/.config/aitrans
cp aitrans.toml ~/.config/aitrans/config.toml
cp aitrans/target/release/aitrans ${HOME}/.local/bin/

echo "Finish install aitrans configuration"

