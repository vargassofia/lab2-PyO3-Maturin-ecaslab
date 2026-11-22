"""Sección 3.7: el GIL, el token Python<'py> y Bound<'py, T>.

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python scripts/seccion_3_7_gil_bound.py
"""
import ejemplos_pyo3 as ej

print(ej.ficha("Luis", 20))
print(ej.aplicar(lambda x: x ** 2, [1, 2, 3, 4]))
print(ej.aplicar(str.upper, ["rust", "python"]))
print(ej.raiz_con_math(2.0))
