//! Proyecto (sección 6): regresión lineal acelerada con Rust.
//!
//! Implementación de referencia del Anexo A.3 (abajo). Compila con
//! `maturin develop --release` sin advertencias, produce la salida de la
//! sección 6.4 y pasa las cinco pruebas de `tests/test_regresion.py`.
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

#[pymodule]
mod regresion_rs {
    use pyo3::exceptions::{PyRuntimeError, PyValueError};
    use pyo3::prelude::*;

    /// Revisa que X e y sean compatibles y devuelve la cantidad de columnas de X.
    fn validar_datos(x: &[Vec<f64>], y: &[f64]) -> PyResult<usize> {
        if x.is_empty() {
            return Err(PyValueError::new_err("X no puede estar vacío"));
        }
        if x.len() != y.len() {
            return Err(PyValueError::new_err(format!(
                "X tiene {} filas, pero y tiene {} valores",
                x.len(),
                y.len()
            )));
        }
        let columnas = x[0].len();
        if columnas == 0 || x.iter().any(|fila| fila.len() != columnas) {
            return Err(PyValueError::new_err(
                "todas las filas de X deben tener la misma cantidad de columnas",
            ));
        }
        Ok(columnas)
    }

    /// ŷ = w · x + b para una sola fila.
    fn predecir_fila(pesos: &[f64], sesgo: f64, fila: &[f64]) -> f64 {
        pesos.iter().zip(fila).map(|(w, x)| w * x).sum::<f64>() + sesgo
    }

    /// Descenso de gradiente por lotes: devuelve (pesos, sesgo, historial).
    fn descenso_gradiente(
        x: &[Vec<f64>],
        y: &[f64],
        columnas: usize,
        tasa: f64,
        epocas: usize,
    ) -> (Vec<f64>, f64, Vec<f64>) {
        let n = x.len() as f64;
        let mut pesos = vec![0.0; columnas];
        let mut sesgo = 0.0;
        let mut historial = Vec::with_capacity(epocas);

        for _ in 0..epocas {
            let mut grad_pesos = vec![0.0; columnas];
            let mut grad_sesgo = 0.0;
            let mut perdida = 0.0;

            for (fila, &objetivo) in x.iter().zip(y) {
                let error = predecir_fila(&pesos, sesgo, fila) - objetivo;
                perdida += error * error;
                for (g, &xj) in grad_pesos.iter_mut().zip(fila) {
                    *g += error * xj;
                }
                grad_sesgo += error;
            }

            for (w, g) in pesos.iter_mut().zip(&grad_pesos) {
                *w -= tasa * 2.0 * g / n;
            }
            sesgo -= tasa * 2.0 * grad_sesgo / n;
            historial.push(perdida / n);
        }
        (pesos, sesgo, historial)
    }

    /// Error cuadrático medio entre dos listas del mismo tamaño.
    #[pyfunction]
    fn error_cuadratico_medio(y_real: Vec<f64>, y_pred: Vec<f64>) -> PyResult<f64> {
        if y_real.is_empty() || y_real.len() != y_pred.len() {
            return Err(PyValueError::new_err(
                "las listas deben tener el mismo tamaño y no estar vacías",
            ));
        }
        let suma: f64 = y_real
            .iter()
            .zip(&y_pred)
            .map(|(r, p)| (r - p).powi(2))
            .sum();
        Ok(suma / y_real.len() as f64)
    }

    #[pyclass]
    struct RegresionLineal {
        #[pyo3(get)]
        tasa_aprendizaje: f64,
        #[pyo3(get)]
        epocas: usize,
        #[pyo3(get)]
        pesos: Vec<f64>,
        #[pyo3(get)]
        sesgo: f64,
        #[pyo3(get)]
        historial_perdida: Vec<f64>,
    }

    #[pymethods]
    impl RegresionLineal {
        #[new]
        #[pyo3(signature = (tasa_aprendizaje = 0.01, epocas = 1000))]
        fn new(tasa_aprendizaje: f64, epocas: usize) -> PyResult<Self> {
            if tasa_aprendizaje <= 0.0 || epocas == 0 {
                return Err(PyValueError::new_err(
                    "la tasa de aprendizaje y las épocas deben ser positivas",
                ));
            }
            Ok(RegresionLineal {
                tasa_aprendizaje,
                epocas,
                pesos: Vec::new(),
                sesgo: 0.0,
                historial_perdida: Vec::new(),
            })
        }

        /// Ajusta el modelo. El cálculo se hace sin el GIL.
        fn entrenar(
            &mut self,
            py: Python<'_>,
            x: Vec<Vec<f64>>,
            y: Vec<f64>,
        ) -> PyResult<()> {
            let columnas = validar_datos(&x, &y)?;
            let (tasa, epocas) = (self.tasa_aprendizaje, self.epocas);
            let (pesos, sesgo, historial) =
                py.detach(|| descenso_gradiente(&x, &y, columnas, tasa, epocas));
            self.pesos = pesos;
            self.sesgo = sesgo;
            self.historial_perdida = historial;
            Ok(())
        }

        fn predecir(&self, x: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
            if self.pesos.is_empty() {
                return Err(PyRuntimeError::new_err(
                    "el modelo no ha sido entrenado; llame primero a entrenar()",
                ));
            }
            if x.iter().any(|fila| fila.len() != self.pesos.len()) {
                return Err(PyValueError::new_err(format!(
                    "cada fila debe tener {} columnas",
                    self.pesos.len()
                )));
            }
            let predicciones = x
                .iter()
                .map(|fila| predecir_fila(&self.pesos, self.sesgo, fila))
                .collect();
            Ok(predicciones)
        }

        fn __repr__(&self) -> String {
            format!(
                "RegresionLineal(tasa_aprendizaje={}, epocas={}, entrenado={})",
                self.tasa_aprendizaje,
                self.epocas,
                if self.pesos.is_empty() { "False" } else { "True" }
            )
        }
    }
}
