import pytest
import mi_paquete


def test_suma_cuadrados():
    assert mi_paquete.suma_cuadrados([3, 4]) == pytest.approx(25.0)


def test_norma():
    assert mi_paquete.norma([3, 4]) == pytest.approx(5.0)
