#! /usr/bin/env python3
import argparse
import pathlib

parser = argparse.ArgumentParser(description='Brain F**k interpreter.')
parser.add_argument('file', type=str)
args = parser.parse_args()

script = ''
with open(args.file, 'r') as f:
    script = f.read()

memory = [0]
point = 0
idx = 0
while idx < len(script):
    token = script[idx]
    if token == '>':
        point += 1
        if len(memory) < (point+1):
            memory.append(0)
    elif token == '<':
        point -= 1
        if point < 0:
            print('ぬるぽ!')
            exit(1)
    elif token == '+':
        memory[point] += 1
    elif token == '-':
        memory[point] -= 1
    elif token == '.':
        print(chr(memory[point]), end='')
    elif token == ',':
        memory[point] = input()[0].encode()[0]
    elif token == '[':
        if memory[point] == 0:
            roop = 1
            while roop > 0:
                idx += 1
                if script[idx] == ']':
                    roop -= 1
                elif script[idx] == '[':
                    roop += 1
                else:
                    pass
        else:
            pass
    elif token == ']':
        if memory[point] == 0:
            pass
        else:
            roop = 1
            while roop > 0:
                idx -= 1
                if script[idx] == '[':
                    roop -= 1
                elif script[idx] == ']':
                    roop += 1
                else:
                    pass
    else:
        pass
    idx += 1
