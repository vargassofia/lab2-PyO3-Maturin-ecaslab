"""Ejercicio 2 (sección 5.2): Inventario de la soda, ahora desde Python.

Ejecutar desde ejercicios/soda:
    maturin develop
    python prueba.py
"""

from soda import Inventario

inv = Inventario()
inv.agregar("empanadas", 12)
inv.agregar("Café ", 30)
inv.agregar("empanadas", 8)
inv.agregar("jugo")
print(inv)
print("Productos distintos:", len(inv))
print("¿Hay café?", "CAFÉ" in inv)

print("Quedan", inv.vender("café", 5), "cafés")
for nombre, cantidad in inv.productos():
    print(f"  {nombre:<10} {cantidad:>3}")

for producto, cantidad in [("gallo pinto", 1), ("jugo", 3)]:
    try:
        inv.vender(producto, cantidad)
    except KeyError as e:
        print("KeyError:", e)
    except ValueError as e:
        print("ValueError:", e)

try:
    inv.agregar("   ", 4)
except ValueError as e:
    print("ValueError:", e)
