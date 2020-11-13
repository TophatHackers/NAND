.define AND r0 r1 rt
START r0 r1
NAND r0 r1
NAND rn rn
END rt
.end_define


.define OR r0 r1 rt
START r0 r1 
NAND r0 r0
sys STACK PUSH rn
NAND r1 r1
sys STACK POP r2
NAND rn r2
END rt
.end_define

.define NOR r0 r1 rt
START r0 r1
NAND r0 r0
STACK PUSH rn
NAND r1 r1 
sys STACK POP r2
NAND rn r2
NAND rn rn
.end_define

.define XOR r0 r1 rt
START r0 r1
NAND r0 r1
sys STACK PUSH rn
NAND r0 rn
sys STACK POP r2
sys STACK PUSH rn
NAND r1 r2
sys STACK POP r2
NAND r2 rn
END rt
.end_define

.define NOT r0 r1 rt
START r0 r1
NAND r0 r0
END rt
.end_define

.define XNOR r0 r1 rt
START r0 r1 
NAND r0 r0
sys STACK PUSH rn
NAND r1 r1
sys STACK POP r2
NAND r2 rn
sys STACK PUSH rn
NAND r1 r0 
sys STACK POP r2
NAND r2 rn
END rt
.end_define
