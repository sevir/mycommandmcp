# Content Types and File Response Support

## Overview

This version of MyCommandMCP now supports returning command outputs as different content types with proper content disposition headers. This allows tools to return binary files, images, documents, and other non-text content.

## Configuration

Add the following optional fields to your tool configuration in `mycommand-tools.yaml`:

```yaml
tools:
  - name: "your_tool_name"
    # ... existing configuration ...
    content_type: "application/pdf"          # MIME type of the output
    content_disposition: "attachment; filename=report.pdf"  # How the content should be handled
```

### Supported Fields

- **content_type** (optional): MIME type of the command output
  - Examples: `image/png`, `application/pdf`, `application/gzip`, `text/csv`
  - When not specified, output is treated as plain text
  
- **content_disposition** (optional): How the content should be handled by the client
  - Examples: `attachment; filename=document.pdf`, `inline; filename=image.png`

## Behavior

### Text Content
When no `content_type` is specified or when `content_type` starts with `text/`, `application/json`, or `application/xml`:
- Output is returned as plain text
- Existing behavior is maintained

### Binary Content
When `content_type` is specified and indicates binary content:
- Command output is automatically encoded as base64
- Response includes proper content type and disposition headers
- Client can decode and handle the file appropriately

## Examples

### PDF Generation
```yaml
- name: "generate_report"
  description: "Generate a PDF report"
  command: "wkhtmltopdf"
  path: "/"
  accepts_args: true
  accept_input: false
  default_args: "- /dev/stdout"
  content_type: "application/pdf"
  content_disposition: "attachment; filename=report.pdf"
```

### Image Capture
```yaml
- name: "screenshot"
  description: "Take a screenshot"
  command: "scrot"
  path: "/"
  accepts_args: true
  accept_input: false
  default_args: "-o /dev/stdout"
  content_type: "image/png"
  content_disposition: "attachment; filename=screenshot.png"
```

### Archive Creation
```yaml
- name: "create_backup"
  description: "Create a compressed backup"
  command: "tar"
  path: "/"
  accepts_args: true
  accept_input: false
  default_args: "-czf - ."
  content_type: "application/gzip"
  content_disposition: "attachment; filename=backup.tar.gz"
```

## Response Format

### Text Response (existing behavior)
```json
{
  "content": [{
    "type": "text",
    "text": "command output as string"
  }],
  "isError": false
}
```

### File Response (new behavior)
```json
{
  "content": [{
    "type": "resource",
    "resource": {
      "uri": "data:application/pdf;base64,JVBERi0xLjQK...",
      "mimeType": "application/pdf",
      "contentDisposition": "attachment; filename=report.pdf"
    }
  }],
  "isError": false
}
```

## Implementation Details

1. **Binary Detection**: Content is considered binary if `content_type` doesn't start with `text/` and isn't `application/json` or `application/xml`

2. **Base64 Encoding**: Binary output is automatically encoded as base64 for safe JSON transport

3. **Backward Compatibility**: Tools without `content_type` specification continue to work as before

4. **Error Handling**: If a command fails, the error response format remains unchanged regardless of content type
