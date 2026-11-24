//! Proyecto (sección 6): regresión lineal acelerada con Rust.
//!
//! Este archivo es la plantilla de `maturin new -b pyo3 regresion_rs`. Aquí debe
//! escribir su implementación. La implementación de referencia está en el
//! Anexo A.3 de la guía.
//!
//! 6.3 Requisitos. Su proyecto debe cumplir lo siguiente:
//! 1. Crearse con `maturin new -b pyo3 regresion_rs` y compilarse con
//!    `maturin develop --release` sin advertencias.
//! 2. Exponer la clase `RegresionLineal(tasa_aprendizaje=0.01, epocas=1000)`. El
//!    constructor lanza ValueError si alguno de los dos valores no es positivo.
//! 3. Tener el método `entrenar(x, y)`, donde x es una lista de filas (listas de
//!    float) e y una lista de float. Debe validar los datos y lanzar ValueError si
//!    x está vacío, si x e y tienen distinta cantidad de filas o si las filas de x
//!    no tienen todas la misma cantidad de columnas. El entrenamiento implementa
//!    el descenso de gradiente de la sección 6.2 y se ejecuta sin el GIL
//!    (py.detach).
//! 4. Exponer como atributos de solo lectura pesos, sesgo, historial_perdida,
//!    tasa_aprendizaje y epocas.
//! 5. Tener el método `predecir(x)`, que retorne una lista de predicciones. Debe
//!    lanzar RuntimeError si el modelo no ha sido entrenado y ValueError si las
//!    filas no tienen la cantidad de columnas esperada.
//! 6. Exponer la función `error_cuadratico_medio(y_real, y_pred)`, que lance
//!    ValueError si las listas están vacías o tienen distinto tamaño.
//! 7. Implementar `__repr__`, que muestre los hiperparámetros e indique si el
//!    modelo ya fue entrenado.
//! 8. Incluir al menos cuatro pruebas con pytest en `tests/test_regresion.py`.
//! 9. No usar unwrap() ni expect() sobre datos que vienen de Python.
//!
//! 6.5 Pasos sugeridos y pistas:
//! 1. Cree el proyecto, copie los tres archivos de Python e implemente primero
//!    error_cuadratico_medio. Compílela y pruébela desde Python antes de seguir.
//! 2. Escriba el cálculo como funciones privadas de Rust sin PyO3:
//!    `predecir_fila(pesos, sesgo, fila) -> f64` y
//!    `descenso_gradiente(x, y, columnas, tasa, epocas) -> (Vec<f64>, f64, Vec<f64>)`.
//!    Separar la matemática de PyO3 facilita probarla y permite ejecutarla sin
//!    el GIL.
//! 3. Escriba `validar_datos(x, y) -> PyResult<usize>`, que retorne la cantidad
//!    de columnas o el error.
//! 4. Defina el struct con #[pyclass] y el constructor con #[new] y signature.
//! 5. Implemente entrenar, predecir y __repr__.
//! 6. Escriba las pruebas, compile con --release y ejecute
//!    `python -m pytest -v tests` y `python main.py`.
//!
//! Este es un posible esqueleto del método entrenar. Observe que los
//! hiperparámetros se copian en variables locales antes de soltar el GIL, de modo
//! que el cierre no toca el objeto de Python:
//!
//! ```ignore
//! fn entrenar(
//!     &mut self,
//!     py: Python<'_>,
//!     x: Vec<Vec<f64>>,
//!     y: Vec<f64>,
//! ) -> PyResult<()> {
//!     let columnas = validar_datos(&x, &y)?;
//!     let (tasa, epocas) = (self.tasa_aprendizaje, self.epocas);
//!     let (pesos, sesgo, historial) =
//!         py.detach(|| descenso_gradiente(&x, &y, columnas, tasa, epocas));
//!     // ... guardar los resultados en self
//!     Ok(())
//! }
//! ```
//!
//! Consejo: para la predicción de una fila,
//! `pesos.iter().zip(fila).map(|(w, x)| w * x).sum::<f64>()` calcula el producto
//! punto sin índices. Para acumular el gradiente, recorra
//! `grad_pesos.iter_mut().zip(fila)`.
//!
//! Cuidado: si la tasa de aprendizaje es demasiado grande, el descenso de
//! gradiente diverge. Con estos datos, una tasa de 0.1 lleva la pérdida de 56.1 a
//! 0.10 en 50 épocas, mientras que con 1.5 sube a 1.6 × 10³¹. Si su
//! historial_perdida crece o termina en inf o nan, reduzca la tasa.

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod regresion_rs {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
