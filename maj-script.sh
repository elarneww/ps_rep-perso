#!/usr/bin/env bash
set -e  # stoppe si une commande échoue

# === CONFIG ===
# Chemin vers ton repo local (change ici si nécessaire)
REPO_DIR="$HOME/PS/ps_rep-perso"

# Chemin vers le dossier Windows (optionnel)
WINDOWS_HTML_DIR="/mnt/c/Users/asus/SUPPORT/01_Study_SUPPORT/01_License&info/02_Cours _Universitaire/L3_COURS/L3/Semestre 5/IN512 - Programmation Système/prog-efficace"

echo "🔹 Mise à jour du repo du prof..."
cd "$REPO_DIR" || { echo "❌ Repo introuvable : $REPO_DIR"; exit 1; }

# Passe sur main et update depuis upstream
git checkout main
git pull upstream main

# Passe sur ta branche notes (la crée si elle n’existe pas encore)
if git show-ref --verify --quiet refs/heads/notes; then
    git checkout notes
else
    git checkout -b notes
fi

# Merge main dans notes
git merge main || echo "⚠️ Conflits détectés, à résoudre manuellement"

# Build du HTML
echo "🔹 Build du HTML avec bundle exec..."
bundle exec asciidoctor -r asciidoctor-diagram -D html/ src/index.adoc

# Copie vers Windows si le dossier existe
if [ -d "$WINDOWS_HTML_DIR" ]; then
    echo "🔹 Copie du HTML vers Windows..."
    cp -r html/ "$WINDOWS_HTML_DIR"
else
    echo "⚠️ Dossier Windows non trouvé ($WINDOWS_HTML_DIR), HTML non copié"
fi

echo "✅ Build et mise à jour terminés !"

