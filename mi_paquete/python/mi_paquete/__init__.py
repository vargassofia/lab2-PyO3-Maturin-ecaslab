"""Paquete mixto: funciones de Rust + funciones de Python."""
import math

from .mi_paquete import suma_cuadrados


def norma(valores):
    """Norma euclidiana: usa la función de Rust y termina en Python."""
    return math.sqrt(suma_cuadrados(valores))


__all__ = ["suma_cuadrados", "norma"]
