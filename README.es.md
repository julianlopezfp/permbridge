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

> **Estado actual:** Este repositorio sigue siendo un esqueleto de Rust. No
> carga políticas YAML, importa configuraciones de agentes, compara posturas,
> genera diagnósticos, intercepta acciones, media aprobaciones ni se integra
> con VS Code. No hay adaptadores implementados. Actualmente no proporciona
> protección de seguridad.

## Dirección del producto

El flujo previsto es:

```text
Política deseada -> modelo canónico -> adaptador -> postura efectiva
                 -> comparación -> diagnósticos -> informe
```

El modelo canónico expresará los permisos deseados sin depender de un agente.
Los adaptadores interpretarán las configuraciones nativas que admitan y
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
- Un tipo `Decision`, independiente de la interfaz, con los valores `Allow`,
  `Ask` y `Deny`, valor predeterminado `Ask` y orden según la restricción.
- Pruebas unitarias de ese tipo, CI de Rust y documentación del modelo previsto.
- Ejemplos YAML preliminares y un directorio reservado para una futura
  extensión TypeScript de VS Code. El código no lee ni valida los ejemplos.

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

El modelo previsto incluye una política global del usuario y otra opcional
del proyecto. La política del proyecto podrá añadir restricciones, pero no
reducir las globales. Las decisiones canónicas pueden ser `ALLOW`, `ASK` y
`DENY`, con precedencia `DENY > ASK > ALLOW`; las acciones deseadas desconocidas
se tratarán como `ASK`. Los adaptadores no deberán suponer que todos los
agentes tienen estados de permisos idénticos.

Los próximos hitos son un esquema YAML versionado y su cargador, un modelo
canónico de capacidades, un primer adaptador probado, la comparación de
posturas y diagnósticos útiles en el CLI. Más adaptadores y una experiencia
localizada en VS Code llegarán después. Ninguno de esos hitos está
implementado en este cambio.

## Estructura del repositorio

| Ruta | Propósito |
| --- | --- |
| `crates/permbridge-core/` | Tipos Rust canónicos; futura lógica de comparación |
| `crates/permbridge-cli/` | Programa básico Rust; futuro CLI para usuarios |
| `adapters/` | Futuros adaptadores específicos; aún sin integraciones |
| `extensions/vscode/` | Directorio reservado para la futura extensión TypeScript |
| `policies/examples/` | Ejemplos YAML preliminares sin funcionalidad |
| `docs/` | Documentación técnica canónica en inglés |
| `tests/` | Directorio reservado para futuras pruebas de integración |
| `.github/workflows/` | CI de Rust |

La [arquitectura](docs/architecture.md) y los
[conceptos de política](docs/policy-model.md) se documentan en inglés. Consulta
también las normas para [contribuir](CONTRIBUTING.md) y la información sobre
[seguridad](SECURITY.md). El proyecto usa la [licencia MIT](LICENSE).
