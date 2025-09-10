2️⃣ Workflow générique “pro”

Travailler dans le submodule pour le TD → commits/push dans Classroom

Travailler dans ton repo perso pour scripts et setup

Pour versionner tout ce que tu as fait dans ton repo perso :

git checkout notes            # ou main
git add -A                     # tout ce qui a changé
git commit -m "TD1 / mise à jour scripts"
git push

2️⃣ Nouveaux repos Classroom

Si ton prof rajoute un nouveau repo Classroom :

Tu ajoutes un nouveau submodule dans ton repo perso :
git submodule add <url_du_nouveau_repo> T1-Syntaxe/nom_du_nouveau_repo
git add .gitmodules
git commit -m "Ajout du submodule TD2"
git push
