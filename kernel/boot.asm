bits 16
org 0x7C00

start:
    mov si, message

print:
    lodsb
    test al, al
    jz halt

    mov ah, 0x0E
    int 0x10
    jmp print

halt:
    cli
    hlt
    jmp halt

message db 'Hello World from CrustOS!', 0

times 510-($-$$) db 0
dw 0xAA55