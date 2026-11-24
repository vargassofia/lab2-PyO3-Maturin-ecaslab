"""Error común de la sección 3.3: pasar por posición un argumento que solo acepta nombre.

Requiere el módulo de ejemplos_pyo3 instalado (cd ejemplos_pyo3 && maturin develop).
Ejecutar desde errores-comunes:
    python seccion_3_3_solo_por_nombre.py

En la firma de repetir, separador está después del *, así que solo se puede
pasar por nombre. La función acepta como máximo dos argumentos posicionales
(texto y veces) y recibió tres.

Corrección: ej.repetir("hola", 3, separador="-").
"""

import ejemplos_pyo3 as ej

try:
    print(ej.repetir("hola", 3, "-"))
except TypeError as e:
    print("TypeError:", e)

# Versión corregida.
print(ej.repetir("hola", 3, separador="-"))
