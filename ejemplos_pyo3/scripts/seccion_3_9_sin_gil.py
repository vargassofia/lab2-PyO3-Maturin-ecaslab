"""Sección 3.9: liberar el GIL en cálculos largos.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop --release`:
    python scripts/seccion_3_9_sin_gil.py
"""
import time
from concurrent.futures import ThreadPoolExecutor

import ejemplos_pyo3 as ej

LIMITES = [3_000_000] * 4  # cuatro tareas iguales


def en_hilos(funcion):
    inicio = time.perf_counter()
    with ThreadPoolExecutor(max_workers=4) as hilos:
        list(hilos.map(funcion, LIMITES))
    return time.perf_counter() - inicio


print(f"Con GIL:  {en_hilos(ej.contar_primos):.3f} s")
print(f"Sin GIL:  {en_hilos(ej.contar_primos_sin_gil):.3f} s")
