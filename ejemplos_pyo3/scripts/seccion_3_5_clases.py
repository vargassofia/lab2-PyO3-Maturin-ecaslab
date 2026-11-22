"""Sección 3.5: clases con #[pyclass] y #[pymethods].

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_5_clases.py
"""
from ejemplos_pyo3 import Cuenta

cuenta = Cuenta("Ana", 500.0)
cuenta.depositar(250)
cuenta.retirar(100)
print(cuenta)
print("Saldo:", cuenta.saldo)

try:
    cuenta.retirar(1000)
except ValueError as e:
    print("Error:", e)

try:
    cuenta.saldo = 1_000_000
except AttributeError as e:
    print("Error:", e)
