# Tarea 2: Investigación — generadores de analizadores léxicos y sintácticos

Compiladores (Módulo 3) · Tecnológico de Monterrey · Entrega: 5 de octubre de 2026

**Alumno:** Jesus Adrian Lopez Gaona · **Matrícula:** A00835462

> **Resultado:** se selecciona **LALRPOP** para el mini-proyecto en Rust; **pest** queda como alternativa si se prefiere PEG.

## Contenido

| Archivo | Descripción |
|---|---|
| [`Entrega_Tarea2.docx`](Entrega_Tarea2.docx) | Documento de entrega: investigación completa, tabla comparativa, selección, conclusión, uso de IA y referencias |
| `README.md` | Este resumen, para leer la investigación sin descargar el documento |

GitHub no muestra archivos `.docx` en el navegador; usa **View raw** o **Download** para abrirlo en Word o Google Docs.

## Objetivo

Comparar tres herramientas que generan automáticamente un **analizador léxico** (scanner: convierte texto en tokens) y un **analizador sintáctico** (parser: verifica que los tokens sigan la gramática), y elegir una para el mini-proyecto del curso, que se programa en Rust.

| Herramienta | Por qué se incluye |
|---|---|
| **Flex/Bison** | La pareja clásica scanner + parser que pide el outline de la tarea |
| **LALRPOP** | Generador de parsers LR que produce código Rust |
| **pest** | Generador de parsers PEG que produce código Rust |

## Comparación por aspecto

La tarea pide documentar seis aspectos de cada herramienta:

| Aspecto | Flex/Bison | LALRPOP | pest |
|---|---|---|---|
| **Plataforma y lenguaje base** | Entornos GNU/Linux y Unix; en Windows, dentro de WSL. Flex genera scanners en C (con soporte C++); Bison genera parsers en C, C++, Java y D | Rust. Se integra con Cargo en Windows, Linux y macOS | Rust. Se integra con Cargo en Windows, Linux y macOS |
| **Características básicas** | Flex recibe patrones y acciones en un archivo `.l`; Bison recibe producciones y acciones en un `.y`. Se ejecutan desde la terminal, generan código fuente y después se compila el programa | Recibe una gramática `.lalrpop`. `build.rs` ejecuta el generador al construir el proyecto y Cargo compila el Rust generado junto con el programa | Recibe una gramática `.pest`. `pest_derive` genera el parser al compilar. Reconoce caracteres y estructuras en la misma gramática, sin scanner separado |
| **Licencia y costo** | Flex: tipo BSD. Bison: GPLv3, con una excepción que permite usar libremente el parser generado. Ambas gratuitas | Dual MIT/Apache-2.0. Gratuita | Dual MIT/Apache-2.0. Gratuita |
| **Fundamentos teóricos** | Flex: autómatas finitos para reconocer expresiones regulares. Bison: gramáticas libres de contexto; tablas LALR(1) por defecto (también IELR(1) y LR(1) canónico) y análisis GLR | LR(1) por defecto; también permite LALR(1). Su lexer predeterminado reconoce tokens con las expresiones regulares y símbolos de la gramática | Gramáticas PEG con **alternativas ordenadas**: intenta la primera opción y, si falla, la siguiente. El orden de las reglas puede cambiar el resultado |
| **Interfaz** | Línea de comandos para generar el código; el programa generado expone `yylex()` (siguiente token) y `yyparse()` (análisis sintáctico) | Cargo + API generada: `ExprParser::new().parse(texto)` devuelve el resultado o un error | API de Rust: `Parser::parse(Rule::regla, texto)` devuelve `Pairs`, que se recorren con `Pair` para obtener el texto y sus partes |
| **Código propio** | Acciones en C dentro de cada regla; los tokens y sus valores se coordinan entre Flex y Bison | Acciones en Rust dentro de cada regla, después de `=>` | Funciones de Rust **fuera** de la gramática que procesan el resultado |

## Código propio: el mismo ejemplo en las tres herramientas

Las tres versiones suman números. Cada fragmento se compiló y ejecutó (bison 3.8.2 + flex 2.6.4, LALRPOP 0.23.1, pest 2.9.2).

### Flex/Bison — acción en C dentro de la regla

```yacc
/* suma.y (Bison) */
expr : expr '+' term     { $$ = $1 + $3; }
     | term
     ;
term : NUM ;
```

