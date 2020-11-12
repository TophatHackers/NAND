.define AND r0 r1 rt
START r0 r1
NAND r0 r1
NAND rn rn
END rt
.end_define


.define OR r0 r1 rt
START r0 r1 
NAND r0 r0
STACK PUSH rn
NAND r1 r1
STACK POP r2
NAND rn r2
END rt
.end_define

.define NOR r0 r1 rt
START r0 r1
NAND r0 r0
STACK PUSH rn
NAND r1 r1 
STACK POP r2
NAND rn r2
NAND rn rn
.end_define
