



.intel_syntax noprefix
.text
.p2align 5
.globl fma_mmm_f32_16x6_0_15_4
fma_mmm_f32_16x6_0_15_4:
.cfi_startproc



    push        rbp
    mov         rbp, rsp



    push        rbx
    push        r12
    push        r13
    push        r14
    push        r15

    sub         rsp, 8


.cfi_def_cfa_offset 64


    stmxcsr     [rsp + 4]

    mov         rax, 0x1FC0

    mov         [rsp], eax
    ldmxcsr     [rsp]

    vzeroall

// vim: set syntax=asm :

.non_linear:

.non_linear_loop_enter:
    sub     rdi,    40
.non_linear_loop:
    add     rdi,    40
    mov     rax,    [rdi]

    mov     r8, 24
    cmp     rax, 0
    cmovl   rax, r8
    cmp     rax, 24
    cmovg   rax, r8


    lea     r8, [ rip + .jmp_table ]

    movsxd  r9, dword ptr [ r8 + rax * 4 ]
    lea     r8, [ r8 + r9 ]
    jmp     r8

.jmp_table:

    .long      .done-.jmp_table

    .long      .scalar_min-.jmp_table

    .long      .scalar_max-.jmp_table

    .long      .scalar_add-.jmp_table

    .long      .scalar_mul-.jmp_table

    .long      .scalar_sub-.jmp_table

    .long      .scalar_sub_flipped-.jmp_table

    .long      .per_row_min-.jmp_table

    .long      .per_row_max-.jmp_table

    .long      .per_row_add-.jmp_table

    .long      .per_row_mul-.jmp_table

    .long      .per_row_sub-.jmp_table

    .long      .per_row_sub_flipped-.jmp_table

    .long      .per_col_min-.jmp_table

    .long      .per_col_max-.jmp_table

    .long      .per_col_add-.jmp_table

    .long      .per_col_mul-.jmp_table

    .long      .per_col_sub-.jmp_table

    .long      .per_col_sub_flipped-.jmp_table

    .long      .q_scale-.jmp_table

    .long      .add_unicast-.jmp_table

    .long      .add_row_col_products-.jmp_table

    .long      .store-.jmp_table

    .long      .add_mat_mul-.jmp_table

    .long      .unsupported-.jmp_table

.unsupported:
    mov     rax,    1
    jmp     .return


.done:
    mov     rax, 0
    jmp     .return



.add_mat_mul:
    mov     rbx,    [rdi + 24]   // B
    mov     rax,    [rdi + 16]   // A

    mov     rcx,    [rdi + 8]    // k
    test    rcx,    rcx
    jz      .non_linear_loop

    mov     rsi, [rbx]   // B discriminant
    cmp     rsi,  0
    je      .packed_packed

.packed_tops_and_offsets:
    mov     rsi,    [rbx + 16]  // B cols head
    mov     rbx,    [rbx + 8]   // rbx: current row offset ptr

    mov     r8,     [rsi]
    mov     r9,     [rsi + 8]
    mov     r10,    [rsi + 16]
    mov     r11,    [rsi + 24]
    mov     r12,    [rsi + 32]
    mov     r13,    [rsi + 40]
 
.main_loop_packed_tops_and_offsets:
    mov             rsi,    [rbx]   // rsi: current row offset

    vmovaps         ymm12,  [rax]
    vmovaps         ymm13,  [rax + 32]

    vbroadcastss    ymm14,  dword ptr [r8 + rsi]
    vbroadcastss    ymm15,  dword ptr [r9 + rsi]

    vfmadd231ps     ymm0,   ymm12, ymm14
    vfmadd231ps     ymm1,   ymm13, ymm14

    vbroadcastss    ymm14,  dword ptr [r10 + rsi]

    vfmadd231ps     ymm2,   ymm12, ymm15
    vfmadd231ps     ymm3,   ymm13, ymm15

    vbroadcastss    ymm15,  dword ptr [r11 + rsi]

    vfmadd231ps     ymm4,   ymm12, ymm14
    vfmadd231ps     ymm5,   ymm13, ymm14

    vbroadcastss    ymm14,  dword ptr [r12 + rsi]

    vfmadd231ps     ymm6,   ymm12, ymm15
    vfmadd231ps     ymm7,   ymm13, ymm15

    vbroadcastss    ymm15,  dword ptr [r13 + rsi]

    vfmadd231ps     ymm8,   ymm12, ymm14
    vfmadd231ps     ymm9,   ymm13, ymm14

    vfmadd231ps     ymm10,   ymm12, ymm15
    vfmadd231ps     ymm11,   ymm13, ymm15

    add             rbx,    8
    add             rax,    64
    dec             rcx
    jnz             .main_loop_packed_tops_and_offsets

    jmp             .non_linear_loop

