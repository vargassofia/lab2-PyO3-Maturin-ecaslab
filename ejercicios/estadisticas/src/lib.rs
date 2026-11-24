//! Ejercicio 1 (sección 5.1): Estadísticas de calificaciones.
//! Solución del Anexo A.1 del Laboratorio 2: PyO3 y Maturin en Rust.

use pyo3::prelude::*;

#[pymodule]
mod estadisticas {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    /// Error común a todas las funciones que reciben una lista vacía.
    fn verificar_no_vacia(notas: &[f64]) -> PyResult<()> {
        if notas.is_empty() {
            return Err(PyValueError::new_err("la lista de notas está vacía"));
        }
        Ok(())
    }

    #[pyfunction]
    fn promedio(notas: Vec<f64>) -> PyResult<f64> {
        verificar_no_vacia(&notas)?;
        let suma: f64 = notas.iter().sum();
        Ok(suma / notas.len() as f64)
    }

    /// Desviación estándar poblacional.
    #[pyfunction]
    fn desviacion_estandar(notas: Vec<f64>) -> PyResult<f64> {
        let media = promedio(notas.clone())?;
        let suma_cuadrados: f64 = notas.iter().map(|n| (n - media).powi(2)).sum();
        Ok((suma_cuadrados / notas.len() as f64).sqrt())
    }

    #[pyfunction]
    fn clasificar(nota: f64) -> PyResult<&'static str> {
        match nota {
            n if !(0.0..=100.0).contains(&n) => Err(PyValueError::new_err(format!(
                "nota fuera de rango: {n}"
            ))),
            n if n >= 90.0 => Ok("excelente"),
            n if n >= 70.0 => Ok("aprobado"),
            _ => Ok("reprobado"),
        }
    }

    #[pyfunction]
    #[pyo3(signature = (notas, *, nota_minima = 70.0))]
    fn resumen<'py>(
        py: Python<'py>,
        notas: Vec<f64>,
        nota_minima: f64,
    ) -> PyResult<Bound<'py, PyDict>> {
        verificar_no_vacia(&notas)?;
        let maxima = notas.iter().copied().fold(f64::MIN, f64::max);
        let minima = notas.iter().copied().fold(f64::MAX, f64::min);
        let aprobados = notas.iter().filter(|&&n| n >= nota_minima).count();

        let datos = PyDict::new(py);
        datos.set_item("cantidad", notas.len())?;
        datos.set_item("promedio", promedio(notas.clone())?)?;
        datos.set_item("desviacion", desviacion_estandar(notas)?)?;
        datos.set_item("maxima", maxima)?;
        datos.set_item("minima", minima)?;
        datos.set_item("aprobados", aprobados)?;
        Ok(datos)
    }
}
