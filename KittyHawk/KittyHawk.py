#!/usr/bin/env python3.5
import sys, os, argparse
from ctypes import *

dir_path = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
optimize = 1  # Tree-walk by default

versionf = open(dir_path+"/info/version", 'r')
version = versionf.read()
versionf.close()

wrightlib = cdll.LoadLibrary(dir_path+"/wright/target/release/libwright.so")

parser = argparse.ArgumentParser()
compile_style = parser.add_mutually_exclusive_group()
parser.add_argument('-v', '--version', help="print version information and exit.", action='store_true')
parser.add_argument('-V', '--version-number', help="print version number and exit.", action='store_true')
parser.add_argument('-i', '--interactive', help="run BlackRose interactively", action='store_true')
compile_style.add_argument('-I', '--interpret', help="Interprets and runs via a tree walk interpreter.", action='store_true')
parser.add_argument('file', nargs='?', default=None, help="BlackRose file to use")
# Todo: Implement as a non-tree-walk interpreter. Until then: Just Tree-Walk
args = parser.parse_args()

if args.version:
    print("[ Wright programming language ]\nVersion: "+version)
    sys.exit(0)
elif args.version_number:
    print(version)
    sys.exit(0)
elif args.interactive:
    wrightlib.start_prompt()
    sys.exit(0)
elif args.interpret:
    optimize = 1

if optimize == 1:
    if args.file == None:
        sys.exit("file is required for interpreter.")
    else:
        if os.path.isfile(args.file):
            wrightfile = open(args.file, 'r')
            wrightlib.run_file(optimize, wrightfile.read().encode("utf-8"), os.path.abspath(args.file).encode("utf-8"))
            wrightfile.close()
        else:
            sys.exit("{} is not a file.".format(args.file))