.packed_packed:
    mov     rbx,   [rbx + 8] // B 

.main_loop_packed_packed:
    vbroadcastss    ymm14,  dword ptr [rbx]
    vbroadcastss    ymm15,  dword ptr [rbx + 4]

    vmovaps         ymm12,  [rax]
    vmovaps         ymm13,  [rax + 32]

    vfmadd231ps     ymm0,   ymm12, ymm14
    vfmadd231ps     ymm1,   ymm13, ymm14

    vbroadcastss    ymm14,  dword ptr [rbx + 8]

    vfmadd231ps     ymm2,   ymm12, ymm15
    vfmadd231ps     ymm3,   ymm13, ymm15

    vbroadcastss    ymm15,  dword ptr [rbx + 12]

    vfmadd231ps     ymm4,   ymm12, ymm14
    vfmadd231ps     ymm5,   ymm13, ymm14

    vbroadcastss    ymm14,  dword ptr [rbx + 16]

    vfmadd231ps     ymm6,   ymm12, ymm15
    vfmadd231ps     ymm7,   ymm13, ymm15

    vbroadcastss    ymm15,  dword ptr [rbx + 20]

    vfmadd231ps     ymm8,   ymm12, ymm14
    vfmadd231ps     ymm9,   ymm13, ymm14

    vfmadd231ps     ymm10,   ymm12, ymm15
    vfmadd231ps     ymm11,   ymm13, ymm15

    add             rbx,    24
    add             rax,    64
    dec             rcx
    jnz             .main_loop_packed_packed

    jmp             .non_linear_loop

// NON LINEAR / ADDC

// vim: set syntax=asm :

// vim: set syntax=asm :

.scalar_min:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vminps          ymm0, ymm12, ymm0
        
            vminps          ymm1, ymm12, ymm1
        
            vminps          ymm2, ymm12, ymm2
        
            vminps          ymm3, ymm12, ymm3
        
            vminps          ymm4, ymm12, ymm4
        
            vminps          ymm5, ymm12, ymm5
        
            vminps          ymm6, ymm12, ymm6
        
            vminps          ymm7, ymm12, ymm7
        
            vminps          ymm8, ymm12, ymm8
        
            vminps          ymm9, ymm12, ymm9
        
            vminps          ymm10, ymm12, ymm10
        
            vminps          ymm11, ymm12, ymm11
        
    

    jmp    .non_linear_loop

// vim: set syntax=asm :

.scalar_max:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vmaxps          ymm0, ymm12, ymm0
        
            vmaxps          ymm1, ymm12, ymm1
        
            vmaxps          ymm2, ymm12, ymm2
        
            vmaxps          ymm3, ymm12, ymm3
        
            vmaxps          ymm4, ymm12, ymm4
        
            vmaxps          ymm5, ymm12, ymm5
        
            vmaxps          ymm6, ymm12, ymm6
        
            vmaxps          ymm7, ymm12, ymm7
        
            vmaxps          ymm8, ymm12, ymm8
        
            vmaxps          ymm9, ymm12, ymm9
        
            vmaxps          ymm10, ymm12, ymm10
        
            vmaxps          ymm11, ymm12, ymm11
        
    

    jmp    .non_linear_loop

// vim: set syntax=asm :

.scalar_add:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vaddps          ymm0, ymm12, ymm0
        
            vaddps          ymm1, ymm12, ymm1
        
            vaddps          ymm2, ymm12, ymm2
        
            vaddps          ymm3, ymm12, ymm3
        
            vaddps          ymm4, ymm12, ymm4
        
            vaddps          ymm5, ymm12, ymm5
        
            vaddps          ymm6, ymm12, ymm6
        
            vaddps          ymm7, ymm12, ymm7
        
            vaddps          ymm8, ymm12, ymm8
        
            vaddps          ymm9, ymm12, ymm9
        
            vaddps          ymm10, ymm12, ymm10
        
            vaddps          ymm11, ymm12, ymm11
        
    

    jmp    .non_linear_loop

// vim: set syntax=asm :

