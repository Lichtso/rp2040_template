/*
 * Copyright (c) 2021 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#ifndef _BOOT2_HELPER_EXIT_FROM_BOOT2
#define _BOOT2_HELPER_EXIT_FROM_BOOT2

#include "hardware/regs/m0plus.h"

// If entered from the bootrom, lr (which we earlier pushed) will be 0.
// Any regular function call will have a nonzero value for lr.
check_return:
    pop {r0}
    cmp r0, #0
    bne exit_to_program
    ldr r0, =(XIP_BASE + 0x1A9)
exit_to_program:
    bx r0
    // ldr r1, =(PPB_BASE + M0PLUS_VTOR_OFFSET)
    // str r0, [r1]
    // ldmia r0, {r0, r1}
    // msr msp, r0
    // bx r1

#endif
