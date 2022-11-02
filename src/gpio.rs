//! General purpose input / output

use core::marker::PhantomData;
use cortex_m::interrupt::CriticalSection;
use crate::error::Error;
use crate::drivers::cpuss::interrupt::InterruptSource;
/// Extension trait to split a GPIO peripheral in independent pins and
/// registers.
pub trait GpioExt {
    /// The parts to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}

/// HSIOM GPIO mode (type state)
pub struct GpioPinMode;

/// High impedance drive mode (type state)
pub struct HighZ;

/// Strong output drive mode
pub struct Strong;

pub struct ResistivePullUp;
pub struct ResistivePullDown;
pub struct ResistivePullUpDown;
pub struct OpenDrainDrivesHigh;
pub struct OpenDrainDrivesLow;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

pub enum EdgeSelect {
    Disable = 0,
    Rising = 1,
    Falling = 2,
    Both = 3,
}
pub enum FilterSelect {
    Disable = 0,
    Rising = 1,
    Falling = 2,
    Both = 3,
}
pub enum PinLevel {
    Low = 0,
    High = 1,
}

// `i` -> port number
// `j` -> pin number
macro_rules! gpio {
    ([
     $($Pi_j:ident: ($pi_j:ident, $prti:ident, $inx:ident, $j:expr, $MODE:ty)),+
    ]) => {

        use core::convert::Infallible;

        use embedded_hal::digital::v2::{OutputPin, InputPin};
        use psoc6_pac::GPIO;

        /// GPIO parts
        pub struct Parts {
            $(
                /// Pin
                pub $pi_j: $Pi_j<$MODE>,
            )+
        }

        impl GpioExt for GPIO {
            type Parts = Parts;

            fn split(self) -> Parts {
                Parts {
                    $(
                        $pi_j: $Pi_j { _mode: PhantomData },
                    )+
                }
            }
        }

        $(
            /// Pin
            pub struct $Pi_j<MODE> {
                _mode: PhantomData<MODE>,
            }

            impl<MODE> $Pi_j<MODE> {
                pub fn into_pull_up_output(self, _cs: &CriticalSection) -> $Pi_j<Output<ResistivePullUp>> {
                    self.set_to_output();
                    self.set_drive_mode(2);
                    $Pi_j { _mode: PhantomData }
                }
                pub fn into_pull_down_output(self, _cs: &CriticalSection) -> $Pi_j<Output<ResistivePullDown>> {
                    self.set_to_output();
                    self.set_drive_mode(3);
                    $Pi_j { _mode: PhantomData }
                }
                 pub fn into_open_drain_high_output(self, _cs: &CriticalSection) -> $Pi_j<Output<OpenDrainDrivesHigh>> {
                    self.set_to_output();
                    self.set_drive_mode(4);
                    $Pi_j { _mode: PhantomData }
                }
                pub fn into_open_drain_low_output(self, _cs: &CriticalSection) -> $Pi_j<Output<OpenDrainDrivesLow>> {
                    self.set_to_output();
                    self.set_drive_mode(5);
                    $Pi_j { _mode: PhantomData }
                }
                /// Configures the pin to operate as a strong output pin
                pub fn into_strong_output(self, _cs: &CriticalSection) -> $Pi_j<Output<Strong>> {
                    self.set_to_output();
                    self.set_drive_mode(6);
                    $Pi_j { _mode: PhantomData }
                }

                pub fn into_pull_up_down_output(self, _cs: &CriticalSection) -> $Pi_j<Output<ResistivePullUpDown>> {
                    self.set_to_output();
                    self.set_drive_mode(7);
                    $Pi_j { _mode: PhantomData }
                }
                pub fn into_pull_up_input(self, _cs: &CriticalSection) -> $Pi_j<Input<ResistivePullUp>> {

                    self.set_to_input();
                    self.set_input_high_low(PinLevel::High);  //needed to activate pullup
                    self.set_drive_mode(2);
                    $Pi_j { _mode: PhantomData }
                }
                pub fn into_pull_down_input(self, _cs: &CriticalSection) -> $Pi_j<Input<ResistivePullDown>> {
                    self.set_to_input();
                    self.set_input_high_low(PinLevel::Low);  //needed to activate pulldown
                    self.set_drive_mode(3);
                    $Pi_j { _mode: PhantomData }
                }
                 pub fn into_open_drain_high_input(self, _cs: &CriticalSection) -> $Pi_j<Input<OpenDrainDrivesHigh>> {
                    self.set_to_input();
                    self.set_drive_mode(4);
                    $Pi_j { _mode: PhantomData }
                }
                pub fn into_open_drain_low_input(self, _cs: &CriticalSection) -> $Pi_j<Input<OpenDrainDrivesLow>> {
                    self.set_to_input();
                    self.set_drive_mode(5);
                    $Pi_j { _mode: PhantomData }
                }
                /// Configures the pin to operate as a strong output pin
                pub fn into_strong_input(self, _cs: &CriticalSection) -> $Pi_j<Input<Strong>> {
                    self.set_to_input();
                    self.set_drive_mode(6);
                    $Pi_j { _mode: PhantomData }
                }

                pub fn into_pull_up_down_input(self, _cs: &CriticalSection) -> $Pi_j<Input<ResistivePullUpDown>> {
                    self.set_to_input();
                    self.set_input_high_low(PinLevel::High); // needed to activate ,pullup.
                    self.set_drive_mode(7);
                    $Pi_j { _mode: PhantomData }
                }


                /// Set the drive mode for the pin
                #[inline(always)]
                fn set_drive_mode(&self, bits: u8) {
                    unsafe { (*GPIO::PTR).$prti.cfg.modify(|_, w| {
                        match $j {
                            0 => w.drive_mode0().bits(bits),
                            1 => w.drive_mode1().bits(bits),
                            2 => w.drive_mode2().bits(bits),
                            3 => w.drive_mode3().bits(bits),
                            4 => w.drive_mode4().bits(bits),
                            5 => w.drive_mode5().bits(bits),
                            6 => w.drive_mode6().bits(bits),
                            7 => w.drive_mode7().bits(bits),
                            _ => panic!(),
                        }
                    })}
                }
                #[inline(always)]
                fn set_to_input(&self) {
                    unsafe{(*GPIO::PTR).$prti.cfg.modify(|_, w| {
                        match $j {
                            0 => w.in_en0().set_bit(),
                            1 => w.in_en1().set_bit(),
                            2 => w.in_en2().set_bit(),
                            3 => w.in_en3().set_bit(),
                            4 => w.in_en4().set_bit(),
                            5 => w.in_en5().set_bit(),
                            6 => w.in_en6().set_bit(),
                            7 => w.in_en7().set_bit(),
                            _ => panic!(),
                        }
                    })}
                }
                #[inline(always)]
                fn set_to_output(&self) {
                    unsafe{(*GPIO::PTR).$prti.cfg.modify(|_, w| {
                        match $j {
                            0 => w.in_en0().clear_bit(),
                            1 => w.in_en1().clear_bit(),
                            2 => w.in_en2().clear_bit(),
                            3 => w.in_en3().clear_bit(),
                            4 => w.in_en4().clear_bit(),
                            5 => w.in_en5().clear_bit(),
                            6 => w.in_en6().clear_bit(),
                            7 => w.in_en7().clear_bit(),
                            _ => panic!(),
                        }
                    })}
                }
                #[inline(always)]
                fn set_input_high_low(&self, level: PinLevel){
                    match level{
                        PinLevel::Low =>  unsafe { (*GPIO::PTR).$prti.out_clr.write(|w| w.bits(1 << $j)) },
                        PinLevel::High => unsafe { (*GPIO::PTR).$prti.out_set.write(|w| w.bits(1 << $j)) },
                    }
                }
                ////////////////////////////
                // GPIO interrupts provides 16 interrupt lines(IRQn) that
                // can be triggered by an edge on a gpio pin. Each port
                // has one fixed IRQn, e.g. pin0_4 will trigger the
                // port0 IRQn, IOSS_INTERRUPTS_GPIO_0.
                // **configure_interrupt** 
                // 1. Configures the 50ns  glitch filter:
                //    - Only one pin can be filtered at anytime.
                //      Configuring this pin will stop filtering on
                //      any previously configured pins.
                //    - The filter can filter a rising, falling or both
                //      edges. Or can be disabled.
                // 2. Configures the interrupt trigger for rising, falling or both
                //    edges. The trigger can also be disabled.
                // 3. Configures the interrupt forwarding to the nvic.
                fn configure_interrupt(&self, filt_sel: FilterSelect, edge_sel: EdgeSelect){
                    //Filtering allows a 50ns glitch filter to be added to the input path.'
                    unsafe{
                    if (*GPIO::PTR).$prti.intr_cfg.read().flt_sel().bits() != $j {
                        match filt_sel {
                            //These options need to set flt_edg_sel while checking flt_sel set to pin number.
                            //Also set the interrupt for the filtered pin.
                            FilterSelect::Rising | FilterSelect::Falling | FilterSelect::Both => {
                                (*GPIO::PTR).$prti.intr_cfg.write(|w| w.flt_sel().bits($j as u8));
                                (*GPIO::PTR).$prti.intr_cfg.write(|w| w.flt_edge_sel().bits(filt_sel as u8));
                                (*GPIO::PTR).$prti.intr_mask.modify(|_, w| w.flt_edge().set_bit());
                            }
                            _=> { //Do nothing here. The pin is not being filtered and therefore
                                // won't need to be disabled.
                            }
                        }
                    }else{
                        //the filter has been configured for this pin so set the filters as required.
                        (*GPIO::PTR).$prti.intr_cfg.write(|w| w.flt_edge_sel().bits(filt_sel as u8));
                        (*GPIO::PTR).$prti.intr_mask.modify(|_, w| w.flt_edge().set_bit());
                    }
                    //modify the edge_sel type.
                    (*GPIO::PTR).$prti.intr_cfg.modify(|_,w|
                                               match $j{
                                                   0 => w.edge0_sel().bits(edge_sel as u8),
                                                   1 => w.edge1_sel().bits(edge_sel as u8),
                                                   2 => w.edge2_sel().bits(edge_sel as u8),
                                                   3 => w.edge3_sel().bits(edge_sel as u8),
                                                   4 => w.edge4_sel().bits(edge_sel as u8),
                                                   5 => w.edge5_sel().bits(edge_sel as u8),
                                                   6 => w.edge6_sel().bits(edge_sel as u8),
                                                   7 => w.edge7_sel().bits(edge_sel as u8),
                                                   _ => panic!(),
                                               }
                    );
                    //Set an interrupt to be
                    (*GPIO::PTR).$prti.intr_mask.modify(|_, w|  
                                                match $j{
                                                    0 => w.edge0().set_bit(),
                                                    1 => w.edge1().set_bit(),
                                                    2 => w.edge2().set_bit(),
                                                    3 => w.edge3().set_bit(),
                                                    4 => w.edge4().set_bit(),
                                                    5 => w.edge5().set_bit(),
                                                    6 => w.edge6().set_bit(),
                                                    7 => w.edge7().set_bit(),
                                                    _ => panic!(),
                                                }
                    );}
                }
                
                
                #[inline(always)]
                fn clear_interrupt(&self){
                    //psoc code reads the buffer
                    //then clears the interrupt
                    //then flushes to hardware with another read.
                    unsafe{
                        (*GPIO::PTR).$prti.intr_mask.read(); //seems like this will this be optimised out?
                        (*GPIO::PTR).$prti.intr_mask.modify(|_ ,w|
                                                match $j{
                                                    0 => w.edge0().set_bit(),
                                                    1 => w.edge1().set_bit(),
                                                    2 => w.edge2().set_bit(),
                                                    3 => w.edge3().set_bit(),
                                                    4 => w.edge4().set_bit(),
                                                    5 => w.edge5().set_bit(),
                                                    6 => w.edge6().set_bit(),
                                                    7 => w.edge7().set_bit(),
                                                    _ => panic!(),
                                                });
                        (*GPIO::PTR).$prti.intr_mask.read();//seems like this will this be optimised out?
                    }
                }
            }

            impl<MODE> OutputPin for $Pi_j<Output<MODE>> {
                type Error = Infallible;
                #[inline(always)]
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::PTR).$prti.out_set.write(|w| w.bits(1 << $j)) };
                    Ok(())
                }
                #[inline(always)]
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*GPIO::PTR).$prti.out_clr.write(|w| w.bits(1 << $j)) };
                    Ok(())
                }

            }
            impl<MODE> InputPin for $Pi_j<Input<MODE>>{
                type Error = Infallible;
                #[inline(always)]
                fn is_high(&self) -> Result<bool, Self::Error>{
                    Ok(unsafe{ (*GPIO::PTR).$prti.in_.read().$inx().bit_is_set()})
                }
                #[inline(always)]
                fn is_low(&self) -> Result<bool, Self::Error>{
                    Ok(unsafe{ (*GPIO::PTR).$prti.in_.read().$inx().bit_is_clear()})
                }
            }

        )+
    };

}

