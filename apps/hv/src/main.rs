#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate libax;

use libax::hv::{GuestPageTable, HyperCraftHalImpl, PerCpu, VmCpus, VmTrait, VM};

mod dtb;
mod gpm;

use gpm::setup_gpm;
#[no_mangle]

/// For any single core Hypervisor，this is the same.
fn main(hart_id: usize) {
    println!("Hello, hv!");
    // boot cpu
    let _ = PerCpu::<HyperCraftHalImpl>::init(0, 0x4000);

    // get current percpu
    let pcpu = PerCpu::<HyperCraftHalImpl>::this_cpu();

    // create vcpu
    let gpt = setup_gpm(0x9000_0000).unwrap();
    // Entry 说明起始地址在 0x9020_0000
    let vcpu = pcpu.create_vcpu(0, 0x9020_0000).unwrap();

    let mut vcpus = VmCpus::new();

    // add vcpu into vm
    vcpus.add_vcpu(vcpu).unwrap();
    // because of this line, we need to use libax::hv::VmTrait
    let mut vm: VM<HyperCraftHalImpl, GuestPageTable> = VM::new(vcpus, gpt).unwrap();
    vm.init_vcpu(0);

    // vm run
    info!("vm run cpu{}", hart_id);
    vm.run(0);
}
