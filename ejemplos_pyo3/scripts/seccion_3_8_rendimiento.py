"""Sección 3.8: medir el rendimiento (modo debug y modo release).

Ejecute desde la carpeta ejemplos_pyo3, primero con `maturin develop` y
luego con `maturin develop --release`:
    python scripts/seccion_3_8_rendimiento.py
"""
import time

import ejemplos_pyo3 as ej
import primos_py

LIMITE = 1_000_000


def medir(nombre, funcion):
    inicio = time.perf_counter()
    resultado = funcion(LIMITE)
    segundos = time.perf_counter() - inicio
    print(f"{nombre:<8} {resultado} primos en {segundos:.3f} s")
    return segundos


t_py = medir("Python", primos_py.contar_primos)
t_rs = medir("Rust", ej.contar_primos)
print(f"Rust fue {t_py / t_rs:.1f} veces más rápido")
