#!/usr/bin/env python3
"""
FlowLang Code Formatter
A simple code formatter for FlowLang files.

Usage:
    python flowfmt.py <file.flow>
    python flowfmt.py --check <file.flow>  # Check if file is formatted
    python flowfmt.py --stdin              # Format from stdin
"""

import sys
import re
import argparse
from typing import List

class FlowFormatter:
    def __init__(self):
        self.indent_level = 0
        self.indent_size = 4
        
    def format_code(self, code: str) -> str:
        """Format FlowLang code."""
        lines = code.split('\n')
        formatted_lines = []
        
        for line in lines:
            formatted_line = self._format_line(line.strip())
            if formatted_line:  # Skip empty lines processing
                formatted_lines.append(formatted_line)
            else:
                formatted_lines.append('')  # Preserve empty lines
                
        return '\n'.join(formatted_lines)
    
    def _format_line(self, line: str) -> str:
        """Format a single line."""
        if not line or line.startswith('#'):
            return line  # Don't indent comments or empty lines
            
        # Decrease indent for end, else
        if re.match(r'^(end|else)\b', line):
            self.indent_level = max(0, self.indent_level - 1)
            
        # Apply current indentation
        formatted = ' ' * (self.indent_level * self.indent_size) + line
        
        # Increase indent after do, then
        if re.search(r'\b(do|then)\s*$', line):
            self.indent_level += 1
        # Special case for else - it should be at same level as if
        elif line.startswith('else'):
            self.indent_level += 1
            
        return formatted
    
    def _add_spacing_around_operators(self, line: str) -> str:
        """Add proper spacing around operators."""
        # Add spaces around operators, but be careful with strings
        operators = ['==', '!=', '<=', '>=', '<', '>', '+', '-', '*', '/', '%', '=']
        
        # Simple approach - add spaces around operators not in strings
        # This is a basic implementation and could be improved
        for op in operators:
            if '"' not in line:  # Simple check to avoid operators in strings
                line = re.sub(f'\\s*{re.escape(op)}\\s*', f' {op} ', line)
                
        return line

def main():
    parser = argparse.ArgumentParser(description='Format FlowLang code')
    parser.add_argument('file', nargs='?', help='FlowLang file to format')
    parser.add_argument('--check', action='store_true', help='Check if file is formatted')
    parser.add_argument('--stdin', action='store_true', help='Read from stdin')
    parser.add_argument('--output', '-o', help='Output file (default: overwrite input)')
    
    args = parser.parse_args()
    
    formatter = FlowFormatter()
    
    if args.stdin:
        code = sys.stdin.read()
        formatted = formatter.format_code(code)
        print(formatted, end='')
    elif args.file:
        try:
            with open(args.file, 'r') as f:
                code = f.read()
                
            formatted = formatter.format_code(code)
            
            if args.check:
                if code == formatted:
                    print(f"{args.file} is already formatted")
                    sys.exit(0)
                else:
                    print(f"{args.file} needs formatting")
                    sys.exit(1)
            else:
                output_file = args.output or args.file
                with open(output_file, 'w') as f:
                    f.write(formatted)
                print(f"Formatted {args.file}")
                
        except FileNotFoundError:
            print(f"Error: File {args.file} not found")
            sys.exit(1)
        except Exception as e:
            print(f"Error: {e}")
            sys.exit(1)
    else:
        parser.print_help()
        sys.exit(1)

if __name__ == '__main__':
    main()