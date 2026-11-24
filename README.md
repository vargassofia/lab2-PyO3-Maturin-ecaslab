# Laboratorio 2: PyO3 y Maturin en Rust

Código de la guía *Laboratorio 2: PyO3 y Maturin en Rust* (ECAS Lab). Los ejemplos se compilaron con Rust 1.99.1, Python 3.10.8, Maturin 1.16.0 y PyO3 0.29.3 en macOS (Apple Silicon).

## Contenido

| Carpeta | Sección de la guía | Qué contiene |
|---|---|---|
| [ejemplos_pyo3/](ejemplos_pyo3/) | 2.4–3.9 y 4.4 | Un solo crate con todos los ejemplos de las secciones 2 y 3, un script por sección en `scripts/` y las pruebas de `tests/test_cuenta.py` |
| [errores-comunes/](errores-comunes/) | 3.1–3.7 | Los cuadros "Error común": el código que falla, la salida obtenida y cómo se corrige. Ver su [README](errores-comunes/README.md) |
| [mi_paquete/](mi_paquete/) | 4.3 | Proyecto mixto de Rust y Python, con stubs (`.pyi`) y `py.typed` |
| [ejercicios/](ejercicios/) | 5.1 y 5.2 | Proyectos `estadisticas` y `soda`, cada uno con el enunciado en `src/lib.rs` y su `prueba.py` |
| [regresion/regresion_rs/](regresion/regresion_rs/) | 6 | Proyecto de regresión lineal: los archivos de Python proporcionados (`datos.py`, `regresion_py.py` y `main.py`) y los requisitos en `src/lib.rs` |

> **Soluciones.** Esta rama incluye las soluciones del Anexo A:
> - A.1, en [ejercicios/estadisticas/src/lib.rs](ejercicios/estadisticas/src/lib.rs).
> - A.2, en [ejercicios/soda/src/lib.rs](ejercicios/soda/src/lib.rs).
> - A.3, en [regresion/regresion_rs/src/lib.rs](regresion/regresion_rs/src/lib.rs) y [tests/test_regresion.py](regresion/regresion_rs/tests/test_regresion.py).
> - El `__rmul__` del *Pruébelo usted* de la sección 3.6, en `Vector2D`, con su script `scripts/seccion_3_6_pruebelo_rmul.py`.

## 2. Preparación del entorno

### 2.1 Verificar Rust y Python

```bash
rustc --version
cargo --version
python3 --version
```

En Ubuntu o Debian (incluido WSL): `sudo apt install python3 python3-venv python3-pip`.

### 2.2 Crear un entorno virtual

El entorno se crea una sola vez, en la raíz del repositorio (la carpeta `lab2` de la guía):

```bash
python3 -m venv .venv
```

Para activarlo:

```bash
# macOS y Linux
source .venv/bin/activate

# Windows (PowerShell)
.venv\Scripts\Activate.ps1

# Windows (símbolo del sistema, cmd)
.venv\Scripts\activate.bat
```

Para salir del entorno use `deactivate`. Si PowerShell no permite ejecutar el script de activación, ejecute una vez `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`.

### 2.3 Instalar Maturin

```bash
pip install maturin
maturin --version
pip install pytest
```

Si prefiere tener Maturin fuera del entorno: `pipx install maturin` o `uv tool install maturin`. En Linux, para construir wheels: `pip install maturin[patchelf]`.

### 2.4 Crear un proyecto con maturin new

```bash
maturin new --bindings pyo3 ejemplos_pyo3
cd ejemplos_pyo3
```

### 2.5 Compilar e importar el módulo

```bash
maturin develop
python -c "import ejemplos_pyo3; print(ejemplos_pyo3.sum_as_string(5, 20))"
```

*Pruébelo usted:* en la consola interactiva (`python`):

```python
import ejemplos_pyo3
help(ejemplos_pyo3.sum_as_string)
print(ejemplos_pyo3.__file__)
```

## 3. Fundamentos de PyO3

Cada sección tiene su script en [ejemplos_pyo3/scripts/](ejemplos_pyo3/scripts/):

```bash
cd ejemplos_pyo3
maturin develop
python scripts/seccion_3_1_anatomia.py
```

### 3.8 Medir el rendimiento

```bash
maturin develop --release
python scripts/seccion_3_8_rendimiento.py
python -m timeit -s "import ejemplos_pyo3 as ej" "ej.contar_primos(100_000)"
```

## 4. Maturin a fondo

### 4.1 Comandos principales

| Comando | Qué hace |
|---|---|
| `maturin develop` | Compila en modo depuración e instala el módulo en el entorno activo |
| `maturin develop --release` | Igual, pero con optimizaciones |
| `maturin build --release` | Crea una wheel optimizada en `target/wheels/`, sin instalarla |
| `maturin sdist` | Crea una distribución de código fuente (`.tar.gz`) |
| `maturin publish` | Construye y sube el paquete a PyPI |
| `pip install .` | pip compila el proyecto usando Maturin como build backend |

### 4.2 Distribuir el módulo como una wheel

```bash
cd ejemplos_pyo3
maturin build --release

cd ..
python3 -m venv otro_venv
source otro_venv/bin/activate
pip install ejemplos_pyo3/target/wheels/ejemplos_pyo3-0.1.0-cp310-cp310-macosx_11_0_arm64.whl
```

El nombre de la wheel depende de su versión de Python y de su plataforma. Al terminar, vuelva al entorno del laboratorio:

```bash
deactivate
source .venv/bin/activate
```

Con la característica `abi3`, una sola wheel sirve para todas las versiones de CPython desde una mínima. En `Cargo.toml`:

```toml
pyo3 = { version = "0.29.0", features = ["abi3-py38"] }
```

La wheel se llamará, por ejemplo, `ejemplos_pyo3-0.1.0-cp38-abi3-macosx_11_0_arm64.whl`.

### 4.3 Proyectos mixtos de Rust y Python

```bash
maturin new --mixed --bindings pyo3 mi_paquete
cd mi_paquete
maturin develop
python -c "import mi_paquete as m; print(m.suma_cuadrados([3, 4]), m.norma([3, 4]))"
python -m pytest -v python/tests
```

### 4.4 Pruebas automatizadas con pytest

```bash
cd ejemplos_pyo3
maturin develop
python -m pytest -v tests
```

## 5. Ejercicios

```bash
cd ejercicios/estadisticas   # o ejercicios/soda
maturin develop
python prueba.py
```

## 6. Proyecto: regresión lineal

```bash
cd regresion/regresion_rs
maturin develop --release
python -m pytest -v tests
python main.py
```

## Anexo B. Otros comandos

| Comando | Qué hace |
|---|---|
| `pip install archivo.whl` | Instala una wheel en el entorno activo |
| `rustc --explain E0277` | Explica en detalle un código de error del compilador |
