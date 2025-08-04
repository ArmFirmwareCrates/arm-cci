// SPDX-FileCopyrightText: Copyright The arm-cci Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

use bitflags::bitflags;
pub use safe_mmio::UniqueMmioPointer;
use safe_mmio::{
    field, field_shared,
    fields::{ReadPure, ReadPureWrite},
};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Control Override Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct ControlOverrideRegister(u32);

/// Secure Access Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct SecureAccessRegister(u32);

/// Status Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct StatusRegister(u32);

/// Snoop Control Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct SnoopControlRegister(u32);

bitflags! {
    impl ControlOverrideRegister: u32 {
        /// CCI-550 only, Enable both snoop data layers for data transfer between ACE masters.
        const CD_LAYER2_ACE_ENABLE = 1 << 4;
        /// Disable regional clock gating
        const DISABLE_CLOCK_GATING = 1 << 3;
        /// Disable the snoop filter
        const DISABLE_SNOOP_FILTER = 1 << 2;
        /// DVM message disable
        const DVM_MESSAGE_DISABLE = 1 << 1;
        /// Snoop disable
        const SNOOP_DISABLE = 1 << 0;
    }

    impl SecureAccessRegister: u32 {
        /// Enable the PMU to count both Secure and Non-secure events.
        const SECURE_OBSERVATION_OVERRIDE = 1 << 2;
        /// Disable Non-secure access to the PMU and Interface Monitor Registers, unless overridden
        /// by `NON_SECURE_REGISTER_ACCESS_OVERRIDE`.
        const DEBUG_MONITOR_SECURITY_OVERRIDE = 1 << 1;
        /// Enable Non-secure access to the CCI-550 registers.
        const NON_SECURE_REGISTER_ACCESS_OVERRIDE = 1 << 0;
    }

    impl StatusRegister: u32 {
        /// Snoop filter RAM power state change pending.
        const SF_RAM_STATE_CHANGE_PENDING = 1 << 8;
        /// Indicates when the snoop filter RAM is initialized.
        const SF_RAM_INITIALIZATION = 1 << 1;
        /// Indicates whether any changes to the Snoop Control Registers or the Control Override
        /// Register are pending:
        const CHANGE_PENDING = 1 << 0;
    }

    impl SnoopControlRegister: u32 {
        /// Indicates whether the slave interface supports DVM messages.
        const SUPPORT_DVMS = 1 << 31;
        /// Indicates whether the slave interface supports snoop requests.
        const SUPPORT_SNOOPS = 1 << 30;
        /// CCI-550 only, Indicates whether the slave interface has a system coherency interface to
        /// provide hardware snoop enable control.
        const HARDWARE_SNOOP_ENABLE_CONTROL = 1 << 29;
        /// Enables issuing of DVM message requests from the slave interface.
        const ENABLE_DVMS = 1 << 1;
        /// Enables issuing of snoop requests from the slave interface.
        const ENABLE_SNOOP = 1 << 0;
    }
}

/// CCI base registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CciBaseRegisters {
    /// 0x000: Control Override Register
    ctrl_ovr: ReadPureWrite<ControlOverrideRegister>,
    /// 0x004
    reserved_004: u32,
    /// 0x008: Secure Access Register
    secr_acc: ReadPureWrite<SecureAccessRegister>,
    /// 0x00C: Status Register
    status: ReadPure<StatusRegister>,
    /// 0x010: Imprecise Error Register
    impr_error: ReadPureWrite<u32>,
    /// 0x014: QoS Threshold Register
    qos_threshold: ReadPureWrite<u32>,
    /// 0x018 - 0x0FC
    reserved_018: [u32; 58],
    /// 0x0100: Performance Monitor Control Register
    pmu_ctrl: ReadPure<u32>,
    /// 0x0104: Interface Monitor Control Register
    debug_ctrl: ReadPureWrite<u32>,
    /// 0x108 - 0xFCC
    reserved_108: [u32; 946],
    /// 0xFD0: Peripheral ID4
    peripheral_id4: ReadPure<u32>,
    /// 0xFD4: Peripheral ID5
    peripheral_id5: ReadPure<u32>,
    /// 0xFD8: Peripheral ID6
    peripheral_id6: ReadPure<u32>,
    /// 0xFDC: Peripheral ID7
    peripheral_id7: ReadPure<u32>,
    /// 0xFE0: Peripheral ID0
    peripheral_id0: ReadPure<u32>,
    /// 0xFE4: Peripheral ID1
    peripheral_id1: ReadPure<u32>,
    /// 0xFE8: Peripheral ID2
    peripheral_id2: ReadPure<u32>,
    /// 0xFEC: Peripheral ID3
    peripheral_id3: ReadPure<u32>,
    /// 0xFF0: Component ID0
    component_id0: ReadPure<u32>,
    /// 0xFF4: Component ID1
    component_id1: ReadPure<u32>,
    /// 0xFF8: Component ID2
    component_id2: ReadPure<u32>,
    /// 0xFFC: Component ID3
    component_id3: ReadPure<u32>,
}

/// Slave interface registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CciSlaveRegisters {
    /// 0x000: Snoop Control Register
    snoop_ctrl: ReadPureWrite<SnoopControlRegister>,
    /// 0x004: Shareable Override Register
    share_ovr: ReadPureWrite<u32>,
    /// 0x008 - 0x0FC
    reserved_008: [u32; 62],
    /// 0x100: Read Channel QoS Value Override Register
    arqos_ovr: ReadPureWrite<u32>,
    /// 0x104: Write Channel QoS Value Override Register
    awqos_ovr: ReadPureWrite<u32>,
    reserved_108: [u32; 2],
    /// 0x110: Max OT Register
    qos_max_ot: ReadPureWrite<u32>,
    /// 0x114 - 0xFFC
    reserved_13c: [u32; 955],
}

