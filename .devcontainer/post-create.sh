#!/bin/bash
cd /workspaces/path_to_solana/.devcontainer || exit

printf "Ejecutar configuración de neovim.\n"
./nvim-setup.sh
