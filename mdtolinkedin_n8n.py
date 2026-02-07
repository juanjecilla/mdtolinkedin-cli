#!/usr/bin/env python3
"""
Markdown to LinkedIn Converter - n8n Optimized Version
Lightweight version optimized for n8n workflows.

This version uses minimal dependencies and is designed to work in n8n's Python environment.

Usage in n8n:
1. Copy this file to your n8n workflow
2. Use it in a Code node with:
   
   from mdtolinkedin_n8n import convert
   
   # Get markdown from previous node
   markdown = $input.item.json.markdown
   
   # Convert
   linkedin_text = convert(markdown)
   
   # Return result
   return [{"json": {"linkedin_text": linkedin_text, "char_count": len(linkedin_text)}}]
"""

import re
from typing import Tuple, Optional


def to_bold(text: str) -> str:
    """Convert ASCII letters to Mathematical Bold Unicode."""
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            result.append(chr(0x1D400 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            result.append(chr(0x1D41A + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


def to_italic(text: str) -> str:
    """Convert ASCII letters to Mathematical Italic Unicode."""
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            result.append(chr(0x1D434 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            result.append(chr(0x1D44E + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


def to_bold_italic(text: str) -> str:
    """Convert ASCII letters to Mathematical Bold Italic Unicode."""
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            result.append(chr(0x1D468 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            result.append(chr(0x1D482 + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


def convert(markdown_text: str, use_carbon: bool = False) -> str:
    """
    Convert Markdown to LinkedIn-compatible text.
    
    This is a pure Python implementation using regex, no external dependencies.
    Perfect for n8n workflows.
    
    Args:
        markdown_text: Input Markdown text
        use_carbon: If True, generate Carbon.now.sh URLs for code blocks
    
    Returns:
        LinkedIn-compatible text with Unicode formatting
    """
    if not markdown_text:
        return ""
    
    output = markdown_text
    
    # Code blocks → Remove or Carbon URL (do this first to avoid processing code)
    if use_carbon:
        output = re.sub(r'```[\s\S]*?```', '[Code image: carbon.now.sh]\n', output)
    else:
        output = re.sub(r'```[\s\S]*?```', '', output)
    
    # Headers → Bold (handle all levels)
    def header_replace(match):
        header_text = match.group(1).strip()
        return to_bold(header_text) + "\n\n"
    
    output = re.sub(r'^#{1,6}\s+(.+)$', header_replace, output, flags=re.MULTILINE)
    
    # **bold** → Bold (handle nested bold)
    output = re.sub(r'\*\*([^*]+?)\*\*', lambda m: to_bold(m.group(1)), output)
    
    # *italic* → Italic (but not **bold** or ***bold italic***)
    # Match single asterisks that are not part of double/triple asterisks
    output = re.sub(r'(?<!\*)\*([^*\n]+?)\*(?!\*)', lambda m: to_italic(m.group(1)), output)
    
    # ***bold italic*** → Bold Italic
    output = re.sub(r'\*\*\*([^*]+?)\*\*\*', lambda m: to_bold_italic(m.group(1)), output)
    
    # Lists → Bullet points (unordered lists)
    def list_item_replace(match):
        item_text = match.group(1).strip()
        # Process formatting within list items
        item_text = re.sub(r'\*\*([^*]+?)\*\*', lambda m: to_bold(m.group(1)), item_text)
        item_text = re.sub(r'(?<!\*)\*([^*\n]+?)\*(?!\*)', lambda m: to_italic(m.group(1)), item_text)
        return "• " + item_text + "\n"
    
    output = re.sub(r'^[-*]\s+(.+)$', list_item_replace, output, flags=re.MULTILINE)
    
    # Ordered lists → Preserve numbers
    def ordered_list_replace(match):
        item_text = match.group(2).strip()
        # Process formatting within list items
        item_text = re.sub(r'\*\*([^*]+?)\*\*', lambda m: to_bold(m.group(1)), item_text)
        item_text = re.sub(r'(?<!\*)\*([^*\n]+?)\*(?!\*)', lambda m: to_italic(m.group(1)), item_text)
        return match.group(1) + ". " + item_text + "\n"
    
    output = re.sub(r'^(\d+)\.\s+(.+)$', ordered_list_replace, output, flags=re.MULTILINE)
    
    # Blockquotes → Italic
    def blockquote_replace(match):
        quote_text = match.group(1).strip()
        # Process formatting within quotes
        quote_text = re.sub(r'\*\*([^*]+?)\*\*', lambda m: to_bold(m.group(1)), quote_text)
        quote_text = re.sub(r'(?<!\*)\*([^*\n]+?)\*(?!\*)', lambda m: to_italic(m.group(1)), quote_text)
        return to_italic(quote_text) + "\n"
    
    output = re.sub(r'^>\s+(.+)$', blockquote_replace, output, flags=re.MULTILINE)
    
    # Links → text (url)
    output = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'\1 (\2)', output)
    
    # Inline code → Just text (remove backticks)
    output = re.sub(r'`([^`]+)`', r'\1', output)
    
    # Clean up extra newlines (max 2 consecutive)
    output = re.sub(r'\n{3,}', '\n\n', output)
    
    # Remove trailing whitespace from lines
    output = '\n'.join(line.rstrip() for line in output.split('\n'))
    
    return output.strip()


def convert_with_warning(markdown_text: str, use_carbon: bool = False, warn_limit: int = 3000) -> dict:
    """
    Convert Markdown and return result with metadata.
    
    Perfect for n8n workflows - returns a dictionary with all info.
    
    Args:
        markdown_text: Input Markdown text
        use_carbon: If True, generate Carbon.now.sh URLs for code blocks
        warn_limit: Character limit for warning (default: 3000)
    
    Returns:
        Dictionary with:
        - text: Converted LinkedIn text
        - char_count: Character count
        - exceeds_limit: Boolean indicating if limit exceeded
        - warning: Warning message if limit exceeded, None otherwise
    """
    result = convert(markdown_text, use_carbon)
    char_count = len(result)
    exceeds_limit = char_count > warn_limit
    
    return {
        "text": result,
        "char_count": char_count,
        "exceeds_limit": exceeds_limit,
        "warning": f"⚠️  Warning: Output is {char_count} characters (LinkedIn limit: {warn_limit})" if exceeds_limit else None
    }


# Example n8n Code Node usage:
"""
# In n8n Code Node, use this:

from mdtolinkedin_n8n import convert_with_warning

# Get markdown from previous node
markdown = $input.item.json.markdown or $input.item.json.text

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
"""


if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        if sys.argv[1] == "-":
            markdown_input = sys.stdin.read()
        else:
            with open(sys.argv[1], 'r', encoding='utf-8') as f:
                markdown_input = f.read()
    else:   
        markdown_input = sys.stdin.read()
    
    use_carbon = "--carbon" in sys.argv
    result = convert_with_warning(markdown_input, use_carbon)
    
    if result["warning"]:
        print(result["warning"], file=sys.stderr)
    
    print(result["text"])
