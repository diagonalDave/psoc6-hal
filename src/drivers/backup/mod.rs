//! backup.rs implements a driver for the backup functionality. The hardware backup
//! functionality uses a separate power domain provided by a power supply
//! to keep the psoc running if Vddd has issues.
//! Chapter 19 of the trm pp 213.
use crate::pac::BACKUP;

pub struct Backup {
    backup: BACKUP,
}

impl Backup {
    fn new(backup: BACKUP) -> Self {
        Self { backup }
    }
    pub fn from(backup: BACKUP) -> Backup {
        Self::new(backup)
    }
    /// reset() is the only means of triggering a reset in the Backup power domain
    /// without removing the backup supply entirely.
    /// The Backup domain should be reset during start up if a reset was
    /// caused by a power related cause POR/BOD or WDT/XRES
    pub fn reset(&self) -> () {
        self.backup.reset.modify(|_, w| w.reset().set_bit());
        //check is running again
        while self.backup.reset.read().bits() != 0 {}
    }
    /// is_running returns true when a reset has been completed.
    pub fn not_running(&self) -> bool {
        self.backup.reset.read().reset().bit_is_set()
    }
    /// configure_backup_source is a minimal implementation to enable
    /// start up.
    /// TODO: needs full implementation.
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn configure_backup_clock_source(&self) -> () {
        self.backup.ctl.modify(|_, w| w.clk_sel().altbak());
    }
    /// enable_pmic unlocks the backup pmic then enables it.
    /// This function enables the PMIC_EN pin. Thereby enabling
    /// a signal to the external power supply.
    /// Implemented as described in trm pp 635-636
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn enable_pmic(&self) -> () {
        //Safety:
        self.backup
            .pmic_ctl
            .modify(|_, w| unsafe { w.unlock().bits(0x3a) });
        self.backup
            .pmic_ctl
            .modify(|_, w| w.pmic_en_outen().set_bit());
    }
    /// disable_pmic unlocks the backup pmic then disables it.
    /// This function disables the PMIC_EN pin and switches it
    /// to Hi-z state so it can be used for GPIO.
    /// Implemented as described in trm pp 635-636
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn disable_pmic(&self) -> () {
        //Safety:
        self.backup
            .pmic_ctl
            .modify(|_, w| unsafe { w.unlock().bits(0x3a) });
        self.backup
            .pmic_ctl
            .modify(|_, w| w.pmic_en_outen().clear_bit());
    }
}
