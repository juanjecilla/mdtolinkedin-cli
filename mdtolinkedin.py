#!/usr/bin/env python3
"""
Markdown to LinkedIn Converter (Python version)
Converts Markdown to LinkedIn-compatible text using Unicode formatting.

Usage:
    from mdtolinkedin import convert
    
    linkedin_text = convert("**bold** text")
    print(linkedin_text)

For n8n:
    Import this module and use the convert() function in your workflow.
"""

try:
    import mistune
    HAS_MISTUNE = True
except ImportError:
    HAS_MISTUNE = False
    try:
        import markdown
        HAS_MARKDOWN = True
    except ImportError:
        HAS_MARKDOWN = False


# Unicode conversion functions
def to_bold(text: str) -> str:
    """
    Convert ASCII letters to Mathematical Bold Unicode.
    
    Unicode ranges:
    - Uppercase: U+1D400 – U+1D419 (A-Z)
    - Lowercase: U+1D41A – U+1D433 (a-z)
    """
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            # Mathematical Bold uppercase: 0x1D400 + (char - 'A')
            result.append(chr(0x1D400 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            # Mathematical Bold lowercase: 0x1D41A + (char - 'a')
            result.append(chr(0x1D41A + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


def to_italic(text: str) -> str:
    """
    Convert ASCII letters to Mathematical Italic Unicode.
    
    Unicode ranges:
    - Uppercase: U+1D434 – U+1D44D (A-Z)
    - Lowercase: U+1D44E – U+1D467 (a-z)
    """
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            # Mathematical Italic uppercase: 0x1D434 + (char - 'A')
            result.append(chr(0x1D434 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            # Mathematical Italic lowercase: 0x1D44E + (char - 'a')
            result.append(chr(0x1D44E + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


def to_bold_italic(text: str) -> str:
    """
    Convert ASCII letters to Mathematical Bold Italic Unicode.
    
    Unicode ranges:
    - Uppercase: U+1D468 – U+1D481 (A-Z)
    - Lowercase: U+1D482 – U+1D49B (a-z)
    """
    result = []
    for char in text:
        if 'A' <= char <= 'Z':
            # Mathematical Bold Italic uppercase: 0x1D468 + (char - 'A')
            result.append(chr(0x1D468 + (ord(char) - ord('A'))))
        elif 'a' <= char <= 'z':
            # Mathematical Bold Italic lowercase: 0x1D482 + (char - 'a')
            result.append(chr(0x1D482 + (ord(char) - ord('a'))))
        else:
            result.append(char)
    return ''.join(result)


# Style stack for nested formatting
class TextStyle:
    NORMAL = 0
    BOLD = 1
    ITALIC = 2
    BOLD_ITALIC = 3


def apply_style(text: str, style: int) -> str:
    """Apply text style transformation."""
    if style == TextStyle.BOLD:
        return to_bold(text)
    elif style == TextStyle.ITALIC:
        return to_italic(text)
    elif style == TextStyle.BOLD_ITALIC:
        return to_bold_italic(text)
    else:
        return text


def convert_mistune(markdown_text: str, use_carbon: bool = False) -> str:
    """Convert using mistune (event-based parser, similar to pulldown-cmark)."""
    output = []
    style_stack = [TextStyle.NORMAL]
    pending_link_url = None
    in_code_block = False
    code_block_content = []
    
    # Create a custom renderer
    class LinkedInRenderer(mistune.HTMLRenderer):
        def __init__(self):
            super().__init__()
            self.output = []
            self.style_stack = [TextStyle.NORMAL]
            self.pending_link_url = None
            self.in_code_block = False
            self.code_block_content = []
        
        def heading(self, text, level, **attrs):
            # Headings → Bold
            styled = to_bold(text)
            return styled + "\n\n"
        
        def strong(self, text):
            # Strong → Bold
            return to_bold(text)
        
        def emphasis(self, text):
            # Emphasis → Italic
            return to_italic(text)
        
        def list(self, text, ordered, **attrs):
            # Lists are handled by list_item
            return text
        
        def list_item(self, text, **attrs):
            # Lists → Bullet points
            return "• " + text + "\n"
        
        def block_quote(self, text):
            # Blockquotes → Italic
            return to_italic(text) + "\n"
        
        def link(self, text, url, title=None):
            # Links → text (url)
            return f"{text} ({url})"
        
        def codespan(self, text):
            # Inline code → Just text
            return text
        
        def code(self, text, lang=None):
            # Code blocks → Remove or Carbon URL
            if use_carbon:
                return "[Code image: carbon.now.sh]\n"
            return ""  # Skip code block entirely
        
        def paragraph(self, text):
            return text + "\n\n"
        
        def linebreak(self):
            return "\n"
        
        def thematic_break(self):
            return "\n"
    
    renderer = LinkedInRenderer()
    md = mistune.create_markdown(renderer=renderer)
    result = md(markdown_text)
    
    return result.strip()


def convert_markdown_lib(markdown_text: str, use_carbon: bool = False) -> str:
    """Convert using markdown library (alternative implementation)."""
    # This is a simpler implementation using markdown library
    # It's less precise than mistune but works with standard library
    
    import re
    
    output = markdown_text
    
    # Headers → Bold
    output = re.sub(r'^#+\s+(.+)$', lambda m: to_bold(m.group(1)) + "\n\n", output, flags=re.MULTILINE)
    
    # **bold** → Bold
    output = re.sub(r'\*\*(.+?)\*\*', lambda m: to_bold(m.group(1)), output)
    
    # *italic* → Italic (but not **bold**)
    output = re.sub(r'(?<!\*)\*([^*]+?)\*(?!\*)', lambda m: to_italic(m.group(1)), output)
    
    # Lists → Bullet points
    output = re.sub(r'^[-*]\s+(.+)$', r'• \1\n', output, flags=re.MULTILINE)
    output = re.sub(r'^\d+\.\s+(.+)$', r'\1\n', output, flags=re.MULTILINE)
    
    # Blockquotes → Italic
    output = re.sub(r'^>\s+(.+)$', lambda m: to_italic(m.group(1)) + "\n", output, flags=re.MULTILINE)
    
    # Links → text (url)
    output = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'\1 (\2)', output)
    
    # Inline code → Just text
    output = re.sub(r'`([^`]+)`', r'\1', output)
    
    # Code blocks → Remove or Carbon URL
    if use_carbon:
        output = re.sub(r'```[\s\S]*?```', '[Code image: carbon.now.sh]\n', output)
    else:
        output = re.sub(r'```[\s\S]*?```', '', output)
    
    return output.strip()


def convert(markdown_text: str, use_carbon: bool = False) -> str:
    """
    Convert Markdown to LinkedIn-compatible text.
    
    Args:
        markdown_text: Input Markdown text
        use_carbon: If True, generate Carbon.now.sh URLs for code blocks
    
    Returns:
        LinkedIn-compatible text with Unicode formatting
    
    Examples:
        >>> convert("**bold** text")
        '𝐛𝐨𝐥𝐝 text'
        
        >>> convert("*italic* text")
        '𝘪𝘵𝘢𝘭𝘪𝘤 text'
        
        >>> convert("# Header")
        '𝐇𝐞𝐚𝐝𝐞𝐫'
    """
    if HAS_MISTUNE:
        return convert_mistune(markdown_text, use_carbon)
    elif HAS_MARKDOWN:
        return convert_markdown_lib(markdown_text, use_carbon)
    else:
        raise ImportError(
            "No markdown parser available. Please install one:\n"
            "  pip install mistune  # Recommended (event-based, similar to Rust version)\n"
            "  # OR\n"
            "  pip install markdown  # Alternative"
        )


def convert_with_warning(markdown_text: str, use_carbon: bool = False, warn_limit: int = 3000) -> tuple[str, bool]:
    """
    Convert Markdown and check character limit.
    
    Args:
        markdown_text: Input Markdown text
        use_carbon: If True, generate Carbon.now.sh URLs for code blocks
        warn_limit: Character limit for warning (default: 3000)
    
    Returns:
        Tuple of (linkedin_text, exceeds_limit)
    """
    result = convert(markdown_text, use_carbon)
    char_count = len(result)
    exceeds_limit = char_count > warn_limit
    return result, exceeds_limit


# CLI interface for standalone usage
if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        # Read from file or stdin
        if sys.argv[1] == "-":
            markdown_input = sys.stdin.read()
        else:
            with open(sys.argv[1], 'r', encoding='utf-8') as f:
                markdown_input = f.read()
    else:
        markdown_input = sys.stdin.read()
    
    use_carbon = "--carbon" in sys.argv
    result, exceeds_limit = convert_with_warning(markdown_input, use_carbon)
    
    if exceeds_limit:
        print(f"⚠️  Warning: Output is {len(result)} characters (LinkedIn limit: 3000)", file=sys.stderr)
    
    print(result)