/// Performance counter registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CciPerformanceCounterRegisters {
    /// 0x000: Event Select Register
    evnt_sel: ReadPureWrite<u32>,
    /// 0x004: Event Count Register
    ecnt_data: ReadPureWrite<u32>,
    /// 0x008: Count Control Register
    ecnt_ctlr: ReadPureWrite<u32>,
    /// 0x00c: Overflow Flag Status Register
    ecnt_clr_ovfl: ReadPureWrite<u32>,
    /// 0x010 - 0xFFC
    reserved_10: [u32; 16380],
}

/// Slave Interface Monitor Registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CciSlaveInterfaceMonitorRegisters {
    /// 0x000 - Slave Interface Monitor Registers
    slave_debug: [ReadPureWrite<u32>; 7],
    reserved: [u32; 57],
}

/// Master Interface Monitor Registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CciMasterInterfaceMonitorRegisters {
    /// Master Interface Monitor Registers
    master_debug: [ReadPureWrite<u32>; 6],
    reserved: [u32; 58],
}

/// CCI-500/CCI-550 registers
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct Cci5x0Registers {
    /// 0x00000: Base registers
    base_regs: CciBaseRegisters,
    /// 0x01000 - 0x07FFC: Slave interface 0-6 registers
    slaves: [CciSlaveRegisters; 7],
    /// 0x08000 - 0x0FFFC
    reserved_8000: [u32; 8192],
    /// 0x10000 - 0x8FFFC: Performance counter 0-7 registers
    performance_counters: [CciPerformanceCounterRegisters; 8],
    /// 0x90000 - 0x900FC: Slave Interface Monitor Registers
    slave_interface_monitor: CciSlaveInterfaceMonitorRegisters,
    /// 0x90100 - 0x901FC: Master Interface Monitor Registers
    master_interface_monitor: CciMasterInterfaceMonitorRegisters,
    // 0x90200 - 0xA0000
    reserved_90200: [u32; 16256],
}

/// CCI-500/CCI-550 driver.
#[derive(Eq, Debug, PartialEq)]
pub struct Cci5x0<'a> {
    regs: UniqueMmioPointer<'a, Cci5x0Registers>,
}

impl<'a> Cci5x0<'a> {
    pub fn new(regs: UniqueMmioPointer<'a, Cci5x0Registers>) -> Self {
        Self { regs }
    }

    /// Adds master to cache coherency domain.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the master only enables allocating shareable data into its cache
    /// after this function completes.
    pub unsafe fn add_master_to_coherency(&mut self, index: usize) {
        // Safety: Enabling DVM and snoop propagates the same requirements to the caller of this
        // function. The function waits until the transaction completes by calling
        // wait_pending_change.
        unsafe {
            self.write_snoop_control(
                index,
                SnoopControlRegister::ENABLE_DVMS | SnoopControlRegister::ENABLE_SNOOP,
            );
        }

        self.wait_pending_change();
    }

    /// Removes master from cache coherency domain.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the master is configured so that it does not allocate shareable
    /// data into its cache, for example by disabling the data cache. The caller also has to clean
    /// and invalidate all shareable data from the caches in the master prior calling this function.
    pub unsafe fn remove_master_from_coherency(&mut self, index: usize) {
        // Safety: Disabling DVM and snoop propagates the same requirements to the caller of this
        // function.
        unsafe {
            self.write_snoop_control(index, SnoopControlRegister::empty());
        }
        self.wait_pending_change();
    }

    /// Writes the snoop control register of the slave interface.
    ///
    /// # Safety
    ///
    /// When disabling snoop or DVM messages the caller must ensure that the master is configured so
    /// that it does not allocate shareable data into its cache, for example by disabling the data
    /// cache. The caller also has to clean and invalidate all shareable data from the caches in the
    /// master prior calling this function.
    /// When enabling snoop or DVM messages, the caller must ensure that the master only enables
    /// allocating shareable data into its cache after this transaction completes.
    unsafe fn write_snoop_control(&mut self, index: usize, snoop_control: SnoopControlRegister) {
        let mut slaves = field!(self.regs, slaves);
        let mut slave = slaves.get(index).unwrap();

        field!(slave, snoop_ctrl).write(snoop_control);

        // Execute a barrier instruction to ensure that the previous step is complete.
        // Safety: Synchronization barrier only.
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!(
                "isb
                dsb sy",
                options(nomem, nostack)
            );
        }
    }

    /// Wait until there are any pending changes to the Snoop Control Registers or the Control
    /// Override Register
    fn wait_pending_change(&self) {
        let base_regs = field_shared!(self.regs, base_regs);
        while field_shared!(base_regs, status)
            .read()
            .contains(StatusRegister::CHANGE_PENDING)
        {
            core::hint::spin_loop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(0x1000, core::mem::size_of::<CciBaseRegisters>());
        assert_eq!(0x1000, core::mem::size_of::<CciSlaveRegisters>());
        assert_eq!(
            0x10000,
            core::mem::size_of::<CciPerformanceCounterRegisters>()
        );
        assert_eq!(
            0x100,
            core::mem::size_of::<CciSlaveInterfaceMonitorRegisters>()
        );
        assert_eq!(
            0x100,
            core::mem::size_of::<CciMasterInterfaceMonitorRegisters>()
        );
        assert_eq!(0xa0000, core::mem::size_of::<Cci5x0Registers>());
    }
}
