sys READ r0 #Load in number to print
sys READ r1 #Load in 3 (instruction to jmp to)

sys WRITE r0

sys STACK PUSH r1
sys STACK POP pc