.scalar_mul:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vmulps          ymm0, ymm12, ymm0
        
            vmulps          ymm1, ymm12, ymm1
        
            vmulps          ymm2, ymm12, ymm2
        
            vmulps          ymm3, ymm12, ymm3
        
            vmulps          ymm4, ymm12, ymm4
        
            vmulps          ymm5, ymm12, ymm5
        
            vmulps          ymm6, ymm12, ymm6
        
            vmulps          ymm7, ymm12, ymm7
        
            vmulps          ymm8, ymm12, ymm8
        
            vmulps          ymm9, ymm12, ymm9
        
            vmulps          ymm10, ymm12, ymm10
        
            vmulps          ymm11, ymm12, ymm11
        
    

    jmp    .non_linear_loop

// vim: set syntax=asm :

.scalar_sub:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vsubps          ymm0, ymm12, ymm0
        
            vsubps          ymm1, ymm12, ymm1
        
            vsubps          ymm2, ymm12, ymm2
        
            vsubps          ymm3, ymm12, ymm3
        
            vsubps          ymm4, ymm12, ymm4
        
            vsubps          ymm5, ymm12, ymm5
        
            vsubps          ymm6, ymm12, ymm6
        
            vsubps          ymm7, ymm12, ymm7
        
            vsubps          ymm8, ymm12, ymm8
        
            vsubps          ymm9, ymm12, ymm9
        
            vsubps          ymm10, ymm12, ymm10
        
            vsubps          ymm11, ymm12, ymm11
        
    

    jmp    .non_linear_loop

// vim: set syntax=asm :

.scalar_sub_flipped:
    vbroadcastss    ymm12, dword ptr [rdi + 8]
    
        
            vsubps          ymm0, ymm0, ymm12
        
            vsubps          ymm1, ymm1, ymm12
        
            vsubps          ymm2, ymm2, ymm12
        
            vsubps          ymm3, ymm3, ymm12
        
            vsubps          ymm4, ymm4, ymm12
        
            vsubps          ymm5, ymm5, ymm12
        
            vsubps          ymm6, ymm6, ymm12
        
            vsubps          ymm7, ymm7, ymm12
        
            vsubps          ymm8, ymm8, ymm12
        
            vsubps          ymm9, ymm9, ymm12
        
            vsubps          ymm10, ymm10, ymm12
        
            vsubps          ymm11, ymm11, ymm12
        
    

    jmp    .non_linear_loop



// vim: set syntax=asm :

// vim: set syntax=asm :

.per_row_min:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vminps ymm0, ymm12, ymm0
    
        vminps ymm1, ymm13, ymm1
    
        vminps ymm2, ymm12, ymm2
    
        vminps ymm3, ymm13, ymm3
    
        vminps ymm4, ymm12, ymm4
    
        vminps ymm5, ymm13, ymm5
    
        vminps ymm6, ymm12, ymm6
    
        vminps ymm7, ymm13, ymm7
    
        vminps ymm8, ymm12, ymm8
    
        vminps ymm9, ymm13, ymm9
    
        vminps ymm10, ymm12, ymm10
    
        vminps ymm11, ymm13, ymm11
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_max:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vmaxps ymm0, ymm12, ymm0
    
        vmaxps ymm1, ymm13, ymm1
    
        vmaxps ymm2, ymm12, ymm2
    
        vmaxps ymm3, ymm13, ymm3
    
        vmaxps ymm4, ymm12, ymm4
    
        vmaxps ymm5, ymm13, ymm5
    
        vmaxps ymm6, ymm12, ymm6
    
        vmaxps ymm7, ymm13, ymm7
    
        vmaxps ymm8, ymm12, ymm8
    
        vmaxps ymm9, ymm13, ymm9
    
        vmaxps ymm10, ymm12, ymm10
    
        vmaxps ymm11, ymm13, ymm11
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_add:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vaddps ymm0, ymm12, ymm0
    
        vaddps ymm1, ymm13, ymm1
    
        vaddps ymm2, ymm12, ymm2
    
        vaddps ymm3, ymm13, ymm3
    
        vaddps ymm4, ymm12, ymm4
    
        vaddps ymm5, ymm13, ymm5
    
        vaddps ymm6, ymm12, ymm6
    
        vaddps ymm7, ymm13, ymm7
    
        vaddps ymm8, ymm12, ymm8
    
        vaddps ymm9, ymm13, ymm9
    
        vaddps ymm10, ymm12, ymm10
    
        vaddps ymm11, ymm13, ymm11
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_mul:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vmulps ymm0, ymm12, ymm0
    
        vmulps ymm1, ymm13, ymm1
    
        vmulps ymm2, ymm12, ymm2
    
        vmulps ymm3, ymm13, ymm3
    
        vmulps ymm4, ymm12, ymm4
    
        vmulps ymm5, ymm13, ymm5
    
        vmulps ymm6, ymm12, ymm6
    
        vmulps ymm7, ymm13, ymm7
    
        vmulps ymm8, ymm12, ymm8
    
        vmulps ymm9, ymm13, ymm9
    
        vmulps ymm10, ymm12, ymm10
    
        vmulps ymm11, ymm13, ymm11
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_sub:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vsubps ymm0, ymm12, ymm0
    
        vsubps ymm1, ymm13, ymm1
    
        vsubps ymm2, ymm12, ymm2
    
        vsubps ymm3, ymm13, ymm3
    
        vsubps ymm4, ymm12, ymm4
    
        vsubps ymm5, ymm13, ymm5
    
        vsubps ymm6, ymm12, ymm6
    
        vsubps ymm7, ymm13, ymm7
    
        vsubps ymm8, ymm12, ymm8
    
        vsubps ymm9, ymm13, ymm9
    
        vsubps ymm10, ymm12, ymm10
    
        vsubps ymm11, ymm13, ymm11
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_sub_flipped:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm12,  [rax + 0]

    vmovups         ymm13,  [rax + 32]



    
        vsubps ymm0, ymm0, ymm12
    
        vsubps ymm1, ymm1, ymm13
    
        vsubps ymm2, ymm2, ymm12
    
        vsubps ymm3, ymm3, ymm13
    
        vsubps ymm4, ymm4, ymm12
    
        vsubps ymm5, ymm5, ymm13
    
        vsubps ymm6, ymm6, ymm12
    
        vsubps ymm7, ymm7, ymm13
    
        vsubps ymm8, ymm8, ymm12
    
        vsubps ymm9, ymm9, ymm13
    
        vsubps ymm10, ymm10, ymm12
    
        vsubps ymm11, ymm11, ymm13
    


    jmp .non_linear_loop



