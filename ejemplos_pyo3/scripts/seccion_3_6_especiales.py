"""Sección 3.6: métodos especiales y propiedades.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_6_especiales.py
"""
from ejemplos_pyo3 import Vector2D

a = Vector2D(3, 4)
b = Vector2D(1, -2)
print("a + b  =", a + b)
print("a * 2  =", a * 2)
print("-b     =", -b)
print("|a|    =", a.magnitud)
print("cero   =", Vector2D.cero())
print("a == Vector2D(3, 4):", a == Vector2D(3, 4))
