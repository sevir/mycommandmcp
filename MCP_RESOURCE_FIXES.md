# MCP Resource Specification Fixes

## Issues Identified

The original implementation had several issues with the MCP resource specification:

1. **Wrong method name**: Used `resources/get` instead of `resources/read`
2. **Wrong parameter name**: Used `name` instead of `uri` for resource requests
3. **Incorrect response structure**: The response format didn't match the MCP specification
4. **Missing URI handling**: Resources weren't properly identified with URIs

## Changes Made

### 1. Fixed `resources/list` Response Format

**Before:**
```json
{
  "resources": [
    {
      "name": "resource_name",
      "description": "description",
      "inputSchema": {
        "type": "object",
        "properties": {}
      }
    }
  ]
}
```

**After:**
```json
{
  "resources": [
    {
      "uri": "file://resource_name",
      "name": "resource_name", 
      "description": "description",
      "mimeType": "application/json"
    }
  ]
}
```

### 2. Changed Method from `resources/get` to `resources/read`

Updated the method handler to use the correct MCP specification method name.

### 3. Updated Parameter Handling

**Before:** Used `name` parameter
```json
{
  "params": {
    "name": "resource_name"
  }
}
```

**After:** Uses `uri` parameter
```json
{
  "params": {
    "uri": "file://resource_name"
  }
}
```

### 4. Fixed Response Structure

**Before:**
```json
{
  "content": [
    {
      "type": "resource",
      "resource": {
        "uri": "data:mime/type;base64,content",
        "mimeType": "mime/type",
        "text": "content"
      }
    }
  ],
  "isError": false
}
```

**After:**
```json
{
  "contents": [
    {
      "uri": "file://resource_name",
      "mimeType": "mime/type",
      "text": "content"  // for text content
      // OR
      "blob": "base64_encoded_content"  // for binary content
    }
  ]
}
```

## Key Improvements

1. **Proper URI handling**: Resources are now identified by URIs (`file://resource_name`)
2. **Correct method names**: Uses `resources/read` instead of `resources/get`
3. **Specification-compliant responses**: Response format matches the official MCP specification
4. **Better MIME type detection**: Improved automatic MIME type detection for resources
5. **Cleaner binary/text handling**: Separate fields for text (`text`) and binary (`blob`) content

## Benefits

- **Standards compliance**: Now fully compliant with MCP 2024-11-05 specification
- **Better interoperability**: Works correctly with MCP clients expecting standard responses
- **Cleaner API**: More intuitive URI-based resource identification
- **Improved content handling**: Better separation of text and binary content

These changes ensure that the MCP server now correctly implements the resource specification and will work properly with standard MCP clients.