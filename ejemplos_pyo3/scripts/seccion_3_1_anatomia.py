"""Sección 3.1: anatomía de un módulo.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_1_anatomia.py
"""
import ejemplos_pyo3 as ej

print(ej.saludar("Ana"))
print("20 + 22 =", ej.sumar(20, 22))
print("Versión del módulo:", ej.VERSION)
print("Documentación:", ej.sumar.__doc__)
