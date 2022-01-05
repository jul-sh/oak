



.intel_syntax noprefix
.text
.p2align 5
.globl fma_mmm_f32_64x1_0_15_9_pre
fma_mmm_f32_64x1_0_15_9_pre:
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

// vim: set syntax=asm :

.non_linear:

.non_linear_loop_enter:
    sub     rdi,    40
.non_linear_loop:
    add     rdi,    40
    mov     rax,    [rdi]

    mov     r8, 25
    cmp     rax, 0
    cmovl   rax, r8
    cmp     rax, 25
    cmovg   rax, r8


    lea     r8, [ rip + .jmp_table ]

    movsxd  r9, dword ptr [ r8 + rax * 4 ]
    lea     r8, [ r8 + r9 ]
    jmp     r8

.jmp_table:

    .long      .done-.jmp_table

    .long      .clear-.jmp_table

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



.clear:
    vzeroall
    jmp     .non_linear_loop

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
 
.main_loop_packed_tops_and_offsets:
    mov             rsi,    [rbx]   // rsi: current row offset
    vbroadcastss    ymm14,  dword ptr [r8 + rsi]


    vmovaps         ymm12, [rax + 0]
    vfmadd231ps     ymm0, ymm12, ymm14

    vmovaps         ymm12, [rax + 32]
    vfmadd231ps     ymm1, ymm12, ymm14

    vmovaps         ymm12, [rax + 64]
    vfmadd231ps     ymm2, ymm12, ymm14

    vmovaps         ymm12, [rax + 96]
    vfmadd231ps     ymm3, ymm12, ymm14

    vmovaps         ymm12, [rax + 128]
    vfmadd231ps     ymm4, ymm12, ymm14

    vmovaps         ymm12, [rax + 160]
    vfmadd231ps     ymm5, ymm12, ymm14

    vmovaps         ymm12, [rax + 192]
    vfmadd231ps     ymm6, ymm12, ymm14

    vmovaps         ymm12, [rax + 224]
    vfmadd231ps     ymm7, ymm12, ymm14


    add             rbx, 8
    add             rax, 256
    dec             rcx
    jnz             .main_loop_packed_tops_and_offsets

    jmp             .non_linear_loop

.packed_packed:
    mov     rbx,   [rbx + 8] // B 

.main_loop_packed_packed:
    vbroadcastss    ymm14,  dword ptr [rbx]


    vmovaps         ymm12, [rax + 0]
    vfmadd231ps     ymm0, ymm12, ymm14

    vmovaps         ymm12, [rax + 32]
    vfmadd231ps     ymm1, ymm12, ymm14

    vmovaps         ymm12, [rax + 64]
    vfmadd231ps     ymm2, ymm12, ymm14

    vmovaps         ymm12, [rax + 96]
    vfmadd231ps     ymm3, ymm12, ymm14

    vmovaps         ymm12, [rax + 128]
    vfmadd231ps     ymm4, ymm12, ymm14

    vmovaps         ymm12, [rax + 160]
    vfmadd231ps     ymm5, ymm12, ymm14

    vmovaps         ymm12, [rax + 192]
    vfmadd231ps     ymm6, ymm12, ymm14

    vmovaps         ymm12, [rax + 224]
    vfmadd231ps     ymm7, ymm12, ymm14


    add             rbx,    4
    add             rax,    256
    dec             rcx
    jnz             .main_loop_packed_packed

    jmp             .non_linear_loop

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
        
    

    jmp    .non_linear_loop



// vim: set syntax=asm :

// vim: set syntax=asm :