gpio!([
    P0_0: (p0_0, prt0, in0, 0, Input<HighZ>),
    P0_1: (p0_1, prt0, in1, 1, Input<HighZ>),
    P0_2: (p0_2, prt0, in2, 2, Input<HighZ>),
    P0_3: (p0_3, prt0, in3, 3, Input<HighZ>),
    P0_4: (p0_4, prt0, in4, 4, Input<HighZ>),
    P0_5: (p0_5, prt0, in5, 5, Input<HighZ>),

    P1_0: (p1_0, prt1, in0, 0, Input<HighZ>),
    P1_1: (p1_1, prt1, in1, 1, Input<HighZ>),
    P1_2: (p1_2, prt1, in2, 2, Input<HighZ>),
    P1_3: (p1_3, prt1, in3, 3, Input<HighZ>),
    P1_4: (p1_4, prt1, in4, 4, Input<HighZ>),
    P1_5: (p1_5, prt1, in5, 5, Input<HighZ>),

    P5_0: (p5_0, prt5, in0, 0, Input<HighZ>),
    P5_1: (p5_1, prt5, in1, 1, Input<HighZ>),
    P5_2: (p5_2, prt5, in2, 2, Input<HighZ>),
    P5_3: (p5_3, prt5, in3, 3, Input<HighZ>),
    P5_4: (p5_4, prt5, in4, 4, Input<HighZ>),
    P5_5: (p5_5, prt5, in5, 5, Input<HighZ>),
    P5_6: (p5_6, prt5, in6, 6, Input<HighZ>),
    P5_7: (p5_7, prt5, in7, 7, Input<HighZ>),

    P6_0: (p6_0, prt6, in0, 0, Input<HighZ>),
    P6_1: (p6_1, prt6, in1, 1, Input<HighZ>),
    P6_2: (p6_2, prt6, in2, 2, Input<HighZ>),
    P6_3: (p6_3, prt6, in3, 3, Input<HighZ>),
    P6_4: (p6_4, prt6, in4, 4, Input<HighZ>),
    P6_5: (p6_5, prt6, in5, 5, Input<HighZ>),
    P6_6: (p6_6, prt6, in6, 6, Input<HighZ>),
    P6_7: (p6_7, prt6, in7, 7, Input<HighZ>),

    P7_0: (p7_0, prt7, in0, 0, Input<HighZ>),
    P7_1: (p7_1, prt7, in1, 1, Input<HighZ>),
    P7_2: (p7_2, prt7, in2, 2, Input<HighZ>),
    P7_3: (p7_3, prt7, in3, 3, Input<HighZ>),
    P7_4: (p7_4, prt7, in4, 4, Input<HighZ>),
    P7_5: (p7_5, prt7, in5, 5, Input<HighZ>),
    P7_6: (p7_6, prt7, in6, 6, Input<HighZ>),
    P7_7: (p7_7, prt7, in7, 7, Input<HighZ>),

    P8_0: (p8_0, prt8, in0, 0, Input<HighZ>),
    P8_1: (p8_1, prt8, in1, 1, Input<HighZ>),
    P8_2: (p8_2, prt8, in2, 2, Input<HighZ>),
    P8_3: (p8_3, prt8, in3, 3, Input<HighZ>),
    P8_4: (p8_4, prt8, in4, 4, Input<HighZ>),
    P8_5: (p8_5, prt8, in5, 5, Input<HighZ>),
    P8_6: (p8_6, prt8, in6, 6, Input<HighZ>),
    P8_7: (p8_7, prt8, in7, 7, Input<HighZ>),

    P9_0: (p9_0, prt9, in0, 0, Input<HighZ>),
    P9_1: (p9_1, prt9, in1, 1, Input<HighZ>),
    P9_2: (p9_2, prt9, in2, 2, Input<HighZ>),
    P9_3: (p9_3, prt9, in3, 3, Input<HighZ>),
    P9_4: (p9_4, prt9, in4, 4, Input<HighZ>),
    P9_5: (p9_5, prt9, in5, 5, Input<HighZ>),
    P9_6: (p9_6, prt9, in6, 6, Input<HighZ>),
    P9_7: (p9_7, prt9, in7, 7, Input<HighZ>),

    P10_0: (p10_0, prt10, in0, 0, Input<HighZ>),
    P10_1: (p10_1, prt10, in1, 1, Input<HighZ>),
    P10_2: (p10_2, prt10, in2, 2, Input<HighZ>),
    P10_3: (p10_3, prt10, in3, 3, Input<HighZ>),
    P10_4: (p10_4, prt10, in4, 4, Input<HighZ>),
    P10_5: (p10_5, prt10, in5, 5, Input<HighZ>),
    P10_6: (p10_6, prt10, in6, 6, Input<HighZ>),
    P10_7: (p10_7, prt10, in7, 7, Input<HighZ>),

    P11_0: (p11_0, prt11, in0, 0, Input<HighZ>),
    P11_1: (p11_1, prt11, in1, 1, Input<HighZ>),
    P11_2: (p11_2, prt11, in2, 2, Input<HighZ>),
    P11_3: (p11_3, prt11, in3, 3, Input<HighZ>),
    P11_4: (p11_4, prt11, in4, 4, Input<HighZ>),
    P11_5: (p11_5, prt11, in5, 5, Input<HighZ>),
    P11_6: (p11_6, prt11, in6, 6, Input<HighZ>),
    P11_7: (p11_7, prt11, in7, 7, Input<HighZ>),

    P12_0: (p12_0, prt12, in0, 0, Input<HighZ>),
    P12_1: (p12_1, prt12, in1, 1, Input<HighZ>),
    P12_2: (p12_2, prt12, in2, 2, Input<HighZ>),
    P12_3: (p12_3, prt12, in3, 3, Input<HighZ>),
    P12_4: (p12_4, prt12, in4, 4, Input<HighZ>),
    P12_5: (p12_5, prt12, in5, 5, Input<HighZ>),
    P12_6: (p12_6, prt12, in6, 6, Input<HighZ>),
    P12_7: (p12_7, prt12, in7, 7, Input<HighZ>),

    P13_0: (p13_0, prt13, in0, 0, Input<HighZ>),
    P13_1: (p13_1, prt13, in1, 1, Input<HighZ>),
    P13_2: (p13_2, prt13, in2, 2, Input<HighZ>),
    P13_3: (p13_3, prt13, in3, 3, Input<HighZ>),
    P13_4: (p13_4, prt13, in4, 4, Input<HighZ>),
    P13_5: (p13_5, prt13, in5, 5, Input<HighZ>),
    P13_6: (p13_6, prt13, in6, 6, Input<HighZ>),
    P13_7: (p13_7, prt13, in7, 7, Input<HighZ>)
]);
