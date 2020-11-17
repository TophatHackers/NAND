sys READ r0
sys READ r1
#sys READ r2 

#add-carry r3 r0 r1 r2
#sys WRITE r3
#subtract r3 r0 r1
#comparator r3 r2 r0 r1 
#IsLessThan r3 r0 r1 
IsEqual r3 r0 r1
#or_all_bits r3 r3
sys WRITE r3
#sys WRITE r2
