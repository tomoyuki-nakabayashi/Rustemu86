bits 64

mov word [0xb8000], 0x0148 ; blue     `H`
mov word [0xb8002], 0x0265 ; green    `e`
mov word [0xb8004], 0x036c ; cyan     `l`
mov word [0xb8006], 0x046c ; red      `l`
mov word [0xb8008], 0x056f ; magenta  `o`
hlt