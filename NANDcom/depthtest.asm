sys READ r0
sys READ r1
START r0 r1
AND r0 r4 r5
AND r5 r1 r5
END r5
sys WRITE r5


compiles to:

sys READ r0
sys READ r1
START r0 r1
START r0 r4
NAND r0 r4
NAND rn rn
END r5
START r5 r1
NAND r5 r1
NAND rn rn
END r5
END r5
sys WRITE r5