
#|----------| rs = rt
.define MOVE rs rt 
sys STACK PUSH rt
sys STACK POP rs
.end_define

#|----------| rt = 1
.define load1 rt
START r0 r1
NOT r2 r2 
MOVE r4 rn
MOVE r5 r2
BIT WRITE 0
MOVE rn r4
END rt
.end_define

#|----------| rt = rs0 - rs1
.define subtract rt rs0 rs1  
START rs0 rs1
NOT r1 r1

NOT r2 r2 
MOVE r4 r3
MOVE r5 r2
BIT WRITE 0
MOVE r3 r4

add r1 r1 r3

add rn r0 r1
END rt
.end_define

#|----------| rs = rt & rd
.define AND rs rt rd
START rt rd
NAND rt rd
NAND rn rn
END rs
.end_define

#|----------| rs = rt | rd
.define OR rs rt rd
START rt rd 
NAND rt rt
sys STACK PUSH rn
NAND rd rd
sys STACK POP r2
NAND rn r2
END rs
.end_define

#|----------| rs = !(rt | rd)
.define NOR rs rt rd 
START rt rd
NAND rt rt
STACK PUSH rn
NAND rd rd 
sys STACK POP r2
NAND rn r2
NAND rn rn
END rs
.end_define

#|----------| rs = rt ^ rd
.define XOR rs rt rd 
START rt rd
NAND rt rd
sys STACK PUSH rn
NAND rt rn
sys STACK POP r2
sys STACK PUSH rn
NAND rd r2
sys STACK POP r2
NAND r2 rn
END rs
.end_define

#|----------| rs = !rt
.define NOT rs rt
START rt rd
NAND rt rt
END rs
.end_define

#|----------| rs = rt XNOR rd
.define XNOR rs rt rd 
START rt rd 
NAND rt rt
sys STACK PUSH rn
NAND rd rd
sys STACK POP r2
NAND r2 rn
sys STACK PUSH rn
NAND rd rt 
sys STACK POP r2
NAND r2 rn
END rs
.end_define

#|----------| rt[0] = rs0[0] + rs1[0], rs2 = carryout
.define add-1 rt rs0 rs1 rs2  
sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r4
NAND r0 r1      # U1
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r1      # U3
sys STACK POP r2
sys STACK PUSH rn
NAND r0 r2      # U2
sys STACK POP r2
NAND rn r2      # U4
sys STACK PUSH rn
NAND r4 rn      # U5
sys STACK POP r2
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r2      # U6
sys STACK POP r2
sys STACK PUSH rn
NAND r4 r2      # U7
sys STACK POP r2
NAND rn r2      # U8
sys STACK POP r2
sys STACK POP r3
sys STACK PUSH rn
NAND r3 r2      # U9 rn is carry now 
sys STACK POP r2    # r2 is sum
sys STACK PUSH rn       
sys STACK PUSH r2
sys STACK POP rn
END rt
sys STACK POP rs2
.end_define

#|----------| rt[1:0] = rs0[1:0] + rs1[1:0], rs2 = carryout
.define add-2 rt rs0 rs1 rs2 
sys STACK PUSH rs0
sys STACK PUSH rs1
sys STACK PUSH rs2

START rs0 rs1
sys STACK POP r2
MOVE r4 r0 
BIT READ 0 
MOVE r0 r5
MOVE r4 r1
BIT READ 0
MOVE r1 r5

add-1 r3 r0 r1 r2

MOVE r5 r3
MOVE r4 rn
BIT WRITE 0
MOVE rn r4
sys STACK POP r1
sys STACK POP r0
MOVE r4 r0 
BIT READ 1 
MOVE r0 r5
MOVE r4 r1
BIT READ 1
MOVE r1 r5
add-1 r3 r0 r1 r2
MOVE r5 r3
MOVE r4 rn
BIT WRITE 1
MOVE rn r4
sys STACK PUSH r2

END rt
sys STACK POP r2
.end_define

#|----------| rt[3:0] = rs0[3:0] + rs1[3:0], rs2 = carryout
.define add-4 rt rs0 rs1 rs2
sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r2 #carry


add-2 r3 r0 r1 r2


MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 0
MOVE rn r4

MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 1
MOVE rn r4


MOVE r4 r0
BIT READ 2  # 3rd bit in r5
BIT WRITE 0 # 1rd bit of r4 = r5
BIT READ 3
BIT WRITE 1
MOVE r0 r4

MOVE r4 r1
BIT READ 2  # 3rd bit in r5
BIT WRITE 0 # 1rd bit of r4 = r5
BIT READ 3
BIT WRITE 1
MOVE r1 r4

add-2 r3 r0 r1 r2


MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 2
MOVE rn r4
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 3
MOVE rn r4



sys STACK PUSH r2

END rt
sys STACK POP rs2
.end_define

