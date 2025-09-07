# Ween Language Specification (MVP)

> [!IMPORTANT] 
> **THIS IS ONLY A DRAFT, IT COULD CHANGE WITH TIME**

## 1. Overview

Ween is a **type-safe, dependency-free frontend language** designed for building interactive web applications with a clean, minimal syntax.
Key principles:

* Structured like a DSL rather than HTML.
* Reusable styles via `def`.
* Optional inline styling via `opt={…}`.
* Comma-separated properties inside blocks.
* Strings quoted; bare words are variable references.
* Indentation: 1 tab (or 4 spaces) per block level.

---

## 2. Top-level Directives

### `lang`

Sets the page language. **Semicolon required.**

```ween
lang="en";
```

---

## 3. Style Definitions

### `def`

Declares a reusable style (like a CSS class).

```ween
def important-text(TextFamily) {
    font-style = bold,
    font-size = 16px,
}
```

* **Syntax:** `def Name(Type) { property1=value1, property2=value2, … }`
* Comma-separated properties. Trailing comma optional.
* Applied to elements with `<Name>`.

---

## 4. Page Structure

### `header`

Contains page metadata such as viewport, charset, site name.

```ween
header {
    main = {
        display {
            name = "viewport",
            content = "width=device-width, initial-scale=1.0",
        },
        charset = "UTF-8",
        site_name = "Ween",
    }
}
```

### `body`

Contains visible elements like `h1`, `h2`, `text`, etc.

```ween
body {
    h1<important-text> {
        "Hello, World!",
        opt = {
            color = "Red",
            font-size = 20px,
        }
    }
}
```

---

## 5. Elements

* Basic elements: `h1`, `h2`, `text`, `div`, etc.
* Optional **class application** via `<class-name>`.
* Optional inline styles via `opt={…}`.
* Strings must be quoted. Units like `10px` can be bare or quoted.

---

## 6. Property Rules

* Inside `def` or `opt={…}` blocks:

  * Properties are **comma-separated**.
  * Trailing comma is allowed.
  * Line breaks are allowed.

```ween
opt = {
    color = "red",
    font-size = 16px,
}
```

---

## 7. Comments

* **Single-line:** `// comment`
* **Multi-line:** `/** comment */`

---

## 8. Example: Minimal Page

```ween
lang="en";

def important-text(TextFamily) {
    font-style = bold,
    font-size = 16px,
}

header {
    main = {
        display {
            name = "viewport",
            content = "width=device-width, initial-scale=1.0",
        },
        charset = "UTF-8",
        site_name = "Ween",
    }
}

body {
    h1<important-text> {
        "Hello, World!",
        opt = {
            color = "Red",
            font-size = 20px,
        }
    }
}