// vim: set syntax=asm :

// vim: set syntax=asm :

.per_col_min:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm0, ymm12, ymm0
        
    
        
        
            vminps ymm1, ymm12, ymm1
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm2, ymm12, ymm2
        
    
        
        
            vminps ymm3, ymm12, ymm3
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm4, ymm12, ymm4
        
    
        
        
            vminps ymm5, ymm12, ymm5
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm6, ymm12, ymm6
        
    
        
        
            vminps ymm7, ymm12, ymm7
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm8, ymm12, ymm8
        
    
        
        
            vminps ymm9, ymm12, ymm9
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm10, ymm12, ymm10
        
    
        
        
            vminps ymm11, ymm12, ymm11
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_max:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm0, ymm12, ymm0
        
    
        
        
            vmaxps ymm1, ymm12, ymm1
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm2, ymm12, ymm2
        
    
        
        
            vmaxps ymm3, ymm12, ymm3
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm4, ymm12, ymm4
        
    
        
        
            vmaxps ymm5, ymm12, ymm5
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm6, ymm12, ymm6
        
    
        
        
            vmaxps ymm7, ymm12, ymm7
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm8, ymm12, ymm8
        
    
        
        
            vmaxps ymm9, ymm12, ymm9
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm10, ymm12, ymm10
        
    
        
        
            vmaxps ymm11, ymm12, ymm11
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_add:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm0, ymm12, ymm0
        
    
        
        
            vaddps ymm1, ymm12, ymm1
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm2, ymm12, ymm2
        
    
        
        
            vaddps ymm3, ymm12, ymm3
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm4, ymm12, ymm4
        
    
        
        
            vaddps ymm5, ymm12, ymm5
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm6, ymm12, ymm6
        
    
        
        
            vaddps ymm7, ymm12, ymm7
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm8, ymm12, ymm8
        
    
        
        
            vaddps ymm9, ymm12, ymm9
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm10, ymm12, ymm10
        
    
        
        
            vaddps ymm11, ymm12, ymm11
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_mul:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm0, ymm12, ymm0
        
    
        
        
            vmulps ymm1, ymm12, ymm1
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm2, ymm12, ymm2
        
    
        
        
            vmulps ymm3, ymm12, ymm3
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm4, ymm12, ymm4
        
    
        
        
            vmulps ymm5, ymm12, ymm5
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm6, ymm12, ymm6
        
    
        
        
            vmulps ymm7, ymm12, ymm7
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm8, ymm12, ymm8
        
    
        
        
            vmulps ymm9, ymm12, ymm9
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm10, ymm12, ymm10
        
    
        
        
            vmulps ymm11, ymm12, ymm11
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_sub:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm0, ymm12, ymm0
        
    
        
        
            vsubps ymm1, ymm12, ymm1
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm2, ymm12, ymm2
        
    
        
        
            vsubps ymm3, ymm12, ymm3
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm4, ymm12, ymm4
        
    
        
        
            vsubps ymm5, ymm12, ymm5
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm6, ymm12, ymm6
        
    
        
        
            vsubps ymm7, ymm12, ymm7
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm8, ymm12, ymm8
        
    
        
        
            vsubps ymm9, ymm12, ymm9
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm10, ymm12, ymm10
        
    
        
        
            vsubps ymm11, ymm12, ymm11
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_sub_flipped:
    mov             rax, [ rdi + 8 ]








