"""Datos sintéticos: y = 4·x1 − 2·x2 + 0.5·x3 + 7 + ruido."""
import random

PESOS_REALES = [4.0, -2.0, 0.5]
SESGO_REAL = 7.0


def generar(n, semilla=42):
    rng = random.Random(semilla)
    x, y = [], []
    for _ in range(n):
        fila = [rng.uniform(-1, 1) for _ in PESOS_REALES]
        valor = sum(w * xi for w, xi in zip(PESOS_REALES, fila)) + SESGO_REAL
        x.append(fila)
        y.append(valor + rng.gauss(0, 0.3))
    return x, y


def dividir(x, y, proporcion_prueba=0.2):
    corte = int(len(x) * (1 - proporcion_prueba))
    return x[:corte], x[corte:], y[:corte], y[corte:]
