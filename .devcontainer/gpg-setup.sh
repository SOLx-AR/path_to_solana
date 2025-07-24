#!/bin/bash
set -e

echo "[gpg] Seteando una llave GPG de prueba (solo para desarrollo)"

gpg --batch --gen-key <<EOF
%no-protection
Key-Type: default
Key-Length: 2048
Subkey-Type: default
Name-Real: Dev User
Name-Email: dev@example.com
Expire-Date: 0
%commit
EOF

KEY_ID=$(gpg --list-secret-keys --with-colons | awk -F: '/^sec/ { print $5; exit }')

if [ -n "$KEY_ID" ]; then
    git config --global user.signingkey "$KEY_ID"
    git config --global commit.gpgsign true
    echo "GPG key created and added to git config: $KEY_ID"
else
    echo "Failed to generate GPG key"
fi
