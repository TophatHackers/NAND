Registers:
#0: PC - Program Counter
#1: RN - NAND Output
#2 - #7: r0 - r5: General use registers

8 bit instructionset:

2 bit - OPcode:
00 - NAND - RN = A NAND B
01 - STACK - PUSH/POP RS
10 - START - Identifies start of instruction chain - sets rs and rt to r0 and r1.
11 - END - Identifies end of instruction chain - sets rs to final output

S-Type (identified by OP = 01):
2 bit OP (identifier)
1 bit PUSH or POP (0 = PUSH, 1 = POP)
3 bit rs (input 1)
2 bit Padding (00)

I-type:
2 bit OP (identifier)
3 bit rs (input 1)
3 bit rt (input 2)