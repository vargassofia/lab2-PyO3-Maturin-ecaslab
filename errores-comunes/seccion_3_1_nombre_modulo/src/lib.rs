//! Error común de la sección 3.1: el nombre del módulo no coincide con [lib] name.
//!
//! En Cargo.toml, `[lib] name = "nombre_modulo"`, pero el mod se llama
//! `mis_ejemplos`. Compila sin errores, pero al importar en Python se obtiene:
//!   ImportError: dynamic module does not define module export function
//!   (PyInit_nombre_modulo)
//!
//! Corrección: usar el mismo nombre en el mod y en `[lib] name`
//! (`mod nombre_modulo`) y volver a ejecutar `maturin develop`.

use pyo3::prelude::*;

#[pymodule]
mod mis_ejemplos { // <- no coincide con [lib] name = "nombre_modulo"
    use pyo3::prelude::*;

    /// Devuelve un saludo personalizado.
    #[pyfunction]
    fn saludar(nombre: &str) -> String {
        format!("¡Hola, {nombre}! Este saludo viene de Rust.")
    }

    /// Suma dos números enteros.
    #[pyfunction]
    fn sumar(a: i64, b: i64) -> i64 {
        a + b
    }
}
