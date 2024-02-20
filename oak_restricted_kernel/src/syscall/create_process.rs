//
// Copyright 2024 The Project Oak Authors
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

use core::{
    ffi::{c_size_t, c_ssize_t, c_void},
    slice,
};

use oak_restricted_kernel_interface::Errno;
use x86_64::{
    structures::paging::{MappedPageTable, PageTable},
    PhysAddr, VirtAddr,
};

use crate::{
    mm::{
        encrypted_mapper::{EncryptedPageTable, PhysOffset},
        Translator,
    },
    payload::Process,
    PAGE_TABLES,
};

pub fn syscall_unstable_create_proccess(buf: *mut c_void, count: c_size_t) -> c_ssize_t {
    // We should validate that the pointer and count are valid, as these come from userspace and
    // therefore are not to be trusted, but right now everything is in kernel space so there is
    // nothing to check.
    let elf_binary_buffer = unsafe { slice::from_raw_parts(buf as *mut u8, count) };
    match unstable_create_proccess(elf_binary_buffer) {
        Ok(pid) => pid
            .try_into()
            .expect("pid so large, it could not be represented as isize"),
        Err(err) => err as isize,
    }
}

fn log_fail() {
    log::info!("getting virt");

    {
        let virt_addr = VirtAddr::new(0x0000000041cc54);

        // let phys = crate::PAGE_TABLES
        //     .lock()
        //     .get()
        //     .unwrap()
        //     .translate_virtual(virt_addr)
        //     .unwrap();

        // log::info!("phys {:?}", phys);

        // logs: "kernel INFO: virt VirtAddr(0x41cb94), p4 PageTableIndex(0), p3 PageTableIndex(0),
        // p2 PageTableIndex(2), p1 PageTableIndex(28)"
        log::info!(
            "virt {:?}, p4 {:?}, p3 {:?}, p2 {:?}, p1 {:?}",
            virt_addr,
            virt_addr.p4_index(),
            virt_addr.p3_index(),
            virt_addr.p2_index(),
            virt_addr.p1_index(),
        );
    };

    let mut pt = crate::PAGE_TABLES.lock();
    let mut binding: &mut EncryptedPageTable<MappedPageTable<'_, PhysOffset>> =
        pt.get_mut().unwrap().inner();
    let pml4 = {
        let mut pt_inner = binding.inner().lock();

        pt_inner.level_4_table().clone()
    };

    let pml3_entry_phys_addr = pml4.iter().nth(0).unwrap().addr();
    log::info!("pml3_entry_phys_addr addr {:?}", pml3_entry_phys_addr);
    let pml3_entry_virt_addr = binding.translate_physical(pml3_entry_phys_addr).unwrap();
    log::info!("pml3_entry_virt_addr addr {:?}", pml3_entry_virt_addr);

    let pml_3 = unsafe { &mut *(pml3_entry_virt_addr.as_u64() as *mut PageTable) };

    let pml2_entry_phys_addr = pml_3.iter().nth(0).unwrap().addr();
    log::info!("pml2_entry_phys_addr addr {:?}", pml2_entry_phys_addr);
    let pml2_entry_virt_addr = binding.translate_physical(pml2_entry_phys_addr).unwrap();
    log::info!("pml2_entry_virt_addr addr {:?}", pml2_entry_virt_addr);

    let pml_2 = unsafe { &mut *(pml2_entry_virt_addr.as_u64() as *mut PageTable) };

    let pml2_entry = pml_2.iter().nth(3).unwrap();
    log::info!("pml2_entry is_unused {:?}", pml2_entry.is_unused());
    log::info!("pml2_entry_phys_addr {:?}", pml2_entry.addr());

    if !pml2_entry.is_unused() {
        let addr = binding.translate_physical(pml2_entry.addr()).unwrap();
        log::info!("pml2_entry_virt_addr {:?}", addr);
    }
}

fn unstable_create_proccess(buf: &[u8]) -> Result<usize, Errno> {
    // Copy the ELF file into kernel space.
    let copied_elf_binary: alloc::vec::Vec<u8> = buf.to_vec();

    let application =
        crate::payload::Application::new(copied_elf_binary.clone().into_boxed_slice())
            .inspect_err(|err| log::error!("failed to create application: {:?}", err))
            .map_err(|_| Errno::EINVAL)?;

    {
        let virt_addr = VirtAddr::new(0x0000000041cb94);
        // logs: "kernel INFO: virt VirtAddr(0x41cb94), p4 PageTableIndex(0), p3 PageTableIndex(0),
        // p2 PageTableIndex(2), p1 PageTableIndex(28)"
        log::info!(
            "virt {:?}, p4 {:?}, p3 {:?}, p2 {:?}, p1 {:?}",
            virt_addr,
            virt_addr.p4_index(),
            virt_addr.p3_index(),
            virt_addr.p2_index(),
            virt_addr.p1_index(),
        );
    };

    {
        let virt_addr = VirtAddr::new(0x0000000041cb94);

        let phys = crate::PAGE_TABLES
            .lock()
            .get()
            .unwrap()
            .translate_virtual(virt_addr)
            .unwrap();

        log::info!("phys {:?}", phys);
        // logs: "kernel INFO: virt VirtAddr(0x41cb94), p4 PageTableIndex(0), p3 PageTableIndex(0),
        // p2 PageTableIndex(2), p1 PageTableIndex(28)"
        log::info!(
            "virt {:?}, p4 {:?}, p3 {:?}, p2 {:?}, p1 {:?}",
            virt_addr,
            virt_addr.p4_index(),
            virt_addr.p3_index(),
            virt_addr.p2_index(),
            virt_addr.p1_index(),
        );
    };

    log_fail();

    // Safety: application is assumed to be a valid ELF file.
    let pid = unsafe { Process::from_application(&application).expect("failed to create process") };

    for (a, b) in buf.iter().zip(copied_elf_binary.iter()) {
        assert!(a == b)
    }

    log_fail();

    Ok(pid)
}
