"""Sección 3.8: versión en Python puro de contar_primos, para comparar."""


def es_primo(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def contar_primos(limite):
    return sum(1 for n in range(2, limite) if es_primo(n))
