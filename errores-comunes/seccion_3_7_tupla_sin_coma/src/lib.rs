//! Error común de la sección 3.7: olvidar la coma de una tupla de un elemento (E0277).
//!
//! En Rust, igual que en Python, (valor) no es una tupla: los paréntesis solo
//! agrupan. El compilador da una advertencia por paréntesis innecesarios y un
//! error[E0277] porque un Bound<PyAny> no sirve como lista de argumentos.
//!
//! Corrección: escribir la tupla con la coma final: funcion.call1((valor,)).
//! Para llamar sin argumentos use call0(), y para pasar argumentos por nombre,
//! call(args, Some(&kwargs)).

use pyo3::prelude::*;

#[pymodule]
mod tupla_sin_coma {
    use pyo3::prelude::*;
    use pyo3::types::PyList;

    /// Recibe una función de Python y la aplica a cada elemento.
    #[pyfunction]
    fn aplicar<'py>(
        funcion: &Bound<'py, PyAny>,
        valores: &Bound<'py, PyList>,
    ) -> PyResult<Bound<'py, PyList>> {
        let resultado = PyList::empty(valores.py());
        for valor in valores.iter() {
            // Corrección: funcion.call1((valor,))
            resultado.append(funcion.call1((valor))?)?; // <- E0277: (valor) no es una tupla
        }
        Ok(resultado)
    }
}
