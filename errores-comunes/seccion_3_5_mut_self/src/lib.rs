//! Error común de la sección 3.5: modificar el objeto desde &self (E0594).
//!
//! Con &self el método solo puede leer el objeto, y `self.saldo += monto`
//! intenta modificarlo, así que `maturin develop` falla con error[E0594].
//!
//! Corrección: declarar el método con &mut self, como sugiere `help:`.
//! Desde Python la llamada no cambia: cuenta.depositar(250).

use pyo3::prelude::*;

#[pymodule]
mod mut_self {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    /// Una cuenta bancaria sencilla.
    #[pyclass]
    struct Cuenta {
        #[pyo3(get)] // solo lectura desde Python
        titular: String,
        #[pyo3(get)]
        saldo: f64,
    }

    #[pymethods]
    impl Cuenta {
        /// Constructor: se llama con `Cuenta("Ana", 500.0)`.
        #[new]
        #[pyo3(signature = (titular, saldo_inicial = 0.0))]
        fn new(titular: String, saldo_inicial: f64) -> PyResult<Self> {
            if saldo_inicial < 0.0 {
                return Err(PyValueError::new_err(
                    "el saldo inicial no puede ser negativo",
                ));
            }
            Ok(Cuenta { titular, saldo: saldo_inicial })
        }

        // Corrección: fn depositar(&mut self, monto: f64) -> PyResult<()>
        fn depositar(&self, monto: f64) -> PyResult<()> {
            if monto <= 0.0 {
                return Err(PyValueError::new_err("el monto debe ser positivo"));
            }
            self.saldo += monto; // <- E0594: no se puede asignar a través de &self
            Ok(())
        }

        fn retirar(&mut self, monto: f64) -> PyResult<()> {
            if monto > self.saldo {
                return Err(PyValueError::new_err(format!(
                    "saldo insuficiente: hay {:.2} y se pidieron {:.2}",
                    self.saldo, monto
                )));
            }
            self.saldo -= monto;
            Ok(())
        }

        /// Texto que muestra Python con print() o en la consola.
        fn __repr__(&self) -> String {
            format!("Cuenta(titular='{}', saldo={:.2})", self.titular, self.saldo)
        }
    }
}
