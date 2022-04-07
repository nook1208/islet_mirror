use crate::cpu::get_cpu_id;
use crate::helper::{SPSR_EL2, TPIDR_EL2};
use monitor::realm::vcpu::VCPU;

#[repr(C)]
#[derive(Default, Debug)]
pub struct Context {
    pub gp_regs: [u64; 31],
    pub elr: u64,
    pub spsr: u64,
    pub sys_regs: SystemRegister,
    pub fp_regs: [u128; 32],
}

impl monitor::realm::vcpu::Context for Context {
    fn new() -> Self {
        let mut context: Self = Default::default();

        // Set appropriate sys registers
        context.spsr =
            SPSR_EL2::D | SPSR_EL2::A | SPSR_EL2::I | SPSR_EL2::F | (SPSR_EL2::M & 0b0101);

        // TODO: enable floating point
        // CPTR_EL2, CPACR_EL1, update vectors.s, etc..

        context
    }

    unsafe fn into_current(vcpu: &mut VCPU<Self>) {
        let before = TPIDR_EL2.get();
        if before != 0 {
            let old = &mut *(before as *mut VCPU<Context>);
            old.from_current();
        }

        vcpu.pcpu = Some(get_cpu_id());
        vcpu.context.sys_regs.vmpidr = vcpu.pcpu.unwrap() as u64;
        TPIDR_EL2.set(vcpu as *const _ as u64);
    }

    unsafe fn from_current(vcpu: &mut VCPU<Self>) {
        vcpu.pcpu = None;
        vcpu.context.sys_regs.vmpidr = 0u64;
        TPIDR_EL2.set(0u64);
    }
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct SystemRegister {
    pub sp: u64,
    pub sp_el0: u64,
    pub esr_el1: u64,
    pub vbar: u64,
    pub ttbr0: u64,
    pub ttbr1: u64,
    pub mair: u64,
    pub amair: u64,
    pub tcr: u64,
    pub tpidr: u64,
    pub tpidr_el0: u64,
    pub tpidrro: u64,
    pub actlr: u64,
    pub vmpidr: u64,
    pub csselr: u64,
    pub cpacr: u64,
    pub afsr0: u64,
    pub afsr1: u64,
    pub far: u64,
    pub contextidr: u64,
    pub cntkctl: u64,
    pub par: u64,
    pub vttbr: u64,
    pub esr_el2: u64,
    pub hpfar: u64,
}