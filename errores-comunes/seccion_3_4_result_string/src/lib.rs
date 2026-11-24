//! Error común de la sección 3.4: retornar Result<T, String> (E0277).
//!
//! PyO3 acepta funciones que retornan Result<T, E> solo si E se puede
//! convertir en un PyErr. Con un String no sabe qué excepción lanzar, así que
//! `maturin develop` falla con error[E0277].
//!
//! Corrección: retornar PyResult<u8> y convertir el error con
//! `map_err(|_| PyValueError::new_err(...))`:
//!
//!     use pyo3::exceptions::PyValueError;
//!
//!     #[pyfunction]
//!     fn parsear_edad(texto: &str) -> PyResult<u8> {
//!         let edad: u8 = texto.trim().parse().map_err(|_| {
//!             PyValueError::new_err(format!("'{texto}' no es una edad válida"))
//!         })?;
//!         Ok(edad)
//!     }

use pyo3::prelude::*;

#[pymodule]
mod result_string {
    use pyo3::prelude::*;

    #[pyfunction]
    fn parsear_edad(texto: &str) -> Result<u8, String> { // <- E0277: String no se convierte en PyErr
        texto
            .trim()
            .parse::<u8>()
            .map_err(|_| format!("'{texto}' no es una edad válida"))
    }
}
