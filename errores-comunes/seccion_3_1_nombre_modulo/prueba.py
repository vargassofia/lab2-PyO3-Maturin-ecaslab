"""Error común de la sección 3.1: el nombre del módulo no coincide con [lib] name.

Ejecutar desde errores-comunes/seccion_3_1_nombre_modulo:
    maturin develop
    python prueba.py

Resultado: ImportError, porque Python busca PyInit_nombre_modulo y la
biblioteca exporta PyInit_mis_ejemplos.
"""

import nombre_modulo

print(nombre_modulo.saludar("Ana"))
