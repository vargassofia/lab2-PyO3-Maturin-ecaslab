"""Error común de la sección 3.2: pasar un tipo incorrecto (TypeError).

Requiere el módulo de ejemplos_pyo3 instalado (cd ejemplos_pyo3 && maturin develop).
Ejecutar desde errores-comunes:
    python seccion_3_2_tipo_incorrecto.py

PyO3 intenta convertir cada elemento de la lista en un i64. El texto "3" no se
puede interpretar como entero, así que lanza TypeError antes de ejecutar el
cuerpo de la función.

Corrección: convertir los datos en Python antes de llamar, por ejemplo con
int("3"): ej.min_max([7, int("3"), 12]).
"""

import ejemplos_pyo3 as ej

try:
    print(ej.min_max([7, "3", 12]))
except TypeError as e:
    print("TypeError:", e)

# Pruébelo usted: el número no cabe en un i64.
try:
    print(ej.min_max([2**70]))
except OverflowError as e:
    print("OverflowError:", e)

# Versión corregida.
print(ej.min_max([7, int("3"), 12]))
