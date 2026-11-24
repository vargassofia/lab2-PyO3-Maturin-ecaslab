use pyo3::prelude::*;

/// Parte nativa del paquete: el código que necesita velocidad.
#[pymodule]
mod mi_paquete {
    use pyo3::prelude::*;

    #[pyfunction]
    fn suma_cuadrados(valores: Vec<f64>) -> f64 {
        valores.iter().map(|v| v * v).sum()
    }
}
