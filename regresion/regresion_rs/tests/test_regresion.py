import pytest

from regresion_rs import RegresionLineal, error_cuadratico_medio


def test_aprende_una_recta():
    # y = 2x + 1, sin ruido
    x = [[i / 10] for i in range(20)]
    y = [2 * fila[0] + 1 for fila in x]
    modelo = RegresionLineal(tasa_aprendizaje=0.1, epocas=2000)
    modelo.entrenar(x, y)
    assert modelo.pesos[0] == pytest.approx(2, abs=1e-3)
    assert modelo.sesgo == pytest.approx(1, abs=1e-3)


def test_la_perdida_disminuye():
    x = [[1.0, 2.0], [2.0, 0.5], [3.0, 1.0], [0.5, 3.0]]
    y = [5.0, 4.0, 7.0, 6.0]
    modelo = RegresionLineal(tasa_aprendizaje=0.01, epocas=50)
    modelo.entrenar(x, y)
    assert len(modelo.historial_perdida) == 50
    assert modelo.historial_perdida[-1] < modelo.historial_perdida[0]


def test_predecir_sin_entrenar_lanza_runtime_error():
    with pytest.raises(RuntimeError):
        RegresionLineal().predecir([[1.0]])


def test_tamanos_incompatibles_lanzan_value_error():
    with pytest.raises(ValueError, match="filas"):
        RegresionLineal().entrenar([[1.0], [2.0]], [1.0])


def test_error_cuadratico_medio():
    resultado = error_cuadratico_medio([1.0, 2.0, 3.0], [1.0, 2.0, 5.0])
    assert resultado == pytest.approx(4 / 3)
