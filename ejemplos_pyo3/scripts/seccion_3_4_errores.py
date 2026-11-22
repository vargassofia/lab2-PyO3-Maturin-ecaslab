"""Sección 3.4: errores y excepciones.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_4_errores.py
"""
import ejemplos_pyo3 as ej

print(ej.dividir(10, 4))
for texto in ["21", "veinte", "300"]:
    try:
        print("Edad:", ej.parsear_edad(texto))
    except ValueError as e:
        print("ValueError:", e)

try:
    ej.dividir(1, 0)
except ZeroDivisionError as e:
    print("ZeroDivisionError:", e)

try:
    ej.validar_nota(105)
except ej.NotaInvalidaError as e:
    print(type(e).__name__ + ":", e)
