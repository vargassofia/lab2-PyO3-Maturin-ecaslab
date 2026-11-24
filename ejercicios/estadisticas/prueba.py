"""Ejercicio 1 (sección 5.1): Estadísticas de calificaciones.

Ejecutar desde ejercicios/estadisticas:
    maturin develop
    python prueba.py
"""

import estadisticas as est

notas = [85, 92.5, 67, 74, 100, 58.5]

print(f"Promedio: {est.promedio(notas):.2f}")
print(f"Desviación: {est.desviacion_estandar(notas):.2f}")

for nota in [95, 70, 69.9]:
    print(f"{nota} -> {est.clasificar(nota)}")

r = est.resumen(notas)
print(f"Resumen: {r['cantidad']} notas, {r['aprobados']} aprobadas, "
      f"máx. {r['maxima']}, mín. {r['minima']}")
r = est.resumen(notas, nota_minima=90)
print("Aprobadas con mínimo 90:", r["aprobados"])

for funcion, argumento in [(est.promedio, []), (est.clasificar, 120)]:
    try:
        funcion(argumento)
    except ValueError as e:
        print("ValueError:", e)
