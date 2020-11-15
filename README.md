# NAND
Groundbreaking NAND language based on the universal NAND gate.



# Specifications

## Registers

| **Register encoding** | **Register name** | **Description**                                                                 |
| :-------------------- | :---------------- | :------------------------------------------------------------------------------ |
| `0`                   | `pc`              | Program counter, points at which instruction to execute                         |
| `1`                   | `rn`              | Used as intermediate output, for example NAND saves to rn and END loads from rn |
| `2`                   | `r0`              | General use register, used as first argument                                    |
| `3`                   | `r1`              | General use register, used as second argument                                   |
| `4`                   | `r2`              | General use register                                                            |
| `5`                   | `r3`              | General use register                                                            |
| `6`                   | `r4`              | General use register, used by BIT instruction to change specific bit            |
| `7`                   | `r5`              | General use register, used by BIT instruction to load the changed bit           |

## Instructions

| **Encoding** | **OPcode** |
| :----------- | :--------- |
| `0`          | `NAND`     |
| `1`          | `SYS`      |
| `2`          | `LOOP`     |
| `3`          | `BIT`      |

### NAND

| **Encoding** | **Description**             |
| :----------- | :-------------------------- |
| `op<7:6>`    | Identifies NAND instruction |
| `rs<5:3>`    | First register              |
| `rt<2:0>`    | Second register             |


NANDS the first register with the second and saves it in rn.

`rn = rs NAND rt`

### SYS

| **ID Encoding** | **Description** |
| :-------------- | :-------------- |
| `0`             | Handle stack    |
| `1`             | Read from stdin |
| `2`             | Write to stdin  |
| `3`             | TBD             |

#### STACK

| **Encoding** | **Description**                                             |
| :----------- | :---------------------------------------------------------- |
| `op<7:6>`    | Identifies SYS instruction                                  |
| `id<5:4>`    | Identifies type of syscall                                  |
| `type<3>`    | Type of stack operation, push or pop. `0 = push`, `1 = pop` |
| `rs<2:0>`    | Register to push/pop                                        |

#### READ

| **Encoding** | **Description**                        |
| :----------- | :------------------------------------- |
| `op<7:6>`    | Identifies SYS instruction             |
| `id<5:4>`    | Identifies type of syscall             |
| `rs<3:1>`    | Register to read integer from stdin to |
| `<0>`        | TBD, currently padding.                |

#### WRITE

| **Encoding** | **Description**                 |
| :----------- | :------------------------------ |
| `op<7:6>`    | Identifies SYS instruction      |
| `id<5:4>`    | Identifies type of syscall      |
| `rs<3:1>`    | Register to write to stdin from |
| `<0>`        | TBD, currently padding.         |

### LOOP

Loop is in reality 2 operations, START and END. They are differentiated by the last 3 bits, if they're all 0 then the operation is identified as END otherwise as START. Be wary of the bug if you try to pass PC as the 2nd argument in START (since PC = 000), this will result in unintended behaviour!

#### START

| **Encoding** | **Description**                    |
| :----------- | :--------------------------------- |
| `op<7:6>`    | Identifies LOOP instruction        |
| `rs<5:3>`    | Register to use as first argument  |
| `rt<2:0>`    | Register to use as second argument |

#### END

| **Encoding** | **Description**                                               |
| :----------- | :------------------------------------------------------------ |
| `op<7:6>`    | Identifies LOOP instruction                                   |
| `rs<5:3>`    | Register to use as return value                               |
| `id<2:0>`    | Padding/Identifier, all being 0 differentiates END from START |

### BIT

| **Encoding** | **Description**                                     |
| :----------- | :-------------------------------------------------- |
| `op<7:6>`    | Identifies BIT instruction                          |
| `type<5>`    | Identifies type of BIT operation, READ or WRITE     |
| `imm<4:0>`   | Immediate that signifies index of bit to READ/WRITE |






