# Instrucciones de Uso - MyCommandMCP

## ✅ Estado del Proyecto
El proyecto **MyCommandMCP** se ha compilado exitosamente y está funcionando correctamente con soporte para archivos de configuración personalizables.

## 🚀 Cómo ejecutar el servidor

### Compilar el proyecto
```bash
cargo build --release
```

### Ejecutar el servidor

#### Con configuración por defecto
```bash
./target/release/mycommandmcp
```

#### Con archivo de configuración personalizado
```bash
./target/release/mycommandmcp --config mi-config.yaml
```

#### Ver opciones disponibles
```bash
./target/release/mycommandmcp --help
```

El servidor escucha por stdin y responde por stdout siguiendo el protocolo MCP.

## 🧪 Probar el servidor

### Prueba rápida con el script incluido

#### Con configuración por defecto
```bash
./simple_test.sh
```

#### Con configuración extendida
```bash
./simple_test.sh mycommand-tools-extended.yaml
```

### Pruebas manuales

#### 1. Inicializar el servidor (configuración por defecto)
```bash
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mycommandmcp
```

#### 2. Inicializar con configuración específica
```bash
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mycommandmcp --config mycommand-tools-extended.yaml
```

#### 3. Listar herramientas disponibles
```bash
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | ./target/release/mycommandmcp --config mi-config.yaml
```

#### 4. Ejecutar una herramienta sin argumentos
```bash
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_date", "arguments": {}}}' | ./target/release/mycommandmcp --config mi-config.yaml
```

#### 5. Ejecutar una herramienta con argumentos
```bash
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "-la"}}}' | ./target/release/mycommandmcp --config mi-config.yaml
```

## 📋 Archivos de configuración disponibles

### Configuración básica (`mycommand-tools.yaml`)
- **5 herramientas**: list_files, get_date, disk_usage, process_list, network_info

### Configuración extendida (`mycommand-tools-extended.yaml`) 
- **12 herramientas**: Incluye las básicas más:
  - current_directory, memory_info, network_interfaces, ping_host
  - file_content, file_info, find_files, grep_text

## 🔧 Personalizar herramientas

Edita cualquier archivo YAML o crea uno nuevo:

```yaml
tools:
  - name: "mi_herramienta"
    description: "Descripción para MCP"
    command: "comando_sistema"
    path: "/ruta/ejecucion"
    accepts_args: true
```

Luego ejecuta:
```bash
./target/release/mycommandmcp --config mi-archivo.yaml
```

## 📊 Formato de respuesta

Cada ejecución de herramienta devuelve:

```json
{
  "status_code": 0,
  "output": "salida del comando",
  "error": "errores si los hay"
}
```

## ⚠️ Seguridad

- Solo usar en entornos controlados
- Revisar cuidadosamente las herramientas configuradas
- Los comandos se ejecutan con los permisos del usuario que ejecuta el servidor

## 📁 Estructura del proyecto

```
mycommandmcp/
├── src/
│   └── main.rs                    # Código principal del servidor
├── Cargo.toml                     # Configuración de Rust
├── mycommand-tools.yaml           # Configuración básica (5 herramientas)
├── mycommand-tools-extended.yaml  # Configuración extendida (12 herramientas)
├── simple_test.sh                 # Script de prueba
├── demo.sh                        # Script de demostración
├── test_server.sh                 # Script de prueba avanzado
├── README.md                      # Documentación
└── USAGE.md                       # Este archivo
```

## 🎯 Nuevas características

✅ **Parámetro --config**: Especifica cualquier archivo YAML de configuración
✅ **Configuraciones múltiples**: Básica y extendida incluidas
✅ **Scripts actualizados**: Soporte para archivos de configuración personalizados

¡El servidor MCP está listo para usar con configuraciones flexibles! 🎉