#|----------| rt[7:0] = rs0[7:0] + rs1[7:0], rs2 = carryout
.define add-8 rt rs0 rs1 rs2 

sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r2 #carry


add-4 r3 r0 r1 r2



MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 0
MOVE rn r4
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 1
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 2
MOVE rn r4
MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 3
MOVE rn r4


MOVE r4 r0  #moving second 4 bits to first
BIT READ 4  
BIT WRITE 0
BIT READ 5
BIT WRITE 1
BIT READ 6  
BIT WRITE 2
BIT READ 7
BIT WRITE 3
MOVE r0 r4



MOVE r4 r1
BIT READ 4  
BIT WRITE 0
BIT READ 5
BIT WRITE 1
BIT READ 6  
BIT WRITE 2
BIT READ 7
BIT WRITE 3
MOVE r1 r4

add-4 r3 r0 r1 r2

MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 4
MOVE rn r4
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 5
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 6
MOVE rn r4
MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 7
MOVE rn r4

sys STACK PUSH r2

END rt
sys STACK POP rs2
.end_define 

#|----------| rt[15:0] = rs0[15:0] + rs1[15:0], rs2 = carryout
.define add-16 rt rs0 rs1 rs2 

sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r2 #carry


add-8 r3 r0 r1 r2

MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 0
MOVE rn r4
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 1
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 2
MOVE rn r4
MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 3
MOVE rn r4

MOVE r4 r3
BIT READ 4
MOVE r4 rn
BIT WRITE 4
MOVE rn r4
MOVE r4 r3
BIT READ 5
MOVE r4 rn
BIT WRITE 5
MOVE rn r4

MOVE r4 r3
BIT READ 6
MOVE r4 rn
BIT WRITE 6
MOVE rn r4
MOVE r4 r3
BIT READ 7
MOVE r4 rn
BIT WRITE 7
MOVE rn r4



MOVE r4 r0  #moving second 8 bits to first
BIT READ 8  
BIT WRITE 0
BIT READ 9
BIT WRITE 1
BIT READ 10  
BIT WRITE 2
BIT READ 11
BIT WRITE 3
BIT READ 12  
BIT WRITE 4
BIT READ 13
BIT WRITE 5
BIT READ 14  
BIT WRITE 6
BIT READ 15
BIT WRITE 7
MOVE r0 r4



MOVE r4 r1
BIT READ 8  
BIT WRITE 0
BIT READ 9
BIT WRITE 1
BIT READ 10  
BIT WRITE 2
BIT READ 11
BIT WRITE 3
BIT READ 12  
BIT WRITE 4
BIT READ 13
BIT WRITE 5
BIT READ 14  
BIT WRITE 6
BIT READ 15
BIT WRITE 7
MOVE r1 r4

add-8 r3 r0 r1 r2



MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 8
MOVE rn r4

MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 9
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 10
MOVE rn r4

MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 11
MOVE rn r4

MOVE r4 r3
BIT READ 4
MOVE r4 rn
BIT WRITE 12
MOVE rn r4

MOVE r4 r3
BIT READ 5
MOVE r4 rn
BIT WRITE 13
MOVE rn r4

MOVE r4 r3
BIT READ 6
MOVE r4 rn
BIT WRITE 14
MOVE rn r4

MOVE r4 r3
BIT READ 7
MOVE r4 rn
BIT WRITE 15
MOVE rn r4

sys STACK PUSH r2

END rt
sys STACK POP rs2
.end_define 

#|----------| rt = rs0 + rs1
.define add rt rs0 rs1 

START rs0 rs1
add-carry rn r0 r1 r2 #r2 is carry-in which is initialised as 0
END rt

.end_define

#|----------| #|----------| rt = rs + rs1, rs2 = carryin
.define add-carry rt rs0 rs1 rs2  
sys STACK PUSH rs2
START rs0 rs1 #carry
sys STACK POP r2
add-16 r3 r0 r1 r2

MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 0
MOVE rn r4
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 1
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 2
MOVE rn r4
MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 3
MOVE rn r4

MOVE r4 r3
BIT READ 4
MOVE r4 rn
BIT WRITE 4
MOVE rn r4
MOVE r4 r3
BIT READ 5
MOVE r4 rn
BIT WRITE 5
MOVE rn r4

MOVE r4 r3
BIT READ 6
MOVE r4 rn
BIT WRITE 6
MOVE rn r4
MOVE r4 r3
BIT READ 7
MOVE r4 rn
BIT WRITE 7
MOVE rn r4

MOVE r4 r3
BIT READ 8
MOVE r4 rn
BIT WRITE 8
MOVE rn r4
MOVE r4 r3
BIT READ 9
MOVE r4 rn
BIT WRITE 9
MOVE rn r4

