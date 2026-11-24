import time

import datos
import regresion_py
from regresion_rs import RegresionLineal, error_cuadratico_medio

x, y = datos.generar(5_000)
x_ent, x_pru, y_ent, y_pru = datos.dividir(x, y)
print(f"Entrenamiento: {len(x_ent)} filas | Prueba: {len(x_pru)} filas")

# --- Modelo en Rust ---
modelo = RegresionLineal(tasa_aprendizaje=0.1, epocas=300)
inicio = time.perf_counter()
modelo.entrenar(x_ent, y_ent)
t_rust = time.perf_counter() - inicio

print(modelo)
print("Pesos aprendidos:", [round(w, 3) for w in modelo.pesos])
print("Sesgo aprendido: ", round(modelo.sesgo, 3))
print("Pesos reales:    ", datos.PESOS_REALES, "| sesgo real:", datos.SESGO_REAL)
print(f"Pérdida: época 1 = {modelo.historial_perdida[0]:.3f}, "
      f"época {modelo.epocas} = {modelo.historial_perdida[-1]:.4f}")

predicciones = modelo.predecir(x_pru)
print(f"ECM en prueba: {error_cuadratico_medio(y_pru, predicciones):.4f}")

# --- Misma tarea en Python puro ---
inicio = time.perf_counter()
regresion_py.entrenar(x_ent, y_ent, tasa=0.1, epocas=300)
t_python = time.perf_counter() - inicio

print(f"Tiempo Rust:   {t_rust:.3f} s")
print(f"Tiempo Python: {t_python:.3f} s ({t_python / t_rust:.0f} veces más lento)")
