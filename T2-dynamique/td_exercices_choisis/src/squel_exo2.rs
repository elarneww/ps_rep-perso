Fonction sieve(n):
    Si n < 2 :
        Retourner un vecteur vide

    # 1️⃣ Initialiser un tableau de booléens
    is_prime[0..n] = true
    is_prime[0] = false
    is_prime[1] = false

    # 2️⃣ Boucler sur tous les entiers possibles
    Pour i de 2 à √n inclus :
        Si is_prime[i] est vrai :
            # 3️⃣ Marquer tous les multiples de i comme non premiers
            Pour multiple de i*i à n avec un pas de i :
                is_prime[multiple] = false

    # 4️⃣ Collecter tous les indices encore vrais
    nombres_premiers = []
    Pour i de 2 à n :
        Si is_prime[i] est vrai :
            Ajouter i à nombres_premiers

    Retourner nombres_premiers
