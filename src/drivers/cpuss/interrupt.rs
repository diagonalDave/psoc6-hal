//! drivers/cpuss/interrupt.rs provides the driver for interrupt configuration.
//! Psoc6 implements 147 interrupt sources.The CM4 core maps an IRQ
//! line for each interrupt source.
//! The CM0+ interrupts are mapped to an IRQ line via a multiplexer
//! enabling one or more of the 147 interrupt sources to be mapped to any
//! of the CM0+ IRQ lines. This means that if more than one interrupt
//! source is present on an IRQ line, the IRQ line must be queried to determine
//! which source interrupted.

use crate::drivers::cpuss::Cpuss;

#[cfg(not(armv7em))]
use crate::pac::Interrupt;

impl Cpuss {
    #[allow(non_snake_case)]
    #[cfg(not(armv7em))]
    pub fn configure_interrupt_mux(&self, intr_source: InterruptSource, irqn: Interrupt) -> () {
        // Find correct int_ctl register. There are 4 irqn per ctl register.
        match irqn {
            Interrupt::NVIC_MUX0_IRQn => self
                .cpu_sys
                .cm0_int_ctl0
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX1_IRQn => self
                .cpu_sys
                .cm0_int_ctl0
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX2_IRQn => self
                .cpu_sys
                .cm0_int_ctl0
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX3_IRQn => self
                .cpu_sys
                .cm0_int_ctl0
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX4_IRQn => self
                .cpu_sys
                .cm0_int_ctl1
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX5_IRQn => self
                .cpu_sys
                .cm0_int_ctl1
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX6_IRQn => self
                .cpu_sys
                .cm0_int_ctl1
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX7_IRQn => self
                .cpu_sys
                .cm0_int_ctl1
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX8_IRQn => self
                .cpu_sys
                .cm0_int_ctl2
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX9_IRQn => self
                .cpu_sys
                .cm0_int_ctl2
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX10_IRQn => self
                .cpu_sys
                .cm0_int_ctl2
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX11_IRQn => self
                .cpu_sys
                .cm0_int_ctl2
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX12_IRQn => self
                .cpu_sys
                .cm0_int_ctl3
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX13_IRQn => self
                .cpu_sys
                .cm0_int_ctl3
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX14_IRQn => self
                .cpu_sys
                .cm0_int_ctl3
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX15_IRQn => self
                .cpu_sys
                .cm0_int_ctl3
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX16_IRQn => self
                .cpu_sys
                .cm0_int_ctl4
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX17_IRQn => self
                .cpu_sys
                .cm0_int_ctl4
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX18_IRQn => self
                .cpu_sys
                .cm0_int_ctl4
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX19_IRQn => self
                .cpu_sys
                .cm0_int_ctl4
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX20_IRQn => self
                .cpu_sys
                .cm0_int_ctl5
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX21_IRQn => self
                .cpu_sys
                .cm0_int_ctl5
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX22_IRQn => self
                .cpu_sys
                .cm0_int_ctl5
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX23_IRQn => self
                .cpu_sys
                .cm0_int_ctl5
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX24_IRQn => self
                .cpu_sys
                .cm0_int_ctl6
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX25_IRQn => self
                .cpu_sys
                .cm0_int_ctl6
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX26_IRQn => self
                .cpu_sys
                .cm0_int_ctl6
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX27_IRQn => self
                .cpu_sys
                .cm0_int_ctl6
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX28_IRQn => self
                .cpu_sys
                .cm0_int_ctl7
                .modify(|_, w| unsafe { w.mux0_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX29_IRQn => self
                .cpu_sys
                .cm0_int_ctl7
                .modify(|_, w| unsafe { w.mux1_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX30_IRQn => self
                .cpu_sys
                .cm0_int_ctl7
                .modify(|_, w| unsafe { w.mux2_sel().bits(intr_source as u8) }),
            Interrupt::NVIC_MUX31_IRQn => self
                .cpu_sys
                .cm0_int_ctl7
                .modify(|_, w| unsafe { w.mux3_sel().bits(intr_source as u8) }),
        }
    }
}

/// PsocInterrupts represents all the available interrupts on a Psoc6 device.
/// It is a copy of the pac::Interrupts (for the CM4).
/// It is here to sidestep the non appearance of all the Interrupt types enumerations
/// when compiling for cm0.
/// TODO: remove this duplication by developing a better feature gate.
//#[cfg(armv6m)]
#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum InterruptSource {
    #[doc = "0 - GPIO Port Interrupt #0"]
    IOSS_INTERRUPTS_GPIO_0 = 0,
    #[doc = "1 - GPIO Port Interrupt #1"]
    IOSS_INTERRUPTS_GPIO_1 = 1,
    #[doc = "2 - GPIO Port Interrupt #2"]
    IOSS_INTERRUPTS_GPIO_2 = 2,
    #[doc = "3 - GPIO Port Interrupt #3"]
    IOSS_INTERRUPTS_GPIO_3 = 3,
    #[doc = "4 - GPIO Port Interrupt #4"]
    IOSS_INTERRUPTS_GPIO_4 = 4,
    #[doc = "5 - GPIO Port Interrupt #5"]
    IOSS_INTERRUPTS_GPIO_5 = 5,
    #[doc = "6 - GPIO Port Interrupt #6"]
    IOSS_INTERRUPTS_GPIO_6 = 6,
    #[doc = "7 - GPIO Port Interrupt #7"]
    IOSS_INTERRUPTS_GPIO_7 = 7,
    #[doc = "8 - GPIO Port Interrupt #8"]
    IOSS_INTERRUPTS_GPIO_8 = 8,
    #[doc = "9 - GPIO Port Interrupt #9"]
    IOSS_INTERRUPTS_GPIO_9 = 9,
    #[doc = "10 - GPIO Port Interrupt #10"]
    IOSS_INTERRUPTS_GPIO_10 = 10,
    #[doc = "11 - GPIO Port Interrupt #11"]
    IOSS_INTERRUPTS_GPIO_11 = 11,
    #[doc = "12 - GPIO Port Interrupt #12"]
    IOSS_INTERRUPTS_GPIO_12 = 12,
    #[doc = "13 - GPIO Port Interrupt #13"]
    IOSS_INTERRUPTS_GPIO_13 = 13,
    #[doc = "14 - GPIO Port Interrupt #14"]
    IOSS_INTERRUPTS_GPIO_14 = 14,
    #[doc = "15 - GPIO All Ports"]
    IOSS_INTERRUPT_GPIO = 15,
    #[doc = "16 - GPIO Supply Detect Interrupt"]
    IOSS_INTERRUPT_VDD = 16,
    #[doc = "17 - Low Power Comparator Interrupt"]
    LPCOMP_INTERRUPT = 17,
    #[doc = "18 - Serial Communication Block #8 (DeepSleep capable)"]
    SCB_8_INTERRUPT = 18,
    #[doc = "19 - Multi Counter Watchdog Timer interrupt"]
    SRSS_INTERRUPT_MCWDT_0 = 19,
    #[doc = "20 - Multi Counter Watchdog Timer interrupt"]
    SRSS_INTERRUPT_MCWDT_1 = 20,
    #[doc = "21 - Backup domain interrupt"]
    SRSS_INTERRUPT_BACKUP = 21,
    #[doc = "22 - Other combined Interrupts for SRSS (LVD, WDT, CLKCAL)"]
    SRSS_INTERRUPT = 22,
    #[doc = "23 - CTBm Interrupt (all CTBms)"]
    PASS_INTERRUPT_CTBS = 23,
    #[doc = "24 - Bluetooth Radio interrupt"]
    BLESS_INTERRUPT = 24,
    #[doc = "25 - CPUSS Inter Process Communication Interrupt #0"]
    CPUSS_INTERRUPTS_IPC_0 = 25,
    #[doc = "26 - CPUSS Inter Process Communication Interrupt #1"]
    CPUSS_INTERRUPTS_IPC_1 = 26,
    #[doc = "27 - CPUSS Inter Process Communication Interrupt #2"]
    CPUSS_INTERRUPTS_IPC_2 = 27,
    #[doc = "28 - CPUSS Inter Process Communication Interrupt #3"]
    CPUSS_INTERRUPTS_IPC_3 = 28,
    #[doc = "29 - CPUSS Inter Process Communication Interrupt #4"]
    CPUSS_INTERRUPTS_IPC_4 = 29,
    #[doc = "30 - CPUSS Inter Process Communication Interrupt #5"]
    CPUSS_INTERRUPTS_IPC_5 = 30,
    #[doc = "31 - CPUSS Inter Process Communication Interrupt #6"]
    CPUSS_INTERRUPTS_IPC_6 = 31,
    #[doc = "32 - CPUSS Inter Process Communication Interrupt #7"]
    CPUSS_INTERRUPTS_IPC_7 = 32,
    #[doc = "33 - CPUSS Inter Process Communication Interrupt #8"]
    CPUSS_INTERRUPTS_IPC_8 = 33,
    #[doc = "34 - CPUSS Inter Process Communication Interrupt #9"]
    CPUSS_INTERRUPTS_IPC_9 = 34,
    #[doc = "35 - CPUSS Inter Process Communication Interrupt #10"]
    CPUSS_INTERRUPTS_IPC_10 = 35,
    #[doc = "36 - CPUSS Inter Process Communication Interrupt #11"]
    CPUSS_INTERRUPTS_IPC_11 = 36,
    #[doc = "37 - CPUSS Inter Process Communication Interrupt #12"]
    CPUSS_INTERRUPTS_IPC_12 = 37,
    #[doc = "38 - CPUSS Inter Process Communication Interrupt #13"]
    CPUSS_INTERRUPTS_IPC_13 = 38,
    #[doc = "39 - CPUSS Inter Process Communication Interrupt #14"]
    CPUSS_INTERRUPTS_IPC_14 = 39,
    #[doc = "40 - CPUSS Inter Process Communication Interrupt #15"]
    CPUSS_INTERRUPTS_IPC_15 = 40,
    #[doc = "41 - Serial Communication Block #0"]
    SCB_0_INTERRUPT = 41,
    #[doc = "42 - Serial Communication Block #1"]
    SCB_1_INTERRUPT = 42,
    #[doc = "43 - Serial Communication Block #2"]
    SCB_2_INTERRUPT = 43,
    #[doc = "44 - Serial Communication Block #3"]
    SCB_3_INTERRUPT = 44,
    #[doc = "45 - Serial Communication Block #4"]
    SCB_4_INTERRUPT = 45,
    #[doc = "46 - Serial Communication Block #5"]
    SCB_5_INTERRUPT = 46,
    #[doc = "47 - Serial Communication Block #6"]
    SCB_6_INTERRUPT = 47,
    #[doc = "48 - Serial Communication Block #7"]
    SCB_7_INTERRUPT = 48,
    #[doc = "49 - CSD (Capsense) interrupt"]
    CSD_INTERRUPT = 49,
    #[doc = "50 - CPUSS DataWire #0, Channel #0"]
    CPUSS_INTERRUPTS_DW0_0 = 50,
    #[doc = "51 - CPUSS DataWire #0, Channel #1"]
    CPUSS_INTERRUPTS_DW0_1 = 51,
    #[doc = "52 - CPUSS DataWire #0, Channel #2"]
    CPUSS_INTERRUPTS_DW0_2 = 52,
    #[doc = "53 - CPUSS DataWire #0, Channel #3"]
    CPUSS_INTERRUPTS_DW0_3 = 53,
    #[doc = "54 - CPUSS DataWire #0, Channel #4"]
    CPUSS_INTERRUPTS_DW0_4 = 54,
    #[doc = "55 - CPUSS DataWire #0, Channel #5"]
    CPUSS_INTERRUPTS_DW0_5 = 55,
    #[doc = "56 - CPUSS DataWire #0, Channel #6"]
    CPUSS_INTERRUPTS_DW0_6 = 56,
    #[doc = "57 - CPUSS DataWire #0, Channel #7"]
    CPUSS_INTERRUPTS_DW0_7 = 57,
    #[doc = "58 - CPUSS DataWire #0, Channel #8"]
    CPUSS_INTERRUPTS_DW0_8 = 58,
    #[doc = "59 - CPUSS DataWire #0, Channel #9"]
    CPUSS_INTERRUPTS_DW0_9 = 59,
    #[doc = "60 - CPUSS DataWire #0, Channel #10"]
    CPUSS_INTERRUPTS_DW0_10 = 60,
    #[doc = "61 - CPUSS DataWire #0, Channel #11"]
    CPUSS_INTERRUPTS_DW0_11 = 61,
    #[doc = "62 - CPUSS DataWire #0, Channel #12"]
    CPUSS_INTERRUPTS_DW0_12 = 62,
    #[doc = "63 - CPUSS DataWire #0, Channel #13"]
    CPUSS_INTERRUPTS_DW0_13 = 63,
    #[doc = "64 - CPUSS DataWire #0, Channel #14"]
    CPUSS_INTERRUPTS_DW0_14 = 64,
    #[doc = "65 - CPUSS DataWire #0, Channel #15"]
    CPUSS_INTERRUPTS_DW0_15 = 65,
    #[doc = "66 - CPUSS DataWire #1, Channel #0"]
    CPUSS_INTERRUPTS_DW1_0 = 66,
    #[doc = "67 - CPUSS DataWire #1, Channel #1"]
    CPUSS_INTERRUPTS_DW1_1 = 67,
    #[doc = "68 - CPUSS DataWire #1, Channel #2"]
    CPUSS_INTERRUPTS_DW1_2 = 68,
    #[doc = "69 - CPUSS DataWire #1, Channel #3"]
    CPUSS_INTERRUPTS_DW1_3 = 69,
    #[doc = "70 - CPUSS DataWire #1, Channel #4"]
    CPUSS_INTERRUPTS_DW1_4 = 70,
    #[doc = "71 - CPUSS DataWire #1, Channel #5"]
    CPUSS_INTERRUPTS_DW1_5 = 71,
    #[doc = "72 - CPUSS DataWire #1, Channel #6"]
    CPUSS_INTERRUPTS_DW1_6 = 72,
    #[doc = "73 - CPUSS DataWire #1, Channel #7"]
    CPUSS_INTERRUPTS_DW1_7 = 73,
    #[doc = "74 - CPUSS DataWire #1, Channel #8"]
    CPUSS_INTERRUPTS_DW1_8 = 74,
    #[doc = "75 - CPUSS DataWire #1, Channel #9"]
    CPUSS_INTERRUPTS_DW1_9 = 75,
    #[doc = "76 - CPUSS DataWire #1, Channel #10"]
    CPUSS_INTERRUPTS_DW1_10 = 76,
    #[doc = "77 - CPUSS DataWire #1, Channel #11"]
    CPUSS_INTERRUPTS_DW1_11 = 77,
    #[doc = "78 - CPUSS DataWire #1, Channel #12"]
    CPUSS_INTERRUPTS_DW1_12 = 78,
    #[doc = "79 - CPUSS DataWire #1, Channel #13"]
    CPUSS_INTERRUPTS_DW1_13 = 79,
    #[doc = "80 - CPUSS DataWire #1, Channel #14"]
    CPUSS_INTERRUPTS_DW1_14 = 80,
    #[doc = "81 - CPUSS DataWire #1, Channel #15"]
    CPUSS_INTERRUPTS_DW1_15 = 81,
    #[doc = "82 - CPUSS Fault Structure Interrupt #0"]
    CPUSS_INTERRUPTS_FAULT_0 = 82,
    #[doc = "83 - CPUSS Fault Structure Interrupt #1"]
    CPUSS_INTERRUPTS_FAULT_1 = 83,
    #[doc = "84 - CRYPTO Accelerator Interrupt"]
    CPUSS_INTERRUPT_CRYPTO = 84,
    #[doc = "85 - FLASH Macro Interrupt"]
    CPUSS_INTERRUPT_FM = 85,
    #[doc = "86 - CM0+ CTI #0"]
    CPUSS_INTERRUPTS_CM0_CTI_0 = 86,
    #[doc = "87 - CM0+ CTI #1"]
    CPUSS_INTERRUPTS_CM0_CTI_1 = 87,
    #[doc = "88 - CM4 CTI #0"]
    CPUSS_INTERRUPTS_CM4_CTI_0 = 88,
    #[doc = "89 - CM4 CTI #1"]
    CPUSS_INTERRUPTS_CM4_CTI_1 = 89,
    #[doc = "90 - TCPWM #0, Counter #0"]
    TCPWM_0_INTERRUPTS_0 = 90,
    #[doc = "91 - TCPWM #0, Counter #1"]
    TCPWM_0_INTERRUPTS_1 = 91,
    #[doc = "92 - TCPWM #0, Counter #2"]
    TCPWM_0_INTERRUPTS_2 = 92,
    #[doc = "93 - TCPWM #0, Counter #3"]
    TCPWM_0_INTERRUPTS_3 = 93,
    #[doc = "94 - TCPWM #0, Counter #4"]
    TCPWM_0_INTERRUPTS_4 = 94,
    #[doc = "95 - TCPWM #0, Counter #5"]
    TCPWM_0_INTERRUPTS_5 = 95,
    #[doc = "96 - TCPWM #0, Counter #6"]
    TCPWM_0_INTERRUPTS_6 = 96,
    #[doc = "97 - TCPWM #0, Counter #7"]
    TCPWM_0_INTERRUPTS_7 = 97,
    #[doc = "98 - TCPWM #1, Counter #0"]
    TCPWM_1_INTERRUPTS_0 = 98,
    #[doc = "99 - TCPWM #1, Counter #1"]
    TCPWM_1_INTERRUPTS_1 = 99,
    #[doc = "100 - TCPWM #1, Counter #2"]
    TCPWM_1_INTERRUPTS_2 = 100,
    #[doc = "101 - TCPWM #1, Counter #3"]
    TCPWM_1_INTERRUPTS_3 = 101,
    #[doc = "102 - TCPWM #1, Counter #4"]
    TCPWM_1_INTERRUPTS_4 = 102,
    #[doc = "103 - TCPWM #1, Counter #5"]
    TCPWM_1_INTERRUPTS_5 = 103,
    #[doc = "104 - TCPWM #1, Counter #6"]
    TCPWM_1_INTERRUPTS_6 = 104,
    #[doc = "105 - TCPWM #1, Counter #7"]
    TCPWM_1_INTERRUPTS_7 = 105,
    #[doc = "106 - TCPWM #1, Counter #8"]
    TCPWM_1_INTERRUPTS_8 = 106,
    #[doc = "107 - TCPWM #1, Counter #9"]
    TCPWM_1_INTERRUPTS_9 = 107,
    #[doc = "108 - TCPWM #1, Counter #10"]
    TCPWM_1_INTERRUPTS_10 = 108,
    #[doc = "109 - TCPWM #1, Counter #11"]
    TCPWM_1_INTERRUPTS_11 = 109,
    #[doc = "110 - TCPWM #1, Counter #12"]
    TCPWM_1_INTERRUPTS_12 = 110,
    #[doc = "111 - TCPWM #1, Counter #13"]
    TCPWM_1_INTERRUPTS_13 = 111,
    #[doc = "112 - TCPWM #1, Counter #14"]
    TCPWM_1_INTERRUPTS_14 = 112,
    #[doc = "113 - TCPWM #1, Counter #15"]
    TCPWM_1_INTERRUPTS_15 = 113,
    #[doc = "114 - TCPWM #1, Counter #16"]
    TCPWM_1_INTERRUPTS_16 = 114,
    #[doc = "115 - TCPWM #1, Counter #17"]
    TCPWM_1_INTERRUPTS_17 = 115,
    #[doc = "116 - TCPWM #1, Counter #18"]
    TCPWM_1_INTERRUPTS_18 = 116,
    #[doc = "117 - TCPWM #1, Counter #19"]
    TCPWM_1_INTERRUPTS_19 = 117,
    #[doc = "118 - TCPWM #1, Counter #20"]
    TCPWM_1_INTERRUPTS_20 = 118,
    #[doc = "119 - TCPWM #1, Counter #21"]
    TCPWM_1_INTERRUPTS_21 = 119,
    #[doc = "120 - TCPWM #1, Counter #22"]
    TCPWM_1_INTERRUPTS_22 = 120,
    #[doc = "121 - TCPWM #1, Counter #23"]
    TCPWM_1_INTERRUPTS_23 = 121,
    #[doc = "122 - UDB Interrupt #0"]
    UDB_INTERRUPTS_0 = 122,
    #[doc = "123 - UDB Interrupt #1"]
    UDB_INTERRUPTS_1 = 123,
    #[doc = "124 - UDB Interrupt #2"]
    UDB_INTERRUPTS_2 = 124,
    #[doc = "125 - UDB Interrupt #3"]
    UDB_INTERRUPTS_3 = 125,
    #[doc = "126 - UDB Interrupt #4"]
    UDB_INTERRUPTS_4 = 126,
    #[doc = "127 - UDB Interrupt #5"]
    UDB_INTERRUPTS_5 = 127,
    #[doc = "128 - UDB Interrupt #6"]
    UDB_INTERRUPTS_6 = 128,
    #[doc = "129 - UDB Interrupt #7"]
    UDB_INTERRUPTS_7 = 129,
    #[doc = "130 - UDB Interrupt #8"]
    UDB_INTERRUPTS_8 = 130,
    #[doc = "131 - UDB Interrupt #9"]
    UDB_INTERRUPTS_9 = 131,
    #[doc = "132 - UDB Interrupt #10"]
    UDB_INTERRUPTS_10 = 132,
    #[doc = "133 - UDB Interrupt #11"]
    UDB_INTERRUPTS_11 = 133,
    #[doc = "134 - UDB Interrupt #12"]
    UDB_INTERRUPTS_12 = 134,
    #[doc = "135 - UDB Interrupt #13"]
    UDB_INTERRUPTS_13 = 135,
    #[doc = "136 - UDB Interrupt #14"]
    UDB_INTERRUPTS_14 = 136,
    #[doc = "137 - UDB Interrupt #15"]
    UDB_INTERRUPTS_15 = 137,
    #[doc = "138 - SAR ADC interrupt"]
    PASS_INTERRUPT_SAR = 138,
    #[doc = "139 - I2S Audio interrupt"]
    AUDIOSS_INTERRUPT_I2S = 139,
    #[doc = "140 - PDM/PCM Audio interrupt"]
    AUDIOSS_INTERRUPT_PDM = 140,
    #[doc = "141 - Energy Profiler interrupt"]
    PROFILE_INTERRUPT = 141,
    #[doc = "142 - Serial Memory Interface interrupt"]
    SMIF_INTERRUPT = 142,
    #[doc = "143 - USB Interrupt"]
    USB_INTERRUPT_HI = 143,
    #[doc = "144 - USB Interrupt"]
    USB_INTERRUPT_MED = 144,
    #[doc = "145 - USB Interrupt"]
    USB_INTERRUPT_LO = 145,
    #[doc = "146 - Consolidated interrrupt for all DACs"]
    PASS_INTERRUPT_DACS = 146,
}

unsafe impl cortex_m::interrupt::InterruptNumber for InterruptSource {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