.per_row_min:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vminps ymm0, ymm8, ymm0
    
        vminps ymm1, ymm9, ymm1
    
        vminps ymm2, ymm10, ymm2
    
        vminps ymm3, ymm11, ymm3
    
        vminps ymm4, ymm12, ymm4
    
        vminps ymm5, ymm13, ymm5
    
        vminps ymm6, ymm14, ymm6
    
        vminps ymm7, ymm15, ymm7
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_max:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vmaxps ymm0, ymm8, ymm0
    
        vmaxps ymm1, ymm9, ymm1
    
        vmaxps ymm2, ymm10, ymm2
    
        vmaxps ymm3, ymm11, ymm3
    
        vmaxps ymm4, ymm12, ymm4
    
        vmaxps ymm5, ymm13, ymm5
    
        vmaxps ymm6, ymm14, ymm6
    
        vmaxps ymm7, ymm15, ymm7
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_add:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vaddps ymm0, ymm8, ymm0
    
        vaddps ymm1, ymm9, ymm1
    
        vaddps ymm2, ymm10, ymm2
    
        vaddps ymm3, ymm11, ymm3
    
        vaddps ymm4, ymm12, ymm4
    
        vaddps ymm5, ymm13, ymm5
    
        vaddps ymm6, ymm14, ymm6
    
        vaddps ymm7, ymm15, ymm7
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_mul:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vmulps ymm0, ymm8, ymm0
    
        vmulps ymm1, ymm9, ymm1
    
        vmulps ymm2, ymm10, ymm2
    
        vmulps ymm3, ymm11, ymm3
    
        vmulps ymm4, ymm12, ymm4
    
        vmulps ymm5, ymm13, ymm5
    
        vmulps ymm6, ymm14, ymm6
    
        vmulps ymm7, ymm15, ymm7
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_sub:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vsubps ymm0, ymm8, ymm0
    
        vsubps ymm1, ymm9, ymm1
    
        vsubps ymm2, ymm10, ymm2
    
        vsubps ymm3, ymm11, ymm3
    
        vsubps ymm4, ymm12, ymm4
    
        vsubps ymm5, ymm13, ymm5
    
        vsubps ymm6, ymm14, ymm6
    
        vsubps ymm7, ymm15, ymm7
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_row_sub_flipped:
    mov             rax, [ rdi + 8 ]





    vmovups         ymm8,  [rax + 0]

    vmovups         ymm9,  [rax + 32]

    vmovups         ymm10,  [rax + 64]

    vmovups         ymm11,  [rax + 96]

    vmovups         ymm12,  [rax + 128]

    vmovups         ymm13,  [rax + 160]

    vmovups         ymm14,  [rax + 192]

    vmovups         ymm15,  [rax + 224]



    
        vsubps ymm0, ymm0, ymm8
    
        vsubps ymm1, ymm1, ymm9
    
        vsubps ymm2, ymm2, ymm10
    
        vsubps ymm3, ymm3, ymm11
    
        vsubps ymm4, ymm4, ymm12
    
        vsubps ymm5, ymm5, ymm13
    
        vsubps ymm6, ymm6, ymm14
    
        vsubps ymm7, ymm7, ymm15
    


    jmp .non_linear_loop



// vim: set syntax=asm :

// vim: set syntax=asm :

.per_col_min:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vminps ymm0, ymm8, ymm0
        
    
        
        
            vminps ymm1, ymm8, ymm1
        
    
        
        
            vminps ymm2, ymm8, ymm2
        
    
        
        
            vminps ymm3, ymm8, ymm3
        
    
        
        
            vminps ymm4, ymm8, ymm4
        
    
        
        
            vminps ymm5, ymm8, ymm5
        
    
        
        
            vminps ymm6, ymm8, ymm6
        
    
        
        
            vminps ymm7, ymm8, ymm7
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_max:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmaxps ymm0, ymm8, ymm0
        
    
        
        
            vmaxps ymm1, ymm8, ymm1
        
    
        
        
            vmaxps ymm2, ymm8, ymm2
        
    
        
        
            vmaxps ymm3, ymm8, ymm3
        
    
        
        
            vmaxps ymm4, ymm8, ymm4
        
    
        
        
            vmaxps ymm5, ymm8, ymm5
        
    
        
        
            vmaxps ymm6, ymm8, ymm6
        
    
        
        
            vmaxps ymm7, ymm8, ymm7
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_add:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vaddps ymm0, ymm8, ymm0
        
    
        
        
            vaddps ymm1, ymm8, ymm1
        
    
        
        
            vaddps ymm2, ymm8, ymm2
        
    
        
        
            vaddps ymm3, ymm8, ymm3
        
    
        
        
            vaddps ymm4, ymm8, ymm4
        
    
        
        
            vaddps ymm5, ymm8, ymm5
        
    
        
        
            vaddps ymm6, ymm8, ymm6
        
    
        
        
            vaddps ymm7, ymm8, ymm7
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_mul:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vmulps ymm0, ymm8, ymm0
        
    
        
        
            vmulps ymm1, ymm8, ymm1
        
    
        
        
            vmulps ymm2, ymm8, ymm2
        
    
        
        
            vmulps ymm3, ymm8, ymm3
        
    
        
        
            vmulps ymm4, ymm8, ymm4
        
    
        
        
            vmulps ymm5, ymm8, ymm5
        
    
        
        
            vmulps ymm6, ymm8, ymm6
        
    
        
        
            vmulps ymm7, ymm8, ymm7
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_sub:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm0, ymm8, ymm0
        
    
        
        
            vsubps ymm1, ymm8, ymm1
        
    
        
        
            vsubps ymm2, ymm8, ymm2
        
    
        
        
            vsubps ymm3, ymm8, ymm3
        
    
        
        
            vsubps ymm4, ymm8, ymm4
        
    
        
        
            vsubps ymm5, ymm8, ymm5
        
    
        
        
            vsubps ymm6, ymm8, ymm6
        
    
        
        
            vsubps ymm7, ymm8, ymm7
        
    


    jmp .non_linear_loop

