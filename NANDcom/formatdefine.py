#!/usr/bin/env python3

f = open("define.asm", "r").readlines()

linearray = list()
for line in f:
    linearray.append(line)

i = 0
while i < len(linearray):
    if ".define" in linearray[i]:
        linearray.insert(i, "#|----------\n")
        i+=1
    i+=1

f = open("defineformatted.asm", "w")

f.writelines(linearray)
f.close()