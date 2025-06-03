# Usamos 'scratch' como imagen base, lo que significa un contenedor vacío
FROM scratch

# Copiamos el binario de la aplicación ya compilada
COPY ./mcp-demo /app/mi.bin

# Expone el puerto donde esta el servicio
EXPOSE 8080

# Definimos el comando para ejecutar la aplicación
ENTRYPOINT ["/app/mi.bin"]


# Para una exitosa compilacion para 'scratch'
#cargo build --release --target x86_64-unknown-linux-musl