// vim: set syntax=asm :

.per_col_sub_flipped:
    mov             rax, [ rdi + 8 ]








// 8 cols:1


    vbroadcastss    ymm8, dword ptr [ rax ]
    add             rax, 4

    
        
        
            vsubps ymm0, ymm0, ymm8
        
    
        
        
            vsubps ymm1, ymm1, ymm8
        
    
        
        
            vsubps ymm2, ymm2, ymm8
        
    
        
        
            vsubps ymm3, ymm3, ymm8
        
    
        
        
            vsubps ymm4, ymm4, ymm8
        
    
        
        
            vsubps ymm5, ymm5, ymm8
        
    
        
        
            vsubps ymm6, ymm6, ymm8
        
    
        
        
            vsubps ymm7, ymm7, ymm8
        
    


    jmp .non_linear_loop




.add_unicast:

    mov     r10,    [rdi + 8]           // c ptr
    mov     rsi,    [rdi + 16]          // row stride

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


    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm0,   ymm0,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm1,   ymm1,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm2,   ymm2,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm3,   ymm3,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm4,   ymm4,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm5,   ymm5,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm6,   ymm6,   ymm12
    lea             r10, [ r10 + rsi * 8 ]

    vpcmpeqd        ymm15,  ymm15, ymm15
    vgatherdps      ymm12,  [ r10 + ymm14 ], ymm15

    vaddps          ymm7,   ymm7,   ymm12
    lea             r10, [ r10 + rsi * 8 ]


    jmp    .non_linear_loop

.add_row_col_products:
    mov             rax, [ rdi + 8 ]
    mov             rbx, [ rdi + 16 ]

    vbroadcastss    ymm14, dword ptr [rbx]


    vmovups         ymm12,  [rax + 0]
    vfmadd231ps     ymm0, ymm12, ymm14

    vmovups         ymm12,  [rax + 32]
    vfmadd231ps     ymm1, ymm12, ymm14

    vmovups         ymm12,  [rax + 64]
    vfmadd231ps     ymm2, ymm12, ymm14

    vmovups         ymm12,  [rax + 96]
    vfmadd231ps     ymm3, ymm12, ymm14

    vmovups         ymm12,  [rax + 128]
    vfmadd231ps     ymm4, ymm12, ymm14

    vmovups         ymm12,  [rax + 160]
    vfmadd231ps     ymm5, ymm12, ymm14

    vmovups         ymm12,  [rax + 192]
    vfmadd231ps     ymm6, ymm12, ymm14

    vmovups         ymm12,  [rax + 224]
    vfmadd231ps     ymm7, ymm12, ymm14

    jmp    .non_linear_loop

.store:
    mov     r8,     [rdi + 8]           // c ptr
    mov     rsi,    [rdi + 16]          // row stride

    
        
            
                movaps xmm9, xmm0
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm0, ymm0, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm1, ymm1, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm2
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm2, ymm2, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm3
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm3, ymm3, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm4
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm4, ymm4, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm5
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm5, ymm5, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm6
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm6, ymm6, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    
        
            
                movaps xmm9, xmm7
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
            
                vperm2f128 ymm9, ymm7, ymm7, 1
            
            
                vextractps  dword ptr [r8], xmm9, 0
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 1
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 2
                add         r8, rsi
            
                vextractps  dword ptr [r8], xmm9, 3
                add         r8, rsi
            
        
    

    jmp    .non_linear_loop

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

