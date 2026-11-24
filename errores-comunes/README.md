# Errores comunes

Esta carpeta reúne los cuadros **"Error común"** de la sección 3 del *Laboratorio 2: PyO3 y Maturin en Rust*. Cada error tiene su propia carpeta o script, con el código roto tal como aparece en la guía. La corrección se indica en un comentario `Corrección:`.

Los errores aparecen en dos momentos distintos:

- **Al compilar.** `rustc` los reporta al ejecutar `maturin develop` (secciones 3.4, 3.5 y 3.7).
- **Al ejecutar.** Python lanza una excepción con su Traceback (secciones 3.1, 3.2 y 3.3).

| Sección | Error | Dónde | Tipo |
|---|---|---|---|
| 3.1 | El nombre del módulo no coincide con `[lib] name` | [seccion_3_1_nombre_modulo/](seccion_3_1_nombre_modulo/) | `ImportError` al importar |
| 3.2 | Pasar un tipo incorrecto | [seccion_3_2_tipo_incorrecto.py](seccion_3_2_tipo_incorrecto.py) | `TypeError` |
| 3.3 | Pasar por posición un argumento que solo acepta nombre | [seccion_3_3_solo_por_nombre.py](seccion_3_3_solo_por_nombre.py) | `TypeError` |
| 3.4 | Retornar `Result<T, String>` | [seccion_3_4_result_string/](seccion_3_4_result_string/) | `error[E0277]` |
| 3.5 | Modificar el objeto desde `&self` | [seccion_3_5_mut_self/](seccion_3_5_mut_self/) | `error[E0594]` |
| 3.7 | Olvidar la coma de una tupla de un elemento | [seccion_3_7_tupla_sin_coma/](seccion_3_7_tupla_sin_coma/) | `error[E0277]` |

Todos los comandos suponen que el entorno virtual está activado (`source .venv/bin/activate` desde la raíz del repositorio). Las salidas que se muestran aquí se obtuvieron con PyO3 0.29, Maturin 1.16 y Python 3.10. Las rutas largas se recortaron.

---

## 3.1 El nombre del módulo no coincide con `[lib] name`

En `Cargo.toml` está `[lib] name = "nombre_modulo"`, pero en `src/lib.rs` el mod se llama `mis_ejemplos`.

```bash
cd errores-comunes/seccion_3_1_nombre_modulo
maturin develop
python prueba.py
```

Maturin compila, pero avisa:

```
⚠️  Warning: Couldn't find the symbol `PyInit_nombre_modulo` or `PyModExport_nombre_modulo` in the native library. Python will fail to import this module. If you're using pyo3, check that `#[pymodule]` uses `nombre_modulo` as module name
```

Al importar:

```
Traceback (most recent call last):
  File ".../seccion_3_1_nombre_modulo/prueba.py", line 11, in <module>
    import nombre_modulo
  File ".../site-packages/nombre_modulo/__init__.py", line 1, in <module>
    from .nombre_modulo import *
ImportError: dynamic module does not define module export function (PyInit_nombre_modulo)
```

**¿Qué significa?** La biblioteca compilada exporta una función llamada `PyInit_mis_ejemplos`. Python importa el paquete `nombre_modulo`, así que busca `PyInit_nombre_modulo` y no la encuentra.

**¿Cómo se corrige?** Use el mismo nombre en el `mod` de `src/lib.rs` y en el campo `name` de `[lib]` en `Cargo.toml`, y vuelva a ejecutar `maturin develop`.

> En la guía, el error se muestra cambiando el nombre del mod dentro del proyecto `ejemplos_pyo3`. Aquí el ejemplo usa otro nombre (`nombre_modulo`) para que `maturin develop` no reemplace el módulo `ejemplos_pyo3`, que sí funciona. Al terminar, desinstálelo con `pip uninstall nombre_modulo`.

---

## 3.2 Pasar un tipo incorrecto (`TypeError`)

Este script necesita el módulo `ejemplos_pyo3` instalado (`cd ejemplos_pyo3 && maturin develop`).

```bash
cd errores-comunes
python seccion_3_2_tipo_incorrecto.py
```

```
TypeError: 'str' object cannot be interpreted as an integer
OverflowError: Python int too large to convert to C long
(3, 12)
```

**¿Qué significa?** El compilador de Rust no puede revisar las llamadas que se hacen desde Python. PyO3 intentó convertir cada elemento de la lista en un `i64`, y el texto `"3"` no se puede interpretar como un entero. Por eso lanzó `TypeError` antes de ejecutar el cuerpo de la función. El código de Rust nunca recibe datos con el tipo equivocado.

**¿Cómo se corrige?** Convierta los datos en Python antes de llamar a la función, por ejemplo con `int("3")`. Si la función debe aceptar varios tipos, reciba un `&Bound<'py, PyAny>` y decida qué hacer según el tipo (sección 3.7).

El script también incluye el *Pruébelo usted*: `ej.min_max([2**70])` lanza `OverflowError`, porque los enteros de Python no tienen límite de tamaño y los de Rust sí.

---

## 3.3 Pasar por posición un argumento que solo acepta nombre

Este script necesita el módulo `ejemplos_pyo3` instalado.

```bash
cd errores-comunes
python seccion_3_3_solo_por_nombre.py
```

