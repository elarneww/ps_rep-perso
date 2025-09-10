#!/usr/bin/env bash
set -e  # stoppe si une commande échoue

echo "[1/4] Mise à jour et installation de Ruby + outils..."
sudo apt update
sudo apt install -y ruby-full build-essential git wget

echo "[2/4] Installation de bundler et rake..."
sudo gem install bundler rake

echo "[3/4] Clonage du dépôt du cours..."
if [ ! -d prog-efficace ]; then
    git clone https://github.com/uvsq-pef/prog-efficace.git
fi

echo "[4/4] Installation des dépendances Ruby..."
cd prog-efficace
bundle install

echo "✅ Environnement prêt. Lance maintenant :"
echo "   bundle exec asciidoctor -r asciidoctor-diagram -D html/ src/index.adoc"
 