//! Ejemplos de las secciones 2 y 3 del Laboratorio 2: PyO3 y Maturin en Rust.
//!
//! En la guía, cada sección reemplaza todo `src/lib.rs`. Aquí se reúnen todos
//! los ejemplos en un solo módulo para compilarlos una única vez. Cada bloque
//! está marcado con la sección correspondiente, y los scripts de `scripts/`
//! prueban cada una por separado.

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// 3.4 Excepción propia: en Python será `ejemplos_pyo3.NotaInvalidaError`.
pyo3::create_exception!(ejemplos_pyo3, NotaInvalidaError, PyException);

/// Módulo de Python escrito en Rust.
/// Su nombre debe coincidir con `[lib] name` de Cargo.toml.
#[pymodule]
mod ejemplos_pyo3 {
    use pyo3::exceptions::{PyValueError, PyZeroDivisionError};
    use pyo3::prelude::*;
    use pyo3::types::{PyDict, PyList};
    use std::collections::HashMap;

    // =====================================================================
    // 2.4 Proyecto generado por `maturin new`
    // =====================================================================

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    // =====================================================================
    // 3.1 Anatomía de un módulo
    // =====================================================================

    /// Constante visible desde Python como `ejemplos_pyo3.VERSION`.
    #[pymodule_export]
    const VERSION: &str = "1.0";

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

    // =====================================================================
    // 3.2 Conversión de tipos
    // =====================================================================

    /// list[float] -> float
    #[pyfunction]
    fn promedio(valores: Vec<f64>) -> f64 {
        let suma: f64 = valores.iter().sum();
        suma / valores.len() as f64
    }

    /// str -> dict[str, int]: cuántas veces aparece cada letra.
    #[pyfunction]
    fn contar_letras(texto: &str) -> HashMap<char, usize> {
        let mut conteo = HashMap::new();
        for c in texto.chars().filter(|c| c.is_alphabetic()) {
            *conteo.entry(c).or_insert(0) += 1;
        }
        conteo
    }

    /// Option<usize> -> int o None
    #[pyfunction]
    fn buscar(nombres: Vec<String>, objetivo: &str) -> Option<usize> {
        nombres.iter().position(|n| n == objetivo)
    }

    /// Una tupla de Rust se convierte en una tupla de Python.
    #[pyfunction]
    fn min_max(valores: Vec<i64>) -> Option<(i64, i64)> {
        let min = *valores.iter().min()?;
        let max = *valores.iter().max()?;
        Some((min, max))
    }

    // =====================================================================
    // 3.3 Argumentos opcionales y por nombre
    // =====================================================================

    /// Repite un texto. `veces` es opcional y `separador`
    /// solo se puede pasar por nombre (va después de `*`).
    #[pyfunction]
    #[pyo3(signature = (texto, veces = 2, *, separador = " "))]
    fn repetir(texto: &str, veces: usize, separador: &str) -> String {
        vec![texto; veces].join(separador)
    }

    /// Un parámetro `Option` con valor por defecto `None`.
    #[pyfunction]
    #[pyo3(signature = (nombre, titulo = None))]
    fn presentar(nombre: &str, titulo: Option<&str>) -> String {
        match titulo {
            Some(t) => format!("{t} {nombre}"),
            None => nombre.to_string(),
        }
    }

    // =====================================================================
    // 3.4 Errores y excepciones
    // =====================================================================

    #[pymodule_export]
    use super::NotaInvalidaError;

    #[pyfunction]
    fn dividir(a: f64, b: f64) -> PyResult<f64> {
        if b == 0.0 {
            return Err(PyZeroDivisionError::new_err(
                "no se puede dividir entre cero",
            ));
        }
        Ok(a / b)
    }

    /// Convierte el error de Rust (ParseIntError) en un ValueError.
    #[pyfunction]
    fn parsear_edad(texto: &str) -> PyResult<u8> {
        let edad: u8 = texto.trim().parse().map_err(|_| {
            PyValueError::new_err(format!("'{texto}' no es una edad válida"))
        })?;
        Ok(edad)
    }

    #[pyfunction]
    fn validar_nota(nota: f64) -> PyResult<f64> {
        if !(0.0..=100.0).contains(&nota) {
            return Err(NotaInvalidaError::new_err(format!(
                "la nota {nota} está fuera del rango 0-100"
            )));
        }
        Ok(nota)
    }

    // =====================================================================
    // 3.5 Clases con #[pyclass] y #[pymethods]
    // =====================================================================

    /// Una cuenta bancaria sencilla.
    #[pyclass]
    struct Cuenta {
        #[pyo3(get)] // solo lectura desde Python
        titular: String,
        #[pyo3(get)]
        saldo: f64,
    }

