bits 64

; Expect uart recieve "CBA"
mov rdx, 0x10000000
mov rsp, 0x0100
mov rax, 0x41 ; A
push rax
inc rax       ; B
push rax
inc rax       ; C
push rax
pop rbx
mov [rdx], rbx ; C
pop rbx
mov [rdx], rbx ; B
pop rbx
mov [rdx], rbx ; A
hlt