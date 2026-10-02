[English](README.md) | [Español](README.es.md)

# AgentGuard

AgentGuard es un proyecto en fase inicial para crear una capa local de
seguridad y políticas para agentes de programación que trabajan en repositorios
locales. Su objetivo es responder a cada acción con **ALLOW**, **ASK** o
**DENY**.

> **Estado actual (esqueleto v0.1):** AgentGuard todavía no carga políticas ni
> evalúa acciones, intercepta herramientas, aplica decisiones, solicita
> aprobaciones o se integra con VS Code. Este esqueleto no debe considerarse
> una barrera de seguridad.

## Qué existe actualmente

- Un espacio de trabajo Rust con `agentguard-core` y el esqueleto de
  `agentguard-cli`.
- Un tipo `Decision` cuyo valor predeterminado es `Ask` y cuyo orden representa
  la precedencia prevista entre restricciones.
- Pruebas unitarias, configuración de CI, ejemplos YAML preliminares y un
  directorio reservado para una futura extensión TypeScript, sin paquete
  inicializado.

El CLI solo muestra un aviso sobre su estado. No inspecciona repositorios ni
toma decisiones de política.

## Diseño previsto

AgentGuard Core será independiente de la interfaz y del lenguaje de
programación. Se prevén una política global del usuario y una política
opcional del proyecto. La política del proyecto podrá imponer más
restricciones, pero nunca reducir las de la política global. La precedencia
prevista es **DENY > ASK > ALLOW** y las acciones sin coincidencias deberán
devolver **ASK**. Estos son requisitos de diseño; todavía no se aplican.

Las futuras interfaces de usuario deberán admitir inglés y español. Los
valores de decisión del núcleo son independientes del idioma; cada interfaz
localizará sus propios textos.

## Desarrollo

Instala la versión estable de Rust con los componentes `rustfmt` y `clippy`.
Este esqueleto no requiere Node.js ni VS Code.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p agentguard-cli
```

El último comando solo sirve como comprobación básica. Aún no existe un
procedimiento de instalación, configuración ni uso operativo. El código no
lee los archivos de `policies/examples/`: son ejemplos preliminares del diseño.

## Estructura del repositorio

| Ruta | Propósito |
| --- | --- |
| `crates/agentguard-core/` | Tipos Rust independientes de la interfaz; futuro motor de políticas |
| `crates/agentguard-cli/` | Esqueleto del CLI para desarrollo local |
| `extensions/vscode/` | Directorio reservado para la futura extensión TypeScript |
| `policies/examples/` | Ejemplos YAML preliminares sin funcionalidad |
| `docs/` | Documentación técnica canónica en inglés |
| `tests/` | Directorio reservado para pruebas de integración |
| `.github/workflows/` | CI de Rust |

La [arquitectura](docs/architecture.md) y los [conceptos de política](docs/policy-model.md)
están documentados en inglés. Consulta también las normas para
[contribuir](CONTRIBUTING.md) y la información sobre
[seguridad](SECURITY.md), disponibles en inglés.

Publicado bajo la [licencia MIT](LICENSE).
