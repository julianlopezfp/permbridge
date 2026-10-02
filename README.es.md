[English](README.md) | [Español](README.es.md)

# PermBridge

**PermBridge significa Permission Bridge («puente de permisos»).** Es un
proyecto orientado a desarrolladores para comparar la compatibilidad de
políticas entre agentes de programación. Su objetivo es definir permisos una
sola vez, comparar cómo los aplica cada agente, detectar brechas de seguridad
y reglas no compatibles, y mantener una postura de seguridad coherente entre
herramientas.

Los agentes pueden utilizar modelos de permisos, formatos de configuración,
mecanismos de aprobación y garantías de seguridad diferentes. Cambiar de
agente puede alterar la postura de seguridad efectiva sin que resulte evidente.
PermBridge pretende hacer visibles esas diferencias y explicar cómo abordarlas.

> **Estado actual:** Core dispone de un modelo de políticas en memoria y un
> contrato de adaptadores, ambos experimentales. No carga YAML, importa la
> configuración de ningún agente real, compara posturas, genera diagnósticos,
> intercepta acciones, media aprobaciones ni se integra con VS Code. No hay
> adaptadores reales y actualmente no proporciona protección de seguridad.

![Ilustración conceptual de PermBridge](images/permbridge-github-social-preview.jpg)

La imagen representa la dirección prevista del producto, no funcionalidades
ya implementadas.

## Dirección del producto

El flujo previsto es:

```text
Política deseada -> modelo canónico -> adaptador -> postura efectiva
                 -> comparación -> diagnósticos -> informe
```

El modelo canónico expresa los permisos deseados sin depender de un agente.
Los futuros adaptadores interpretarán las configuraciones nativas que admitan y
declararán sus límites. El comparador identificará correspondencias
equivalentes, más restrictivas, menos restrictivas, no compatibles o ambiguas.
Los diagnósticos e informes explicarán las diferencias y posibles soluciones.
Primero se prevé un CLI unificado; la extensión para VS Code vendrá después.

PermBridge se centra en compatibilidad de políticas, comparación de posturas
y portabilidad entre agentes de programación. La aplicación o mediación de
permisos solo será posible en el futuro cuando el agente subyacente ofrezca un
mecanismo fiable. No es un entorno aislado universal ni un cortafuegos general
para agentes de IA.

## Qué está implementado

- Un espacio de trabajo Rust con `permbridge-core` y un programa básico
  `permbridge-cli`.
- Un modelo experimental en memoria, independiente de la interfaz, para
  decisiones sobre archivos, red y ejecución, además de ámbitos de política
  y mecanismos de aplicación.
- Un contrato `AgentAdapter` y tipos que distinguen correspondencias no
  compatibles o ambiguas. Ningún agente real implementa aún el contrato.
- Resultados conceptuales de comparación y pruebas unitarias de los
  invariantes implementados. Todavía no existe un comparador.
- CI de Rust, ejemplos YAML ilustrativos y un directorio reservado para la
  futura extensión TypeScript de VS Code. El código no lee ni valida el YAML.

El CLI solo muestra un aviso. No calcula ningún resultado de comparación.

## Desarrollo

Instala Rust estable con los componentes `rustfmt` y `clippy`. Este esqueleto
no requiere Node.js ni VS Code.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p permbridge-cli
```

El último comando es una comprobación básica; todavía no hay un CLI de uso
operativo ni un procedimiento de instalación o configuración para usuarios.
Los archivos de `policies/examples/` ilustran un formato de política en
evolución y no tienen ningún efecto.

## Política y próximos pasos

El modelo representa ámbitos de política administrada, global, de proyecto y
de sesión. Está previsto que la política opcional de proyecto no pueda reducir
las restricciones globales. Las decisiones canónicas son `ALLOW`, `ASK` y
`DENY`, con precedencia `DENY > ASK > ALLOW`; una política nueva usa `ASK` por
defecto. La combinación de ámbitos y la evaluación de acciones aún no están
implementadas. Los adaptadores no deberán suponer que todos los agentes
tienen estados de permisos idénticos.

Los próximos hitos son un esquema YAML versionado y su cargador, un primer
adaptador probado, la comparación de posturas y diagnósticos útiles en el CLI.
Más adaptadores y una experiencia localizada en VS Code llegarán después.
Consulta la [hoja de ruta](docs/roadmap.md), disponible en inglés.

## Estructura del repositorio

| Ruta | Propósito |
| --- | --- |
| `crates/permbridge-core/` | Modelo Rust canónico y contrato de adaptadores; futura comparación |
| `crates/permbridge-cli/` | Programa básico Rust; futuro CLI para usuarios |
| `adapters/` | Futuras implementaciones específicas; aún sin integraciones |
| `extensions/vscode/` | Directorio reservado para la futura extensión TypeScript |
| `policies/examples/` | Ejemplos YAML preliminares sin funcionalidad |
| `docs/` | Documentación técnica canónica en inglés |
| `tests/` | Directorio reservado para futuras pruebas de integración |
| `.github/workflows/` | CI de Rust |

La [arquitectura](docs/architecture.md), el
[modelo canónico](docs/canonical-policy-model.md) y el
[contrato de adaptadores](docs/adapter-contract.md) se documentan en inglés.
El [índice técnico](docs/README.md) reúne las demás guías. Consulta
también las normas para [contribuir](CONTRIBUTING.md) y la información sobre
[seguridad](SECURITY.md). El proyecto usa la [licencia MIT](LICENSE).
