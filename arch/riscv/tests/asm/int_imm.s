addi x1, x1, 1
addi x2, x2, -1
ori  x1, x1, 0x2
ori  x2, x2, -1
add  x1, x1, x1
jal  x1, j1
nop
j1:
wfi
jal x1, j1

