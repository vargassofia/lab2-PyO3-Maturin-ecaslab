//! Ejercicio 2 (sección 5.2): Inventario de la soda, ahora desde Python.
//! Temas que integra: #[pyclass], #[pymethods], &mut self, HashMap, excepciones y
//! métodos especiales. Dificultad: intermedia.
//!
//! Enunciado: en el Laboratorio 1, usted procesó el inventario de la soda de la
//! escuela en un programa de Rust. Ahora el inventario será una clase que se usa
//! desde Python. Cree un módulo `soda` con la clase `Inventario`, que debe:
//! 1. Guardar los productos en un `HashMap<String, u32>`. Los nombres se
//!    normalizan sin espacios sobrantes y en minúsculas, de modo que «Café » y
//!    «café» sean el mismo producto.
//! 2. Crearse vacía con `Inventario()`.
//! 3. Tener el método `agregar(producto, cantidad=1)`, que sume unidades al
//!    producto. Debe lanzar ValueError si el nombre queda vacío
//!    (`el nombre del producto está vacío`) o si la cantidad es cero
//!    (`la cantidad debe ser mayor que cero`).
//! 4. Tener el método `vender(producto, cantidad)`, que descuente unidades y
//!    retorne cuántas quedan. Debe lanzar KeyError si el producto no existe
//!    (`no existe el producto '<nombre>'`) y ValueError si no hay suficientes
//!    unidades (`unidades insuficientes de '<nombre>': hay <n>`).
//! 5. Tener el método `existencias(producto)`, que retorne las unidades
//!    disponibles o 0 si el producto no existe.
//! 6. Tener el método `productos()`, que retorne una lista de tuplas
//!    (producto, cantidad) en orden alfabético.
//! 7. Implementar `__len__` (cantidad de productos distintos), `__contains__`
//!    (que también normalice el nombre) y `__repr__` con el formato
//!    `Inventario(<n> productos, <m> unidades)`.
//!
//! Pistas:
//! - Escriba una función privada `fn normalizar(producto: &str) -> String` con
//!   trim() y to_lowercase(), y úsela en todos los métodos.
//! - Para acumular, use `*self.productos.entry(nombre).or_insert(0) += cantidad;`,
//!   como en el Laboratorio 1.
//! - `self.productos.get_mut(&nombre)` retorna un `Option<&mut u32>`; con
//!   `.ok_or_else(|| PyKeyError::new_err(...))?` lo convierte en un error de Python.
//! - Al ordenar un `Vec<(String, u32)>` con sort(), las tuplas se comparan
//!   primero por el nombre.
//! - Como cantidad es u32, una cantidad negativa enviada desde Python se rechaza
//!   automáticamente con OverflowError. Observe también que Python muestra el
//!   mensaje de KeyError entre comillas.
//!
//! El script de prueba es `prueba.py`. Solución: Anexo A.2 de la guía (abajo).

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
