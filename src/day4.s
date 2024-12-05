    .pushsection .text

inline:
    push rbp
    push rbx
    push r15
    push r14
    push r13
    push r12

    vpbroadcastb ymm4, byte ptr [rip + ascii_x]
    vmovdqu ymm0, ymmword ptr [rdi]

    vmovdqu  ymm8,  ymmword ptr [rdi + 141 * 0 + 32 * 0]
    vpcmpeqb ymm9,  ymm8,  ymm4
    vpcmpeqb ymm10, ymm8,  ymm5
    vpcmpeqb ymm11, ymm8,  ymm6
    vpcmpeqb ymm12, ymm8,  ymm7
    
    vpmovmskb eax, ymm9
    vpmovmskb edx, ymm10
    vpmovmskb ecx, ymm11
    vpmovmskb ebx, ymm12

    vpinsrd xmm16, eax, 0
    vpinsrd xmm17, edx, 0
    vpinsrd xmm18, ecx, 0
    vpinsrd xmm19, ebx, 0

    vmovdqu ymm8,  ymmword ptr [rdi + 141 * 0 + 32 * 1]
    vpcmpeqb ymm9,  ymm8,  ymm4
    vpcmpeqb ymm10, ymm8,  ymm5
    vpcmpeqb ymm11, ymm8,  ymm6
    vpcmpeqb ymm12, ymm8,  ymm7
    
    vpmovmskb eax, ymm9
    vpmovmskb edx, ymm10
    vpmovmskb ecx, ymm11
    vpmovmskb ebx, ymm12

    vpinsrd xmm16, eax, 1
    vpinsrd xmm17, edx, 1
    vpinsrd xmm18, ecx, 1
    vpinsrd xmm19, ebx, 1

    vmovdqu ymm8,  ymmword ptr [rdi + 141 * 0 + 32 * 2]
    vpcmpeqb ymm9,  ymm8,  ymm4
    vpcmpeqb ymm10, ymm8,  ymm5
    vpcmpeqb ymm11, ymm8,  ymm6
    vpcmpeqb ymm12, ymm8,  ymm7
    
    vpmovmskb eax, ymm9
    vpmovmskb edx, ymm10
    vpmovmskb ecx, ymm11
    vpmovmskb ebx, ymm12

    vpinsrd xmm16, eax, 2
    vpinsrd xmm17, edx, 2
    vpinsrd xmm18, ecx, 2
    vpinsrd xmm19, ebx, 2

    vmovdqu ymm8,  ymmword ptr [rdi + 141 * 0 + 32 * 3]
    vpcmpeqb ymm9,  ymm8,  ymm4
    vpcmpeqb ymm10, ymm8,  ymm5
    vpcmpeqb ymm11, ymm8,  ymm6
    vpcmpeqb ymm12, ymm8,  ymm7
    
    vpmovmskb eax, ymm9
    vpmovmskb edx, ymm10
    vpmovmskb ecx, ymm11
    vpmovmskb ebx, ymm12

    vpinsrd xmm16, eax, 3
    vpinsrd xmm17, edx, 3
    vpinsrd xmm18, ecx, 3
    vpinsrd xmm19, ebx, 3

    mov r15, 140
1:


    dec r15
    jne 1b

    pop r12
    pop r13
    pop r14
    pop r15
    pop rbx
    pop rbp
    ret

    .popsection

    .pushsection .data

ascii_x: .byte 'X'
ascii_m: .byte 'M'
ascii_a: .byte 'A'
ascii_s: .byte 'S'

    .popsection