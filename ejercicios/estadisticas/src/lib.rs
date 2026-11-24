//! Ejercicio 1 (sección 5.1): Estadísticas de calificaciones.
//! Temas que integra: #[pyfunction], conversión de tipos, signature, PyResult y PyDict.
//! Dificultad: fácil.
//!
//! Enunciado: cree un módulo `estadisticas` que calcule estadísticas de un grupo
//! de notas. Debe:
//! 1. Crear el proyecto con `maturin new -b pyo3 estadisticas`.
//! 2. Implementar `promedio(notas: Vec<f64>) -> PyResult<f64>`, que lance
//!    ValueError con el mensaje `la lista de notas está vacía` si no hay notas.
//! 3. Implementar `desviacion_estandar(notas)`, que calcule la desviación estándar
//!    poblacional (la raíz cuadrada del promedio de los cuadrados de las
//!    diferencias con la media), con el mismo error.
//! 4. Implementar `clasificar(nota: f64) -> PyResult<&'static str>`, que retorne
//!    "excelente" si la nota es de 90 o más, "aprobado" si es de 70 o más y
//!    "reprobado" en otro caso. Si la nota no está entre 0 y 100, debe lanzar
//!    ValueError con el mensaje `nota fuera de rango: <nota>`.
//! 5. Implementar `resumen(notas, *, nota_minima=70.0)`, que retorne un dict con
//!    las claves cantidad, promedio, desviacion, maxima, minima y aprobados
//!    (cantidad de notas mayores o iguales a nota_minima). nota_minima solo se
//!    puede pasar por nombre.
//!
//! Pistas:
//! - Para no repetir la validación, escriba una función privada de Rust (sin
//!   #[pyfunction]) como `fn verificar_no_vacia(notas: &[f64]) -> PyResult<()>` y
//!   llámela con `?`.
//! - f64 no implementa Ord, así que .max() no funciona con un iterador de f64.
//!   Use `notas.iter().copied().fold(f64::MIN, f64::max)`.
//! - Para elevar al cuadrado y sacar la raíz, use `(n - media).powi(2)` y `.sqrt()`.
//! - Para retornar un dict con valores de distintos tipos, cree un PyDict como en
//!   la sección 3.7.
//! - Un match con guardas (`n if n >= 90.0 => ...`) expresa bien los rangos de
//!   clasificar.
//!
//! El script de prueba es `prueba.py`. Solución: Anexo A.1 de la guía (abajo).

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
