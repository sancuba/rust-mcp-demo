# Servidor de Magia MCP

Este proyecto implementa un servidor HTTP en Rust usando [Axum](https://github.com/tokio-rs/axum) que expone un endpoint para realizar una "operación mágica" (división entre dos números). Además, incluye documentación interactiva generada automáticamente con Swagger (OpenAPI).

## Endpoints

- **GET `/operacion_magica`**  
  Realiza una división entre dos números mágicos recibidos como parámetros de consulta (`primer_numero_magico` y `segundo_numero_magico`).  
  - Respuesta 200: Resultado de la división.
  - Respuesta 400: Error si el divisor es cero.

La documentación Swagger está disponible en:  
[http://localhost:8080/docs](http://localhost:8080/docs)

---

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (recomendado: versión estable actual)
- [Docker](https://docs.docker.com/get-docker/) (opcional, para contenedor)

---

## Compilación y ejecución local

1. **Clona el repositorio**  
   ```bash
   git clone <URL_DEL_REPOSITORIO>
   cd rust-mcp-demo
   ```

2. **Compila el proyecto**  
   ```bash
   cargo build --release
   ```

3. **Ejecuta el servidor**  
   ```bash
   cargo run
   ```
   El servidor estará disponible en [http://localhost:8080/docs](http://localhost:8080/docs)

---

## Construcción y ejecución con Docker

1. **Construye la imagen Docker**  
   ```bash
   docker build -t mcp-demo .
   ```

2. **Ejecuta el contenedor**  
   ```bash
   docker run -p 8080:8080 mcp-demo
   ```

   El servidor estará disponible en [http://localhost:8080/docs](http://localhost:8080/docs)

---

## Notas

- Si modificas el código fuente, recuerda reconstruir la imagen Docker para reflejar los cambios.
- El endpoint `/operacion_magica` retorna un error si intentas dividir por cero.

---

¡Listo! Ahora puedes usar y probar tu servidor de magia.
