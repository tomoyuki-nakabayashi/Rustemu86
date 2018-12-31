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
beq  x1, x2, j2
nop
j2:
wfi
lw  x1, 4(x2)
sw  x1, 0xc(x2)
fence.i
slti x2, x1, 1
slti x3, x1, -1
sltiu x2, x1, 1
sltiu x3, x1, -1
andi x2, x1, 170
andi x3, x1, -171
xori x1, zero, -1
xori x2, x1, -1
slli x1, x1, 5
srli x2, x1, 5
srli x3, x1, 4
addi x1, x0, -1
addi x1, x0, -256
srai x2, x1, 5
lui  x1, 0x12345
auipc x1, 0x12345