```
TypeError: repetir() takes from 1 to 2 positional arguments but 3 were given
hola-hola-hola
```

**¿Qué significa?** En la firma `(texto, veces = 2, *, separador = " ")`, `separador` está después del `*`, así que solo se puede pasar por nombre. La función acepta como máximo dos argumentos posicionales (`texto` y `veces`) y recibió tres.

**¿Cómo se corrige?** Llame a la función con el nombre del parámetro: `ej.repetir("hola", 3, separador="-")`.

---

## 3.4 Retornar `Result<T, String>` (E0277)

```bash
cd errores-comunes/seccion_3_4_result_string
maturin develop
```

```
error[E0277]: the trait bound `PyErr: From<String>` is not satisfied
  --> src/lib.rs:27:37
   |
27 |     fn parsear_edad(texto: &str) -> Result<u8, String> { // <- E0277: String no se convierte en PyErr
   |                                     ^^^^^^ the trait `From<String>` is not implemented for `PyErr`
   |
   = help: `PyErr` implements trait `From<T>`:
             From<AddrParseError>
             From<CastError<'_, '_>>
             From<CastIntoError<'_>>
             From<CharTryFromError>
             From<FromUtf8Error>
             From<Infallible>
             From<IntoInnerError<W>>
             From<IntoStringError>
           and 17 others
   = note: required for `String` to implement `Into<PyErr>`
```

**¿Qué significa?** PyO3 acepta funciones que retornan `Result<T, E>` solo si `E` se puede convertir en un `PyErr`, porque necesita saber qué excepción lanzar. Con un `String` no lo sabe: ¿debería ser `ValueError`, `TypeError` u otra? La línea `help:` enumera algunos tipos que sí tienen esa conversión.

**¿Cómo se corrige?** Retorne `PyResult<u8>` y convierta el error con `map_err`:

```rust
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn parsear_edad(texto: &str) -> PyResult<u8> {
    let edad: u8 = texto.trim().parse().map_err(|_| {
        PyValueError::new_err(format!("'{texto}' no es una edad válida"))
    })?;
    Ok(edad)
}
```

Si su programa tiene un tipo de error propio, implemente `From` una sola vez y después use `?` libremente:

```rust
struct EdadInvalida(String);

impl From<EdadInvalida> for PyErr {
    fn from(e: EdadInvalida) -> PyErr {
        PyValueError::new_err(e.0)
    }
}
```

---

## 3.5 Modificar el objeto desde `&self` (E0594)

```bash
cd errores-comunes/seccion_3_5_mut_self
maturin develop
```

```
error[E0594]: cannot assign to `self.saldo`, which is behind a `&` reference
  --> src/lib.rs:44:13
   |
44 |             self.saldo += monto; // <- E0594: no se puede asignar a través de &self
   |             ^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so it cannot be written to
   |
help: consider changing this to be a mutable reference
   |
40 |         fn depositar(&mut self, monto: f64) -> PyResult<()> {
   |                       +++
```

**¿Qué significa?** Es la misma regla de préstamos del Laboratorio 1: con `&self` el método solo puede leer el objeto, y `self.saldo += monto` intenta modificarlo.

**¿Cómo se corrige?** Tal como sugiere `help:`, declare el método con `&mut self`. Desde Python la llamada no cambia: `cuenta.depositar(250)`.

---

## 3.7 Olvidar la coma de una tupla de un elemento (E0277)

```bash
cd errores-comunes/seccion_3_7_tupla_sin_coma
maturin develop
```

Se omitieron algunas líneas intermedias del mensaje:

```
warning: unnecessary parentheses around method argument
  --> src/lib.rs:27:44
   |
27 |             resultado.append(funcion.call1((valor))?)?; // <- E0277: (valor) no es una tupla
   |                                            ^     ^
   |
   = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default

error[E0277]: `pyo3::Bound<'_, pyo3::PyAny>` cannot used as a Python `call` argument
   --> src/lib.rs:27:44
    |
 27 |             resultado.append(funcion.call1((valor))?)?; // <- E0277: (valor) no es una tupla
    |                                      ----- ^^^^^^^ the trait `PyCallArgs<'_>` is not implemented for `pyo3::Bound<'_, pyo3::PyAny>`
    |                                      |
    |                                      required by a bound introduced by this call
    |
    = note: `PyCallArgs` is implemented for Rust tuples, `Bound<'py, PyTuple>` and `Py<PyTuple>`
    = note: if your type is convertible to `PyTuple` via `IntoPyObject`, call `<arg>.into_pyobject(py)` manually
    = note: if you meant to pass the type as a single argument, wrap it in a 1-tuple, `(<arg>,)`
...
help: use a unary tuple instead
    |
 27 |             resultado.append(funcion.call1(((valor),))?)?; // <- E0277: (valor) no es una tupla
    |                                            +       ++
```

**¿Qué significa?** En Rust, igual que en Python, `(valor)` no es una tupla: los paréntesis solo agrupan. El compilador lo detecta dos veces. Da una advertencia por paréntesis innecesarios y un error porque un `Bound<PyAny>` no sirve como lista de argumentos.

**¿Cómo se corrige?** Escriba la tupla de un elemento con la coma final: `funcion.call1((valor,))`. Para llamar sin argumentos use `call0()`, y para pasar argumentos por nombre, `call(args, Some(&kwargs))`.
