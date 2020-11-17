sys READ r0 #Load in number to print
sys READ r1 #Load in 6(-2) (instruction to jmp to)
sys READ rn #Load in 18(-2) (exit instr) #9845
sys READ r2 #Set to 1
sys READ r3 #How many times to print
sys WRITE r0
add r4 r4 r2
IsLessThan r5 r4 r3
multiplyLSB r5
sys STACK PUSH r0
NOT r0 r5
AND r5 r5 r1 # r5 = jmp1*(true/false)
AND r0 r0 rn # r0 = jmp2*!(true/false)
add r5 r5 r0 # r5 = jmp1*(true/false) + jmp2*!(true/false)
sys STACK POP r0
MOVE pc r5 #Only do this if r4 (counter) < r3, meaning IsLessThan r5 r4 r3, r5 is 1
EOF rn