// 12 cols:6


    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm0, ymm0, ymm12
        
    
        
        
            vsubps ymm1, ymm1, ymm12
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm2, ymm2, ymm12
        
    
        
        
            vsubps ymm3, ymm3, ymm12
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm4, ymm4, ymm12
        
    
        
        
            vsubps ymm5, ymm5, ymm12
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm6, ymm6, ymm12
        
    
        
        
            vsubps ymm7, ymm7, ymm12
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm8, ymm8, ymm12
        
    
        
        
            vsubps ymm9, ymm9, ymm12
        
    

    vbroadcastss    ymm12, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm10, ymm10, ymm12
        
    
        
        
            vsubps ymm11, ymm11, ymm12
        
    


    jmp .non_linear_loop




.add_unicast:

    mov     r10,    [rdi + 8]           // c ptr
    mov     rsi,    [rdi + 16]          // row stride
    mov     rbx,    [rdi + 24]          // col stride

    mov     eax,    0

    pinsrd  xmm14, eax, 0
    add     eax,    esi

    pinsrd  xmm14, eax, 1
    add     eax,    esi

    pinsrd  xmm14, eax, 2
    add     eax,    esi

    pinsrd  xmm14, eax, 3
    add     eax,    esi


    pinsrd  xmm15, eax, 0
    add     eax,    esi

    pinsrd  xmm15, eax, 1
    add     eax,    esi

    pinsrd  xmm15, eax, 2
    add     eax,    esi

    pinsrd  xmm15, eax, 3
    add     eax,    esi


    vperm2f128      ymm14,  ymm14, ymm15,         32 // ymm14 <- xmm14::xmm15

    lea             r8, [ r10 + rsi * 8 ]


    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm0,   ymm0,   ymm12
    vaddps          ymm1, ymm1,   ymm13

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm2,   ymm2,   ymm12
    vaddps          ymm3, ymm3,   ymm13

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm4,   ymm4,   ymm12
    vaddps          ymm5, ymm5,   ymm13

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm6,   ymm6,   ymm12
    vaddps          ymm7, ymm7,   ymm13

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm8,   ymm8,   ymm12
    vaddps          ymm9, ymm9,   ymm13

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ],      ymm15
    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm13,  [ r8  + ymm14 ],      ymm15
    add     r10, rbx
    add     r8, rbx
    vaddps          ymm10,   ymm10,   ymm12
    vaddps          ymm11, ymm11,   ymm13


    jmp    .non_linear_loop

.add_row_col_products:
    mov             rax, [ rdi + 8 ]
    mov             rbx, [ rdi + 16 ]

    vmovups         ymm12,  [rax]
    vmovups         ymm13,  [rax + 32]


    vbroadcastss    ymm14, dword ptr [rbx + 0 ]
    vfmadd231ps     ymm0,   ymm12, ymm14
    vfmadd231ps     ymm1, ymm13, ymm14

    vbroadcastss    ymm14, dword ptr [rbx + 4 ]
    vfmadd231ps     ymm2,   ymm12, ymm14
    vfmadd231ps     ymm3, ymm13, ymm14

    vbroadcastss    ymm14, dword ptr [rbx + 8 ]
    vfmadd231ps     ymm4,   ymm12, ymm14
    vfmadd231ps     ymm5, ymm13, ymm14

    vbroadcastss    ymm14, dword ptr [rbx + 12 ]
    vfmadd231ps     ymm6,   ymm12, ymm14
    vfmadd231ps     ymm7, ymm13, ymm14

    vbroadcastss    ymm14, dword ptr [rbx + 16 ]
    vfmadd231ps     ymm8,   ymm12, ymm14
    vfmadd231ps     ymm9, ymm13, ymm14

    vbroadcastss    ymm14, dword ptr [rbx + 20 ]
    vfmadd231ps     ymm10,   ymm12, ymm14
    vfmadd231ps     ymm11, ymm13, ymm14

    jmp    .non_linear_loop