    #[pymethods]
    impl Cuenta {
        /// Constructor: se llama con `Cuenta("Ana", 500.0)`.
        #[new]
        #[pyo3(signature = (titular, saldo_inicial = 0.0))]
        fn new(titular: String, saldo_inicial: f64) -> PyResult<Self> {
            if saldo_inicial < 0.0 {
                return Err(PyValueError::new_err(
                    "el saldo inicial no puede ser negativo",
                ));
            }
            Ok(Cuenta { titular, saldo: saldo_inicial })
        }

        /// `&mut self` porque el método modifica el objeto.
        fn depositar(&mut self, monto: f64) -> PyResult<()> {
            if monto <= 0.0 {
                return Err(PyValueError::new_err("el monto debe ser positivo"));
            }
            self.saldo += monto;
            Ok(())
        }

        fn retirar(&mut self, monto: f64) -> PyResult<()> {
            if monto > self.saldo {
                return Err(PyValueError::new_err(format!(
                    "saldo insuficiente: hay {:.2} y se pidieron {:.2}",
                    self.saldo, monto
                )));
            }
            self.saldo -= monto;
            Ok(())
        }

        /// Texto que muestra Python con print() o en la consola.
        fn __repr__(&self) -> String {
            format!("Cuenta(titular='{}', saldo={:.2})", self.titular, self.saldo)
        }
    }

    // =====================================================================
    // 3.6 Métodos especiales y propiedades
    // =====================================================================

    /// Vector en el plano con operadores de Python.
    #[pyclass(eq)]
    #[derive(PartialEq)]
    struct Vector2D {
        #[pyo3(get)]
        x: f64,
        #[pyo3(get)]
        y: f64,
    }

    #[pymethods]
    impl Vector2D {
        #[new]
        fn new(x: f64, y: f64) -> Self {
            Vector2D { x, y }
        }

        /// Método estático: `Vector2D.cero()`.
        #[staticmethod]
        fn cero() -> Self {
            Vector2D { x: 0.0, y: 0.0 }
        }

        /// Propiedad calculada: se usa como `v.magnitud`, sin paréntesis.
        #[getter]
        fn magnitud(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        /// v1 + v2
        fn __add__(&self, otro: PyRef<'_, Self>) -> Self {
            Vector2D { x: self.x + otro.x, y: self.y + otro.y }
        }

        /// v * 2.0
        fn __mul__(&self, escalar: f64) -> Self {
            Vector2D { x: self.x * escalar, y: self.y * escalar }
        }

        /// 2.0 * v. Solución del "Pruébelo usted" de la sección 3.6:
        /// reutiliza __mul__ porque el producto por un escalar es conmutativo.
        fn __rmul__(&self, escalar: f64) -> Self {
            self.__mul__(escalar)
        }

        /// -v
        fn __neg__(&self) -> Self {
            Vector2D { x: -self.x, y: -self.y }
        }

        fn __repr__(&self) -> String {
            format!("Vector2D({}, {})", self.x, self.y)
        }
    }

    // =====================================================================
    // 3.7 El GIL, el token Python<'py> y Bound<'py, T>
    // =====================================================================

    /// Construye un diccionario de Python directamente.
    /// `py` es el token que demuestra que el hilo está conectado al intérprete.
    #[pyfunction]
    fn ficha<'py>(
        py: Python<'py>,
        nombre: &str,
        edad: u32,
    ) -> PyResult<Bound<'py, PyDict>> {
        let ficha = PyDict::new(py);
        ficha.set_item("nombre", nombre)?;
        ficha.set_item("edad", edad)?;
        ficha.set_item("mayor_de_edad", edad >= 18)?;
        Ok(ficha)
    }

    /// Recibe una función de Python y la aplica a cada elemento.
    #[pyfunction]
    fn aplicar<'py>(
        funcion: &Bound<'py, PyAny>,
        valores: &Bound<'py, PyList>,
    ) -> PyResult<Bound<'py, PyList>> {
        let resultado = PyList::empty(valores.py());
        for valor in valores.iter() {
            // call1 recibe una tupla con los argumentos: (valor,)
            resultado.append(funcion.call1((valor,))?)?;
        }
        Ok(resultado)
    }

    /// Importa y usa un módulo de Python desde Rust.
    #[pyfunction]
    fn raiz_con_math(py: Python<'_>, x: f64) -> PyResult<f64> {
        let math = py.import("math")?;
        math.getattr("sqrt")?.call1((x,))?.extract()
    }

    // =====================================================================
    // 3.8 Medir el rendimiento y 3.9 Liberar el GIL
    // =====================================================================

    fn es_primo(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 1;
        }
        true
    }

    /// Cuenta los números primos menores que `limite`.
    /// Mantiene el GIL durante todo el cálculo.
    #[pyfunction]
    fn contar_primos(limite: u64) -> usize {
        (2..limite).filter(|&n| es_primo(n)).count()
    }

    /// Libera el GIL: otros hilos de Python pueden avanzar en paralelo.
    #[pyfunction]
    fn contar_primos_sin_gil(py: Python<'_>, limite: u64) -> usize {
        py.detach(|| (2..limite).filter(|&n| es_primo(n)).count())
    }
}
