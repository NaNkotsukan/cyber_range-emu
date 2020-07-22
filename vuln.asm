BITS 32
    org 0x7c00
main:
    ; ebp = 0x7c00, esp = ebp - 0x50
    mov eax, ebp
    sub eax,byte +0x40
    push eax
    call check_pass
    add esp,byte +0x4
    jmp 0

check_pass:
    push ebp
    mov ebp, esp
    sub esp,byte +0x20
    mov dword [ebp-0x4],0x0
    mov dword [ebp-0x8],0x1
    mov dword [ebp-0xc],0x0
    mov dword [ebp-0x14],0x73733470
    ; mov dword [ebp-0x10],0x00000000 コピー先
    
    mov dword [ebp-0x4],0x0
strcpy:

cpy_loop:
    mov edx,[ebp-0x4]
    mov eax,[ebp+0x8]
    add eax,edx
    mov edx,[ebp-0x4]
    mov ecx,ebp
    sub ecx,byte +0x10
    add edx,ecx
    mov al,[eax]
    cmp al,0
    jz cpy_end
    mov [edx],al
    inc dword [ebp-0x4]
    jmp cpy_loop
cpy_end:

    mov dword [ebp-0x4],0x0
strcmp:
    jmp cmp_check_cnt
cmp_loop:
    mov eax,[ebp-0x4]
    mov edx,ebp
    sub edx,byte +0x10
    add eax,edx
    mov dl,[eax]
    mov eax,[ebp-0x4]
    mov ecx,ebp
    sub ecx,byte +0x14
    add eax,ecx
    mov al,[eax]
    inc dword [ebp-0x4]
    cmp dl,al
    jz cmp_check_cnt
    mov dword [ebp-0x8],0x0
    jmp cmp_flag

cmp_check_cnt:
    cmp dword [ebp-0x4],byte +0x4
    jnz cmp_loop

cmp_flag:
    cmp dword [ebp-0x8],byte +0x0
    jz cmp_end
    mov dword [ebp-0xc],0x40

cmp_end:
    mov eax,[ebp-0xc]
    leave
    ret
