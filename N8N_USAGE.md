# Using mdtolinkedin in n8n Workflows

This guide shows how to use the Python Markdown to LinkedIn converter in n8n workflows.

## Quick Start

### Option 1: Use the n8n-optimized version (Recommended)

The `mdtolinkedin_n8n.py` file is optimized for n8n with:
- ✅ No external dependencies (pure Python)
- ✅ Regex-based parsing (fast and lightweight)
- ✅ Returns structured data perfect for n8n
- ✅ Character limit checking built-in

### Option 2: Use the full-featured version

The `mdtolinkedin.py` file uses markdown parsers for more accurate parsing:
- Requires `mistune` or `markdown` library
- More accurate parsing (handles edge cases better)
- Event-based parsing (similar to Rust version)

## Setup in n8n

### Method 1: Code Node (Easiest)

1. **Add a Code Node** to your workflow
2. **Copy the entire `mdtolinkedin_n8n.py` file** into the Code Node
3. **Use it like this:**

```python
from mdtolinkedin_n8n import convert_with_warning

# Get markdown from previous node
markdown = $input.item.json.markdown or $input.item.json.text or ""

# Convert with warning
result = convert_with_warning(markdown, use_carbon=False)

# Return result for next node
return [{
    "json": {
        "linkedin_text": result["text"],
        "char_count": result["char_count"],
        "exceeds_limit": result["exceeds_limit"],
        "warning": result["warning"]
    }
}]
```

### Method 2: External Script (For Reusability)

1. **Upload `mdtolinkedin_n8n.py`** to your n8n server
2. **In Code Node, import it:**

```python
import sys
sys.path.append('/path/to/script')
from mdtolinkedin_n8n import convert_with_warning

# Use as above
markdown = $input.item.json.markdown
result = convert_with_warning(markdown)
return [{"json": result}]
```

## Example Workflow

### Simple Conversion

```
[Webhook] → [Code Node] → [LinkedIn Node]
```

**Code Node:**
```python
from mdtolinkedin_n8n import convert

markdown = $input.item.json.markdown
linkedin_text = convert(markdown)

return [{"json": {"text": linkedin_text}}]
```

### With Character Limit Check

```
[Webhook] → [Code Node] → [IF Node] → [LinkedIn Node]
                              ↓
                         [Notification]
```

**Code Node:**
```python
from mdtolinkedin_n8n import convert_with_warning

markdown = $input.item.json.markdown
result = convert_with_warning(markdown)

return [{
    "json": {
        "linkedin_text": result["text"],
        "char_count": result["char_count"],
        "exceeds_limit": result["exceeds_limit"],
        "warning": result["warning"]
    }
}]
```

**IF Node:**
- Condition: `exceeds_limit` equals `true`
- If true → Send notification
- If false → Continue to LinkedIn

## API Reference

### `convert(markdown_text: str, use_carbon: bool = False) -> str`

Convert Markdown to LinkedIn text.

**Parameters:**
- `markdown_text`: Input Markdown string
- `use_carbon`: If True, generate Carbon.now.sh URLs for code blocks

**Returns:**
- LinkedIn-compatible text string

**Example:**
```python
from mdtolinkedin_n8n import convert

text = convert("**Hello** *world*")
# Returns: "𝐇𝐞𝐥𝐥𝐨 𝘸𝘰𝘳𝘭𝘥"
```

### `convert_with_warning(markdown_text: str, use_carbon: bool = False, warn_limit: int = 3000) -> dict`

Convert Markdown and return result with metadata.

**Parameters:**
- `markdown_text`: Input Markdown string
- `use_carbon`: If True, generate Carbon.now.sh URLs for code blocks
- `warn_limit`: Character limit for warning (default: 3000)

**Returns:**
- Dictionary with:
  - `text`: Converted LinkedIn text
  - `char_count`: Number of characters
  - `exceeds_limit`: Boolean (True if exceeds limit)
  - `warning`: Warning message or None

**Example:**
```python
from mdtolinkedin_n8n import convert_with_warning

result = convert_with_warning("**Long text...**")
print(result["text"])        # LinkedIn text
print(result["char_count"])  # Character count
print(result["exceeds_limit"])  # True/False
if result["warning"]:
    print(result["warning"])  # Warning message
```

## Supported Markdown Features

| Markdown | LinkedIn Output |
|----------|----------------|
| `# Header` | Bold text |
| `**bold**` | Unicode bold characters |
| `*italic*` | Unicode italic characters |
| `***bold italic***` | Unicode bold italic characters |
| `- item` | `• item` (bullet) |
| `1. item` | `1. item` (preserved) |
| `> quote` | Italic text |
| `[text](url)` | `text (url)` |
| `` `code` `` | Plain text (backticks removed) |
| ``````` code ``````` | Removed (or Carbon URL with `use_carbon=True`) |

## Tips for n8n

1. **Error Handling:**
```python
try:
    from mdtolinkedin_n8n import convert_with_warning
    result = convert_with_warning($input.item.json.markdown)
    return [{"json": result}]
except Exception as e:
    return [{"json": {"error": str(e)}}]
```

2. **Handle Empty Input:**
```python
markdown = $input.item.json.markdown or ""
if not markdown:
    return [{"json": {"error": "No markdown input provided"}}]
```

3. **Use Carbon for Code Blocks:**
```python
result = convert_with_warning(markdown, use_carbon=True)
```

4. **Custom Character Limit:**
```python
result = convert_with_warning(markdown, warn_limit=2000)  # Custom limit
```

## Troubleshooting

### Import Error
If you get import errors, make sure you've copied the entire `mdtolinkedin_n8n.py` file into the Code Node.

### Character Encoding Issues
The script handles Unicode correctly. If you see encoding issues in n8n, ensure your workflow nodes are set to UTF-8.

### Performance
The regex-based parser is fast for most use cases. For very large documents (>10k characters), consider processing in chunks.

## Example: Complete Workflow

```python
# n8n Code Node
from mdtolinkedin_n8n import convert_with_warning

# Get input
markdown = $input.item.json.markdown or $input.item.json.text or ""

# Validate input
if not markdown.strip():
    return [{"json": {"error": "No markdown provided"}}]

# Convert
try:
    result = convert_with_warning(markdown, use_carbon=False)
    
    # Prepare output
    output = {
        "linkedin_text": result["text"],
        "char_count": result["char_count"],
        "exceeds_limit": result["exceeds_limit"],
        "success": True
    }
    
    # Add warning if needed
    if result["warning"]:
        output["warning"] = result["warning"]
    
    return [{"json": output}]
    
except Exception as e:
    return [{"json": {
        "error": str(e),
        "success": False
    }}]
```

## Next Steps

1. Test the conversion with sample Markdown
2. Set up your LinkedIn posting workflow
3. Add character limit checks
4. Configure notifications for warnings

For more examples, see the test cases in the Rust version's test files.
