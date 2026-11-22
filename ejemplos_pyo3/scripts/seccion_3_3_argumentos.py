"""Sección 3.3: argumentos opcionales y por nombre.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_3_argumentos.py
"""
import ejemplos_pyo3 as ej

print(ej.repetir("hola"))
print(ej.repetir("hola", 3))
print(ej.repetir("hola", veces=3, separador="-"))
print(ej.presentar("Sofía"))
print(ej.presentar("Sofía", titulo="Ing."))
print(ej.repetir.__text_signature__)
