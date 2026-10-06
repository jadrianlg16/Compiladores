# Compiladores — Módulo 3

Tecnológico de Monterrey · Desarrollo de Aplicaciones Avanzadas de Ciencias Computacionales · Septiembre 2026.

Lenguaje del mini-proyecto: **Rust**.

## Contenido

| Carpeta | Entrega | Fecha límite |
|---|---|---|
| [`tarea1-estructuras-datos/`](tarea1-estructuras-datos/) | Tarea 1: STACK (LIFO), QUEUE (FIFO) y TABLE/HASH/DICTIONARY (recorrido ordenado por llave), con versión sobre la biblioteca estándar, versión manual y diccionario con tabla hash (`HashMap`) | 2026-10-01 |
| [`tarea2/`](tarea2/) | Tarea 2: investigación y comparación de generadores de analizadores léxicos y sintácticos (Flex/Bison, LALRPOP, pest) y selección para el mini-proyecto | 2026-10-05 |

## Tarea 1 en corto

```sh
cd tarea1-estructuras-datos
cargo run     # programa de demostración
cargo test    # 41 pruebas
```

- Descripción y diseño: [`tarea1-estructuras-datos/README.md`](tarea1-estructuras-datos/README.md)
- Casos de prueba: [`tarea1-estructuras-datos/TEST_CASES.md`](tarea1-estructuras-datos/TEST_CASES.md)
- Uso de IA (herramientas y prompts): [`tarea1-estructuras-datos/AI_USAGE.md`](tarea1-estructuras-datos/AI_USAGE.md)
- Guía interactiva (simulador paso a paso, código comentado y quiz): abre [`tarea1-estructuras-datos/docs/interactive_guide.html`](tarea1-estructuras-datos/docs/interactive_guide.html) en el navegador
- Guía de estudio línea por línea (PDF): [`tarea1-estructuras-datos/docs/`](tarea1-estructuras-datos/docs/)

## Tarea 2 en corto

Se comparan Flex/Bison, LALRPOP y pest en seis aspectos (plataforma, características, licencia, fundamentos teóricos, interfaz y código propio). Se selecciona **LALRPOP** porque genera Rust, se integra con Cargo y admite acciones Rust en las producciones.

- Resumen, tablas comparativas y ejemplos de código: [`tarea2/README.md`](tarea2/README.md)
- Documento de entrega: [`tarea2/Entrega_Tarea2.docx`](tarea2/Entrega_Tarea2.docx)
