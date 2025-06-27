# MyCommandMCP

Un servidor MCP (Model Context Protocol) escrito en Rust que permite ejecutar comandos de sistema como herramientas MCP.

## Características

- Lee la configuración de herramientas desde un archivo YAML
- Ejecuta comandos de sistema de forma segura
- Devuelve resultados en formato JSON con código de estado, salida y errores
- Compatible con el protocolo MCP 2024-11-05

## Configuración

El servidor lee la configuración desde el archivo `mycommand-tools.yaml` en el directorio actual. La estructura del archivo es:

```yaml
tools:
  - name: "nombre_herramienta"
    description: "Descripción de la herramienta para MCP"
    command: "comando_sistema"
    path: "/ruta/donde/ejecutar"
    accepts_args: true/false
```

### Ejemplo de configuración

```yaml
tools:
  - name: "list_files"
    description: "Lista los archivos en un directorio específico"
    command: "ls"
    path: "/"
    accepts_args: true
    
  - name: "get_date"
    description: "Obtiene la fecha y hora actual del sistema"
    command: "date"
    path: "/"
    accepts_args: false
```

## Instalación y uso

1. Asegúrate de tener Rust instalado
2. Clona o copia este proyecto
3. Configura tu archivo `mycommand-tools.yaml`
4. Compila y ejecuta:

```bash
cargo build --release
cargo run
```

## Protocolo MCP

El servidor implementa los siguientes métodos MCP:

- `initialize`: Inicializa el servidor y devuelve las capacidades
- `tools/list`: Lista todas las herramientas disponibles
- `tools/call`: Ejecuta una herramienta específica

### Formato de respuesta

Cuando se ejecuta una herramienta, el servidor devuelve un JSON con:

```json
{
  "status_code": 0,
  "output": "salida del comando",
  "error": "errores si los hay"
}
```

## Seguridad

**IMPORTANTE**: Este servidor ejecuta comandos de sistema directamente. Asegúrate de:

- Usar solo en entornos controlados
- Configurar herramientas con comandos seguros
- No exponer el servidor a redes no confiables
- Revisar cuidadosamente la configuración YAML

## Licencia

MIT License
