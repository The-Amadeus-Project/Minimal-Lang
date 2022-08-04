import subprocess
import sys
import os

# nasm -f elf64 -gdwarf printftest.asm && gcc -o printftest printftest.o -no-pie && ./printftest


a = sys.argv
if len(a) != 2:
    print("Expected 1 argument, which is the file name with no extensions")
    exit(1)

first = a[1]
if f"{first}.asm" not in os.listdir():
    print(f"{first}.asm doesnt exists")
    exit(1)

assemble = subprocess.run(["nasm", "-f", "elf64", "-g", "-F", "dwarf", f"{first}.asm"])
if assemble.returncode:
    print(assemble.stderr)
    exit(1)
subprocess.run(["gcc", f"{first}.o", "-o", f"{first}.out", "-no-pie"])
ret = subprocess.run([f"./{first}.out"])
if ret.returncode:
    print(f"\nexited with {ret.returncode}")
