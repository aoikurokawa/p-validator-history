# Q-Validator History

Validator History program migrated from Anchor to [Quasar](https://github.com/blueshift-gg/quasar).

## Benchmarks

Compute unit comparison between Anchor and Quasar implementations:

| Instruction | Anchor (CU) | Quasar (CU) | Reduction |
|---|---|---|---|
| `initialize_validator_history_account` | 7,647 | 2,277 | **70%** |
| `realloc_validator_history_account` | 9,480 | 2,798 | **70%** |

## Getting started

### Build

```bash
quasar build
```

### Deploy

```bash
quasar deploy
```
