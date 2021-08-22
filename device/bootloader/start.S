.cpu cortex-m0plus
.thumb

.section .text

.global start
.type start,%function
.thumb_func
start:
    ldr r0, =bss_start
    ldr r1, =bss_end
    movs r2, #0
0:
    cmp r0, r1
    beq 1f
    stm r0!, {r2}
    b 0b
1:
    ldr r0,=data_start
    ldr r1,=data_end
    ldr r2,=data_source
2:
    cmp r0, r1
    beq 3f
    ldm r2!, {r3}
    stm r0!, {r3}
    b 2b
3:
    ldr r0, =interrupt_vector
    ldr r1, =0xE000ED08 // (PPB_BASE + M0PLUS_VTOR_OFFSET)
    str r0, [r1]
    ldr r0, =stack_start
    msr msp, r0
    b main

.end