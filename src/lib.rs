// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

//! This crate provides helper functions for stack analysis on AVR.
//!
//! [estimate_unused_stack_space]:
//! Estimate the number of stack bytes that have never been used.
//!
//! # Initialization of stack space
//!
//! The main crate must call the [init_stack_pattern] macro once
//! to define the stack initialization function.
//!
//! On init, all of the stack space is overwritten with a byte [PATTERN] by the macro.
//! The code that does this runs from the linker section `.init4`.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))]

/// Memory pattern for unused stack space.
///
/// The unused stack space is filled with this byte pattern.
pub const PATTERN: u8 = 0x5A;

/// Define an `.init4` function to initialize the stack.
///
/// This macro shall be called once from the main crate to define
/// an `.init4` function to overwrite the whole stack with [PATTERN].
#[macro_export]
macro_rules! init_stack_pattern {
    () => {
        #[cfg(target_arch = "avr")]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".init4")]
        /// Overwrite the whole stack with the [PATTERN].
        ///
        /// The stack grows downwards.
        /// Start from stack end and iterate upwards to the stack beginning.
        ///
        /// # Safety
        ///
        /// This naked function is run before main() from the .init4 section.
        unsafe extern "C" fn __avr_stack__mark_pattern() {
            core::arch::naked_asm!(
                "   ldi r26, lo8(__bss_end)",   // X = stack end
                "   ldi r27, hi8(__bss_end)",   // ...
                "   ldi r17, hi8(__stack)",     // stack begin (high byte)
                "   ldi r18, {PATTERN}",        // initialization byte pattern
                "1: cpi r26, lo8(__stack)",     // check if we reached stack begin
                "   cpc r27, r17",              // ...
                "   st X+, r18",                // write pattern to stack and inc X
                "   brne 1b",                   // repeat if not at stack begin

                PATTERN = const $crate::PATTERN,
            );
        }
    };
}

#[cfg(target_arch = "avr")]
#[inline(always)]
fn avr_estimate_unused_stack_space() -> u16 {
    let mut nrbytes;

    // SAFETY: The assembly code only does atomic memory reads.
    unsafe {
        core::arch::asm!(
            "   ldi r26, lo8(__bss_end)",               // X = stack end
            "   ldi r27, hi8(__bss_end)",               // ...
            "   ldi r18, hi8(__stack)",                 // stack begin (high byte)
            "1: cpi r26, lo8(__stack)",                 // check if we reached stack begin
            "   cpc r27, r18",                          // ...
            "   breq 2f",                               // reached stack begin -> done
            "   ld r19, X+",                            // read the stack byte and inc X
            "   cpi r19, {PATTERN}",                    // check if the read bytes still matches PATTERN
            "   breq 1b",                               // if it matches, go on searching
            "2: movw {nrbytes}, r26",                   // first mismatch. Copy X
            "   subi {nrbytes:l}, lo8(__bss_end + 1)",  // number of bytes is X minus stack-end minus 1
            "   sbci {nrbytes:h}, hi8(__bss_end + 1)",  // ...

            nrbytes = out(reg_pair) nrbytes,            // nrbytes is output only

            out("r18") _,                               // stack begin high byte
            out("r19") _,                               // stack content temporary
            out("r26") _,                               // X low
            out("r27") _,                               // X high

            PATTERN = const PATTERN,
        );
    }

    nrbytes
}

/// Returns the number of stack bytes that have never been written to.
///
/// This function can be called at any time.
///
/// This function walks the stack from end to beginning
/// and checks if the initialization [PATTERN] is still there.
/// The number of bytes that still contain the [PATTERN] is returned.
///
/// The returned value is only an estimate.
/// If the program code wrote [PATTERN] bytes to the stack, then
/// these bytes will falsely be seen as unused.
/// It is assumed that this scenario is unlikely to occur for more than a couple of bytes
/// and therefore the estimation is expected to be off by no more than a couple of bytes.
///
/// This function does not protect against stack overflows.
/// If an actual stack overflow occured, the behavior is undefined.
#[inline(always)]
pub fn estimate_unused_stack_space() -> u16 {
    #[cfg(target_arch = "avr")]
    let nrbytes = avr_estimate_unused_stack_space();

    #[cfg(not(target_arch = "avr"))]
    let nrbytes = 0;

    nrbytes
}

// vim: ts=4 sw=4 expandtab
