"""Sección 3.2: conversión de tipos.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_2_conversion.py
"""
import ejemplos_pyo3 as ej

print(ej.promedio([90.0, 85.5, 100]))  # los int también se aceptan
letras = ej.contar_letras("banana")
print(sorted(letras.items()), type(letras))
print(ej.buscar(["Ana", "Luis", "Diego"], "Luis"))
print(ej.buscar(["Ana", "Luis", "Diego"], "Sofía"))
print(ej.min_max([7, -3, 12, 5]))
print(ej.min_max([]))
