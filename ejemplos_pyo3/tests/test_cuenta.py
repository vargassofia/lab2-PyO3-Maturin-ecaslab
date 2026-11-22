"""Sección 4.4: pruebas automatizadas de la clase Cuenta (sección 3.5).

Ejecute desde la carpeta ejemplos_pyo3, después de `maturin develop`:
    python -m pytest -v tests
"""
import pytest

from ejemplos_pyo3 import Cuenta


def test_deposito_y_retiro():
    cuenta = Cuenta("Ana", 100.0)
    cuenta.depositar(50)
    cuenta.retirar(30)
    assert cuenta.saldo == pytest.approx(120.0)


def test_saldo_inicial_por_defecto():
    assert Cuenta("Luis").saldo == 0.0


def test_retiro_mayor_al_saldo_lanza_value_error():
    cuenta = Cuenta("Ana", 10.0)
    with pytest.raises(ValueError, match="saldo insuficiente"):
        cuenta.retirar(50)