MOVE r4 r3
BIT READ 10
MOVE r4 rn
BIT WRITE 10
MOVE rn r4
MOVE r4 r3
BIT READ 11
MOVE r4 rn
BIT WRITE 11
MOVE rn r4

MOVE r4 r3
BIT READ 12
MOVE r4 rn
BIT WRITE 12
MOVE rn r4
MOVE r4 r3
BIT READ 13
MOVE r4 rn
BIT WRITE 13
MOVE rn r4

MOVE r4 r3
BIT READ 14
MOVE r4 rn
BIT WRITE 14
MOVE rn r4
MOVE r4 r3
BIT READ 15
MOVE r4 rn
BIT WRITE 15
MOVE rn r4

MOVE r4 r0  #moving second 8 bits to first
BIT READ 16  
BIT WRITE 0
BIT READ 17
BIT WRITE 1
BIT READ 18  
BIT WRITE 2
BIT READ 19
BIT WRITE 3
BIT READ 20 
BIT WRITE 4
BIT READ 21
BIT WRITE 5
BIT READ 22  
BIT WRITE 6
BIT READ 23
BIT WRITE 7
BIT READ 24  
BIT WRITE 8
BIT READ 25
BIT WRITE 9
BIT READ 26  
BIT WRITE 10
BIT READ 27
BIT WRITE 11
BIT READ 28 
BIT WRITE 12
BIT READ 29
BIT WRITE 13
BIT READ 30  
BIT WRITE 14
BIT READ 31
BIT WRITE 15
MOVE r0 r4



MOVE r4 r1
BIT READ 16  
BIT WRITE 0
BIT READ 17
BIT WRITE 1
BIT READ 18  
BIT WRITE 2
BIT READ 19
BIT WRITE 3
BIT READ 20 
BIT WRITE 4
BIT READ 21
BIT WRITE 5
BIT READ 22  
BIT WRITE 6
BIT READ 23
BIT WRITE 7
BIT READ 24  
BIT WRITE 8
BIT READ 25
BIT WRITE 9
BIT READ 26  
BIT WRITE 10
BIT READ 27
BIT WRITE 11
BIT READ 28 
BIT WRITE 12
BIT READ 29
BIT WRITE 13
BIT READ 30  
BIT WRITE 14
BIT READ 31
BIT WRITE 15
MOVE r1 r4

add-16 r3 r0 r1 r2

MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 16
MOVE rn r4

MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 17
MOVE rn r4

MOVE r4 r3
BIT READ 2
MOVE r4 rn
BIT WRITE 18
MOVE rn r4

MOVE r4 r3
BIT READ 3
MOVE r4 rn
BIT WRITE 19
MOVE rn r4

MOVE r4 r3
BIT READ 4
MOVE r4 rn
BIT WRITE 20
MOVE rn r4

MOVE r4 r3
BIT READ 5
MOVE r4 rn
BIT WRITE 21
MOVE rn r4

MOVE r4 r3
BIT READ 6
MOVE r4 rn
BIT WRITE 22
MOVE rn r4

MOVE r4 r3
BIT READ 7
MOVE r4 rn
BIT WRITE 23
MOVE rn r4

MOVE r4 r3
BIT READ 8
MOVE r4 rn
BIT WRITE 24
MOVE rn r4

MOVE r4 r3
BIT READ 9
MOVE r4 rn
BIT WRITE 25
MOVE rn r4

MOVE r4 r3
BIT READ 10
MOVE r4 rn
BIT WRITE 26
MOVE rn r4

MOVE r4 r3
BIT READ 11
MOVE r4 rn
BIT WRITE 27
MOVE rn r4

MOVE r4 r3
BIT READ 12
MOVE r4 rn
BIT WRITE 28
MOVE rn r4

MOVE r4 r3
BIT READ 13
MOVE r4 rn
BIT WRITE 29
MOVE rn r4

MOVE r4 r3
BIT READ 14
MOVE r4 rn
BIT WRITE 30
MOVE rn r4

MOVE r4 r3
BIT READ 15
MOVE r4 rn
BIT WRITE 31
MOVE rn r4

sys STACK PUSH r2
END rt
sys STACK POP rs2
.end_define


#|----------| rt=sum rd = carry-out rs0 and rs1 numbers to compare
# sum = 0, carryout = 1: rs0 = rs1
# carryout = 0: rs0 < rs1
# sum != 0, carryout = 1: rs0 > rs1
.define comparator rt rd rs0 rs1 

START rs0 rs1

NOT r1 r1  # 1's complement of r1
load1 r2      # Carry in = 1
add-carry r3 r0 r1 r2

sys STACK PUSH r2
END rt
sys STACK POP rd
.end_define

#|------------| rt = (rs < rd)
.define IsLessThan rt rs rd

START rs rd
comparator r2 r3 r0 r1
NOT r3 r3
MOVE rn r3
END rt

.end_define




#|-------| rt = (rs == rd)


