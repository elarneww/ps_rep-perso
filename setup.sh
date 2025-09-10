#!/bin/bash
# setup.sh - configuration environnement PS / TD
set -e

echo "=== Mise à jour des paquets ==="
sudo apt update
sudo apt upgrade -y

echo "=== Installation des outils ==="
sudo apt install -y asciidoctor git curl build-essential

echo "=== Submodule Classroom ==="
git submodule update --init --recursive

echo "=== Rappels VS Code ==="
echo "⚠️ N'oublie pas d'installer les extensions suivantes dans VS Code :"
echo "    - AsciiDoc (AsciiDoc support)"
echo "    - Rust Analyzer"

echo "=== Setup terminé ==="
