#!/usr/bin/python3

import os
import subprocess
import sys

raw_options = subprocess.check_output('screech dump_options', shell=True)
lines = [line.split() for line in raw_options.decode().strip().split('\n')]

options = {line[0]:line[1::] for line in lines}
fzf_input = '\n'.join(options.keys()).encode()
out = subprocess.Popen(['fzf'], stdin=subprocess.PIPE, stdout=subprocess.PIPE).communicate(input=fzf_input)
option = out[0].strip().decode()

arguments = []
for parameter in options[option]:
    arguments.append(input(parameter + ' '))

for input_file in sys.argv[1::]:
    name = input_file[:-4:]
    option_suffix = 'd' if option[-1] == 'e' else 'ed'
    arguments_for_name =  '_' + '_'.join(arguments) if len(arguments) > 0 else ''
    output_file = name + '_' + option + option_suffix + arguments_for_name + '.wav'

    os.system(f'screech "{input_file}" {option} {" ".join(arguments)} "{output_file}"')
