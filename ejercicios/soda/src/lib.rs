//! Ejercicio 2 (sección 5.2): Inventario de la soda, ahora desde Python.
//! Solución del Anexo A.2 del Laboratorio 2: PyO3 y Maturin en Rust.

use pyo3::prelude::*;

#[pymodule]
mod soda {
    use pyo3::exceptions::{PyKeyError, PyValueError};
    use pyo3::prelude::*;
    use std::collections::HashMap;

    /// Quita espacios sobrantes y pasa a minúsculas: «Café » == «café».
    fn normalizar(producto: &str) -> String {
        producto.trim().to_lowercase()
    }

    #[pyclass]
    struct Inventario {
        productos: HashMap<String, u32>,
    }

    #[pymethods]
    impl Inventario {
        #[new]
        fn new() -> Self {
            Inventario { productos: HashMap::new() }
        }

        #[pyo3(signature = (producto, cantidad = 1))]
        fn agregar(&mut self, producto: &str, cantidad: u32) -> PyResult<()> {
            let nombre = normalizar(producto);
            if nombre.is_empty() {
                return Err(PyValueError::new_err(
                    "el nombre del producto está vacío",
                ));
            }
            if cantidad == 0 {
                return Err(PyValueError::new_err(
                    "la cantidad debe ser mayor que cero",
                ));
            }
            *self.productos.entry(nombre).or_insert(0) += cantidad;
            Ok(())
        }

        /// Descuenta unidades y devuelve cuántas quedan.
        fn vender(&mut self, producto: &str, cantidad: u32) -> PyResult<u32> {
            let nombre = normalizar(producto);
            let disponibles = self.productos.get_mut(&nombre).ok_or_else(|| {
                PyKeyError::new_err(format!("no existe el producto '{nombre}'"))
            })?;
            if cantidad > *disponibles {
                return Err(PyValueError::new_err(format!(
                    "unidades insuficientes de '{nombre}': hay {disponibles}"
                )));
            }
            *disponibles -= cantidad;
            Ok(*disponibles)
        }

        fn existencias(&self, producto: &str) -> u32 {
            self.productos.get(&normalizar(producto)).copied().unwrap_or(0)
        }

        /// Lista de tuplas (producto, cantidad) en orden alfabético.
        fn productos(&self) -> Vec<(String, u32)> {
            let mut lista: Vec<(String, u32)> =
                self.productos.iter().map(|(p, c)| (p.clone(), *c)).collect();
            lista.sort();
            lista
        }

        fn __len__(&self) -> usize {
            self.productos.len()
        }

        fn __contains__(&self, producto: &str) -> bool {
            self.productos.contains_key(&normalizar(producto))
        }

        fn __repr__(&self) -> String {
            let unidades: u32 = self.productos.values().sum();
            let productos = self.productos.len();
            format!("Inventario({productos} productos, {unidades} unidades)")
        }
    }
}
