"""Versión en Python puro del mismo algoritmo, para comparar."""


def entrenar(x, y, tasa=0.01, epocas=1000):
    n = len(x)
    columnas = len(x[0])
    pesos = [0.0] * columnas
    sesgo = 0.0
    for _ in range(epocas):
        grad_pesos = [0.0] * columnas
        grad_sesgo = 0.0
        for fila, objetivo in zip(x, y):
            error = sum(w * xj for w, xj in zip(pesos, fila)) + sesgo - objetivo
            for j in range(columnas):
                grad_pesos[j] += error * fila[j]
            grad_sesgo += error
        for j in range(columnas):
            pesos[j] -= tasa * 2 * grad_pesos[j] / n
        sesgo -= tasa * 2 * grad_sesgo / n
    return pesos, sesgo
