from ast import dump, parse

code = """
name = 1 + 1 * 1 + 8 * 1
"""

parsed = parse(code)
print(dump(parsed, indent=4))