# Proyecto 2: Problema de k-MST

---

## 📦 Requisitos

Antes de compilar y correr el proyecto, asegúrate de tener instalados los siguientes programas:

### 1. Rust y Cargo
Instala el *toolchain* oficial de Rust que incluye `cargo` (el gestor de paquetes y compilación):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Verifica la instalación:
```bash
rustc --version
cargo --version
```
---

## Construcción del proyecto.
Para esto, clona este repositorio y entra en la carpeta del proyecto:
```bash
git clone https://github.com/MedinaPeraltaJoaquin/k-MST
cd k-MST
```
Compila en modo debug:
```bash
cargo build --release
```

---

## 🚀 Ejecución del Proyecto

Para ejecutar el programa, puedes usar el comando `cargo run --` seguido de las opciones, o ejecutar directamente el binario compilado `target/release/k-MST`:

```bash
cargo run -- -p <file.txt> -k <number> -s <opciones>
./target/debug/k-MST <opciones>
```

### 📋 Opciones de Línea de Comandos

El programa utiliza el parser de argumentos definido en read_input.rs para configurar la ejecución, incluyendo la ruta del grafo y el valor K.

Indica --help o -h para mostrar el menú completo:
```bash
Uso: programa [opciones]

Opciones:
  -h, --help         Muestra esta ayuda y termina
  -v                 Activa el modo verbose
  -p <path>          Ruta explícita del archivo .txt que representa una gráfica (OBLIGATORIO)
  -svg               Activa el modo de salida SVG (Genera imágenes de convergencia y árbol final)
  -s <I> <F>         Genera semillas en el rango [I, F] (ej: -s 1 10)
  -s <n>             Inicializa con la semilla n (ej: -s 42)
  -rs <n>            Genera n semillas aleatorias
  -k <n>             Valor para encontrar la k-MST (OBLIGATORIO)
```
### Ejemplo de Ejecución:

Para buscar el 5-MST en el archivo data/grafo.txt usando la semilla 42 y generar archivos SVG:
```bash
cargo run -- -p data/grafo.txt -k 5 -s 42 -svg
```

### ⚙️ Archivo de Configuración (.env)

El algoritmo WOA (Whale Optimization Algorithm) utiliza un archivo .env para cargar sus hiperparámetros de control. Este archivo debe estar en la raíz del proyecto.

| Parámetro | Descripción | Tipo | Valor Sugerido |
| :--- | :--- | :--- | :--- |
| **`SIZE_POPULATION`** | Número de ballenas (agentes). | `usize` | `50` |
| **`MAX_ITERATION`** | Número máximo de ciclos de optimización. | `usize` | `2000` |
| **`LB`** | Límite inferior del espacio de búsqueda continuo. | `f64` | `-10000.0` |
| **`UB`** | Límite superior del espacio de búsqueda continuo. | `f64` | `10000.0` |

Un ejemplo del archivo .env es:
```bash
# Hiperparámetros del Algoritmo de Optimización de Ballenas (WOA)
SIZE_POPULATION=50
MAX_ITERATION=2000
LB=-10000.0
UB=10000.0
```
