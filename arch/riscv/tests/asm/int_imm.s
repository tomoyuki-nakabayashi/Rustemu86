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
lh  x1, 10(x2)
lhu x1, 10(x2)
lb  x1, 11(x2)
lbu x1, 11(x2)
sw  x1, 0xc(x2)
sh  x1, 0xe(x2)
sb  x1, 0xf(x2)
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
slt x2, x0, x1
slt x3, x1, x0
sltu x2, x0, x1
sltu x3, x1, x0
and x3, x1, x2
or  x3, x1, x2
xor x3, x1, x2
sll x3, x1, x2
srl x3, x1, x2
add x2, x2, x1
sub x2, x2, x1
jalr x2, x1, 4
bne x1, x2, j3
xor x1, x1, x1
j3:
blt x1, x2, j4
nop
j4:
bltu x1, x2, j5
nop
j5:
bge x1, x2, j6
nop
j6:
bgeu x1, x2, j7
nop
j7:
nop
csrr t1, mtvec
csrwi mtvec, 5

