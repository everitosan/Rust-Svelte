# Rust-Svelte integration.

### Este proyecto está pensado como base para aplicaciones de escritorio dónde la lógica de la vista sea manejada por Svelte y las operaciones que requieran procesos más finos por rust.

## Front
Dentro del directorio rust puede encontrar un proyecto de svelte creado con:  
```
$ npx degit sveltejs/template front
```

## Rust
Rust se apoya de web-view por lo que primero deberá crearse un build de svelte y posteriormente correr:
```
$ cargo run
```

ó

```
$ cargo build
```
