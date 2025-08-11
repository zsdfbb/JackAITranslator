#!/bin/bash

mkdir -p ~/.config/aitrans
cp aitrans.toml ~/.config/aitrans/config.toml

CMD=$1
mkdir -p ${HOME}/.local/bin/
if [ "$CMD" = "--bin" ]; then
    wget https://github.com/zsdfbb/JackAITranslator/releases/download/v1.0.1/aitrans_linux_x86_64 -O ${HOME}/.local/bin/aitrans
    chmod +x ${HOME}/.local/bin/aitrans
else
    cd aitrans
    cargo build --release
    cp target/release/aitrans ${HOME}/.local/bin/
fi

echo "Finish install aitrans configuration"
