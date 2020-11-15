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