.store:
    mov     r8,     [rdi + 8]           // c ptr
    mov     rsi,    [rdi + 16]          // row stride
    mov     rbx,    [rdi + 24]          // col stride

    // tops of cols
    lea     r9,     [ r8 + rbx ]
    lea     r10,    [ r8 + 2 * rbx ]
    lea     r12,    [ r8 + 4 * rbx ]
    lea     r11,    [ r10 + rbx ]
    lea     r13,    [ r12 + rbx ]

    
        
        
            
                vextractps  dword ptr [r8], xmm0, 0
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 0
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 0
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 0
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 0
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 0
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 1
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 1
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 1
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 1
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 1
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 1
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 2
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 2
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 2
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 2
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 2
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 2
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 3
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 3
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 3
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 3
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 3
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 3
                add         r13, rsi
            
        
    
        
            // move next four rows at top (xmm0,2,..10)
            vperm2f128  ymm0,   ymm0,   ymm1,  1
            vperm2f128  ymm2,   ymm2,   ymm3,  1
            vperm2f128  ymm4,   ymm4,   ymm5,  1
            vperm2f128  ymm6,   ymm6,   ymm7,  1
            vperm2f128  ymm8,   ymm8,   ymm9,  1
            vperm2f128  ymm10,  ymm10,  ymm11, 1
        
        
            
                vextractps  dword ptr [r8], xmm0, 0
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 0
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 0
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 0
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 0
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 0
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 1
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 1
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 1
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 1
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 1
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 1
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 2
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 2
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 2
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 2
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 2
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 2
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 3
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 3
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 3
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 3
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 3
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 3
                add         r13, rsi
            
        
    
        
            // move next four rows at top (xmm0,2,..10)
            vperm2f128  ymm0,   ymm0,   ymm1,  2
            vperm2f128  ymm2,   ymm2,   ymm3,  2
            vperm2f128  ymm4,   ymm4,   ymm5,  2
            vperm2f128  ymm6,   ymm6,   ymm7,  2
            vperm2f128  ymm8,   ymm8,   ymm9,  2
            vperm2f128  ymm10,  ymm10,  ymm11, 2
        
        
            
                vextractps  dword ptr [r8], xmm0, 0
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 0
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 0
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 0
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 0
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 0
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 1
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 1
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 1
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 1
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 1
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 1
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 2
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 2
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 2
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 2
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 2
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 2
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 3
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 3
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 3
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 3
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 3
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 3
                add         r13, rsi
            
        
    
        
            // move next four rows at top (xmm0,2,..10)
            vperm2f128  ymm0,   ymm0,   ymm1,  3
            vperm2f128  ymm2,   ymm2,   ymm3,  3
            vperm2f128  ymm4,   ymm4,   ymm5,  3
            vperm2f128  ymm6,   ymm6,   ymm7,  3
            vperm2f128  ymm8,   ymm8,   ymm9,  3
            vperm2f128  ymm10,  ymm10,  ymm11, 3
        
        
            
                vextractps  dword ptr [r8], xmm0, 0
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 0
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 0
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 0
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 0
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 0
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 1
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 1
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 1
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 1
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 1
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 1
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 2
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 2
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 2
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 2
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 2
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 2
                add         r13, rsi
            
        
            
                vextractps  dword ptr [r8], xmm0, 3
                add         r8, rsi
            
                vextractps  dword ptr [r9], xmm2, 3
                add         r9, rsi
            
                vextractps  dword ptr [r10], xmm4, 3
                add         r10, rsi
            
                vextractps  dword ptr [r11], xmm6, 3
                add         r11, rsi
            
                vextractps  dword ptr [r12], xmm8, 3
                add         r12, rsi
            
                vextractps  dword ptr [r13], xmm10, 3
                add         r13, rsi
            
        
    

    jmp     .non_linear_loop

.q_scale:
    jmp .unsupported

.return:
    ldmxcsr     [rsp + 4]
    add         rsp, 8

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx



    mov rsp, rbp
    pop rbp
    ret


 
.cfi_endproc

