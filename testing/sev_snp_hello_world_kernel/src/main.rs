//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![no_std]
#![no_main]

use core::{mem::MaybeUninit, panic::PanicInfo};
use sev_guest::{cpuid::CpuidPage, msr, secrets::SecretsPage};
use x86_64::instructions::{hlt, interrupts::int3};

mod asm;
mod ghcb;
mod serial;

#[link_section = ".cpuid"]
static CPUID: MaybeUninit<CpuidPage> = MaybeUninit::uninit();

#[link_section = ".secrets"]
static SECRETS: MaybeUninit<SecretsPage> = MaybeUninit::uninit();

#[no_mangle]
pub extern "C" fn rust64_start() -> ! {
    serial::init_logging();
    log::info!("Hello World!");

    // Safety: these data structures are placed in valid memory so we won't page fault when
    // accessing them and the CPU is supposed to initialize them when running under SEV-SNP. If
    // we're not running under SEV-SNP, we will read garbage from uninialized memory.
    let cpuid: &CpuidPage = unsafe { CPUID.assume_init_ref() };
    let secrets: &SecretsPage = unsafe { SECRETS.assume_init_ref() };

    log::info!("CPUID page: {:?}", cpuid);
    log::info!("Secrets page: {:?}", secrets);
    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if msr::get_sev_status()
        .unwrap()
        .contains(msr::SevStatus::SEV_ES_ENABLED)
    {
        msr::request_termination(msr::TerminationRequest {
            reason: msr::TerminationReason::General,
        });
    } else {
        // Trigger a breakpoint exception. As we don't have a #BP handler, this will triple fault
        // and terminate the program.
        int3();

        // ..but if we're still here, just go in an infinite loop.
        loop {
            hlt();
        }
    }
}