```lex
/* suma.l (Flex): entrega cada token y su valor a Bison */
[0-9]+   { yylval = atoi(yytext); return NUM; }
[+\n]    { return yytext[0]; }
```

`$$` es el valor de la regla y `$1`, `$3` son los valores de sus símbolos. Completado con una regla de inicio que imprime el resultado, la entrada `2 + 3 + 4` imprime `9`.

### LALRPOP — acción en Rust dentro de la regla

```rust
// calculadora.lalrpop
grammar;

pub Expr: i64 = <a:Num> "+" <b:Num> => a + b;

Num: i64 = <s:r"[0-9]+"> => s.parse::<i64>().unwrap();
```

```rust
let suma = calculadora::ExprParser::new().parse("2 + 3"); // Ok(5)
```

Lo que va después de `=>` es Rust y calcula el valor (o construye el árbol) en la misma gramática.

### pest — el código vive fuera de la gramática

```text
// suma.pest
suma   = { numero ~ "+" ~ numero }
numero = @{ ASCII_DIGIT+ }
```

```rust
let par = SumaParser::parse(Rule::suma, "2+3").unwrap().next().unwrap();
let total: i64 = par
    .into_inner()
    .map(|n| n.as_str().parse::<i64>().unwrap())
    .sum(); // 5
```

La gramática solo reconoce el texto; el significado se programa aparte, recorriendo los `Pair`.

## Ventajas y consideraciones

| Herramienta | Ventaja | Consideración |
|---|---|---|
| Flex/Bison | Permite estudiar por separado el scanner y el parser | Su combinación habitual genera C; usarla desde Rust requiere integración adicional |
| LALRPOP | Genera Rust y admite acciones en las producciones | La gramática debe ser compatible con el método LR seleccionado |
| pest | Integra el reconocimiento de caracteres y estructuras | Las alternativas son ordenadas y el procesamiento se programa aparte |

## Selección: LALRPOP

- Genera código **Rust**, el lenguaje del mini-proyecto.
- Se integra con **Cargo** mediante `build.rs`, sin herramientas externas.
- Permite asociar **acciones Rust** a las producciones, lo que facilita estudiar tokens, gramática y construcción de resultados en un solo lugar.

pest es una buena alternativa si se prefiere el enfoque PEG.

## Conclusión

Las tres opciones generan analizadores a partir de reglas, pero difieren en cómo se integran. Flex/Bison muestra la arquitectura clásica de scanner y parser separados; LALRPOP genera parsers LR para Rust; pest utiliza PEG con alternativas ordenadas. Para un proyecto en Rust, LALRPOP ofrece la integración más directa.

## Uso de inteligencia artificial

### Documento de entrega

Según el documento:

- **Herramienta:** ChatGPT, con asistencia de Codex para consultar documentación oficial y preparar el ejemplo, el código y el documento. Se usó búsqueda web para verificar características.
- **Modelo:** no se declara porque no se proporcionó un identificador comprobable.

Prompts utilizados, copiados literalmente:

1. > Ayúdame a hacer el research para hacer la siguiente tarea. Yo quiero hacer la tarea pero apreciaría de tu ayuda con links del research con resúmenes para que pueda comprobar yo la información. la información que me enseñes hazlo con diferentes niveles de dificultad. primero presenta la información de manera clara, concisa y sin tanto técnico jargon. después profundiza en los temas con ejemplos
2. > Aqui esta lo que desarrolle ayudame analizandolo, dime si me falta algo, si esto es trabajo apropiado en base a los requisitos de la tarea o si piensas que puede mejorarse de alguna manera y porque

### Este README

- **Herramienta:** Claude Code (modelo Claude Opus 5.5). Redactó este README a partir del documento de entrega, escribió los ejemplos de código propio y los compiló y ejecutó para verificarlos.
- **Prompt, copiado literalmente:**

  > agrega el siguiente dox al github, agrega un readme bueno

## Referencias

- [Manual de Flex](https://westes.github.io/flex/manual/): entradas, acciones y funcionamiento del scanner.
- [Manual de Bison](https://www.gnu.org/software/bison/manual/bison.html): métodos sintácticos, interfaz y acciones.
- [Inicio rápido de LALRPOP](https://lalrpop.github.io/lalrpop/quick_start_guide.html): integración con Cargo y generación.
- [Documentación de LALRPOP](https://docs.rs/lalrpop/latest/lalrpop/): características y método predeterminado.
- [Documentación de pest](https://docs.rs/pest_derive/latest/pest_derive/): gramáticas y generación del parser.
