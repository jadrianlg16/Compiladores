# AI assistance disclosure

## Tool and scope

Tool: OpenAI ChatGPT / Codex assistant in ChatGPT Work mode. No other AI assistant was used in this conversation. The assistant read the assignment PDF, proposed the design, generated both Rust implementations, the demonstration, tests, English documentation, and the line-by-line PDF. It also executed local verification and inspected rendered PDF pages. Rust/Cargo, Git, Python, ReportLab, and Poppler are supporting development/document tools, not additional AI models. Official Rust documentation was consulted through web retrieval.

This is substantial AI assistance, including generated code, not merely spelling correction.

## Exact user prompts in this conversation

The original spelling and languages are preserved below. These are the actual user inputs, not reconstructed implementation prompts.

### 1. Planning request (with the assignment PDF attached)

```text
ayudame a ver como haria la tarea 1, hazme preguntas aclaratorias si ocupas. y despues haz el plan para completarla
```

### 2. Language clarification

```text
Rust
```

### 3. Library preference clarification

```text
prefiero usar bibiotecas al menos que la tarea especifique hacerlas desde 0
```

### 4. Request for a second implementation

```text
una pequenia cosa, me gustaria tambien hacer otra version que haga lo mismo pero sin las librerias, para entender las dos manera
```

### 5. Request to generate English deliverables and a learning guide

```text
okay also have the work in english as well as the comments and the code, help me write up the files, as weel as a pdf where you take lines of code and functions, classes, etc and help me understand everything line by line, from a macro level to micro as well as the rust syntaxis etc
```

The assistant made routine implementation and formatting decisions under these requests. No invented user prompt or hidden internal reasoning is presented as a user consultation.

## Review, repository organization, and HashMap version

Tool: Anthropic Claude Code (desktop app).

With the first prompt below, it reviewed the delivered project against the assignment PDF, re-ran the tests, formatter, linter, and demonstration, moved the supporting documents into `docs/`, updated file paths in the documentation, added a re-verification section to `docs/VERIFICATION.md`, wrote the course-level `README.md`, and published the folder to the course Git repository. It did not change any Rust file in that step.

With the second prompt, it generated the `LibraryHashDictionary` adapter (appended to `src/std_impl.rs`), the third demonstration in `src/main.rs`, and the `hash_table` test module plus the HashMap comparisons in `tests/behavior.rs`. It updated `README.md`, `TEST_CASES.md`, `docs/VERIFICATION.md`, and the captured outputs in `docs/`. It also removed documentation text addressed to the student instead of the reader: it rewrote the dictionary-interpretation paragraph in `README.md`, which asked to confirm the interpretation with the instructor, and removed from this file the two sentences about reading the guide and reviewing the code before submission.

Exact prompts:

1. With the project ZIP and the assignment PDF attached:

```text
checa si esto esta bien mi trabajo. dime que si esta bien, que no. esto solo es de la tarea 1 asi que asegurate que se organice muy bien en este folder.  sube a mi github sin claude como coauthor.
```

2. After Claude Code reported that the project had no hash table and that two documentation sentences were addressed to the student:

```text
agrega la versión con HashMap y limpia esas frases
```

3. Interactive learning page:

```text
ayudame a agregar un artifacto html que sea interactivo para aprender como funciona, y tomar qui para hacer esto, y alguna represteacion vuizaual de lo que esta pasadno y lo que cada linea o funcion o parte importante en el codigo hace
```

With the third prompt, Claude Code generated `docs/interactive_guide.html`: a step-by-step simulator of every operation with memory diagrams, an annotated source browser, an overview of the design, and a quiz. It did not change any Rust file in that step. The per-line explanations reuse `docs/LINE_BY_LINE.json` (written by ChatGPT, see above); Claude Code wrote the explanations for the lines added with the HashMap version, the function summaries, the step narrations, and the quiz.
