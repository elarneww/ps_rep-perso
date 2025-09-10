#!/bin/bash
# workflow.sh - workflow pro PS/TD

set -e  # stoppe le script si une commande échoue

echo "=== Pull du repo perso ==="
git pull

echo "=== Mise à jour de tous les submodules ==="
git submodule update --init --recursive
git submodule foreach git pull origin main || echo "⚠️ Submodule à mettre à jour manuellement"

echo "=== Rappels VS Code / Setup ==="
echo "⚠️ Vérifie que les extensions suivantes sont installées dans VS Code :"
echo "    - AsciiDoc (AsciiDoc support)"
echo "    - Rust Analyzer"
echo ""
echo "⚠️ N'oublie pas de lancer ./setup.sh si nécessaire pour installer les outils"

echo "=== Workflow terminé ==="
