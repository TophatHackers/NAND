# NAND
Groundbreaking NAND language based on the universal NAND gate.

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

| **2 bit** | **OPcode** |
| :-------- | :--------- |
| `00`      | `NAND`     |
| `01`      | `SYS`      |
| `10`      | `LOOP`     |
| `11`      | `BIT`      |

### NAND

| **Encoding** | **Description**             |
| :----------- | :-------------------------- |
| `op<7:6>`    | Identifies NAND instruction |
| `rs<5:3>`    | First register              |
| `rt<2:0>`    | Second register             |


NANDS the first register with the second and saves it in rn.

`rn = rs NAND rt`
