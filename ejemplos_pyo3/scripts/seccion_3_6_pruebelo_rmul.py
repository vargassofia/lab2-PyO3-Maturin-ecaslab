"""Sección 3.6, Pruébelo usted: __rmul__.

Sin __rmul__, `2 * a` lanza:
    TypeError: unsupported operand type(s) for *: 'int' and 'ejemplos_pyo3.Vector2D'
porque 2 * a busca el método __rmul__ del vector. Con __rmul__ (que reutiliza
__mul__) funciona igual que `a * 2`.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_6_pruebelo_rmul.py
"""
from ejemplos_pyo3 import Vector2D

a = Vector2D(3, 4)
print("a * 2  =", a * 2)
print("2 * a  =", 2 * a)
