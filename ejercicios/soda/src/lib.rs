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
//! El script de prueba es `prueba.py`. La solución está en el Anexo A.2 de la guía.

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod soda {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
