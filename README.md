# Evolution simulator

## Setup

Instalar una versión Nightly del compilador de Rust, método recomendado: `https://rustup.rs/`.

Correr el comando `cargo build --release`, esto descarga todas las librerías utilizadas y las compila, junto con el programa.
La primera vez demora más que las siguientes dado que tiene que descargar todas las dependencias.
Este comando compila la versión release del programa, que ejecuta más rápido que su contraparte debug.

El comando `cargo run --release` puede ser usado para correr el binario.
Por defecto, la aplicación corre un algoritmo genético.

## Configuración

### Argumentos por linea de comandos

Para pasarle flags al binario es necesario separar las flags que recibe `cargo` con `--`.
Ejemplo: `cargo run --<cargo-flag> -- --<binary-flag>`

El binario puede recibir las siguientes flags:
- `-h` o `--headless`: Corre el programa sin abrir una aplicación gráfica, mejora la performance.
- `-s` o `--speciesism`: Corre el algoritmo genético con una técnica de reproducción entre especies.
- `--max-generations`: Define la cantidad máxima de generaciones para el algoritmo.
- `--max-no-improvement`: Define la cantidad de generaciones que pueden pasar sin que mejore la fitness máxima antes de que finalice el programa
- `--mutation`: Define la probabilidad de mutación
- `--crossover`: Define la probabilidad de cruzamiento
- `--population-size`: Define el tamaño de población para el algoritmo
- `--instance <number>`: Define la instancia que se ejecutará, se pueden definir distintas instancias en `config.ron`
- `--playground`: Corre un playground en vez de un algoritmo genético, en el cual se lee un cromosoma de `chromosome.ron` y se simula el movimiento de esa criatura (explicado en detalle más adelante)
- `--test`: Indica que el programa es parte de un test (esto permite realizar muchas ejecuciones y guardar resultados en archivos)
- `--test-count`: Define la cantidad de ejecuciones de esta sesión (requiere `--test`)

### Archivo de configuración

Otros valores de configuración pueden ser modificados creando un archivo `config.ron` y cambiando los valores por defecto.
Los valores por defecto se pueden encontrar en `example.config.ron`.

## Tests

Luego de correr una sesión de test (con las flags `--test` y `--test-count`), los resultados se guardarán en `experiments/population_{tamaño de población}_mutation_{probabilidad de mutación}_crossover_{probabilidad de cruzamiento}/{numero de instancia que se esta ejecutando}/`

Para generar más estadísticas para las instancias, se pueden usar los scripts (escritos en Python) en `scripts/`.
Para ejecutar ciertos scripts es necesario instalar las dependencias de Python de `requirements.txt`.
Para esto, se puede usar el comando `pip install -r requirements.txt` con una versión de Python 3.

Todos los scripts deben ser ejecutados en la carpeta de una instancia (`experiments/{configuración paramétrica}/instance_{numero instancia}/`).

### generate\_stats

Genera estadísticas para cada ejecución y las guarda en `execution_{numero}/new_stats.txt`.

### avg\_stats

Usa los `new_stats.txt` de cada ejecución para promediarlos e imprime esas estadísticas en stdout.

### ks\_test

Realiza el test de Kolmogorov-Smirnov.

### mw\_test

Realiza el test de Mann-Witney.

## Playground

Para no correr el algoritmo genético y en su lugar experimentar con la simulación, se puede pasar la flag `--playground`.
El programa lee un cromosoma del archivo `chromosome.ron` y crea una criatura con los atributos definidos en él.
Un archivo ejemplo se encuentra en `example.chromosome.ron`.
