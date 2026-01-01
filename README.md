# Ween Compiler

> [!NOTE]  
> The main branch contains the last working version of the compiler, while the develop the last updated (but not tested).

The **Ween Compiler** is the core of the Ween frontend programming language: a type-safe, minimal, dependency-free language for building interactive web applications.

## Overview

Ween is designed with security, stability, and longevity in mind. Unlike traditional frontend frameworks, it does not rely on external dependencies or runtime libraries. Applications written in Ween compile directly to minimal, standards-compliant web output, ensuring small bundle sizes and predictable behavior for decades.

The compiler handles:

* Parsing `.wn` source files into an abstract syntax tree (AST)
* Type checking and validation of UI elements, attributes, and reactive state
* Code generation to minimal DOM instructions or WASM output
* Handling of reactive state and asynchronous operations

## Contributing

Contributions are welcome. Please fork the repository, create a branch for your changes, and submit a pull request. All contributions must comply with the Apache-2.0 license.

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
