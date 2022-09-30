//! clocks.rs implements high level clock methods

use crate::psoc::Psoc;

pub struct DerivedClock{}
pub struct RealTimeClock{}

impl DerivedClock{
    pub fn new(freq:u32)-> DerivedClock{
        let _= freq;
        DerivedClock{}
    }
}
impl Psoc{
    pub fn start_system_clocks(&self){
        //config & start root clocks and paths
        //config &start fll
        //adjust wait states for clock speed
        
    }
    pub fn create_clock(&self, freq:u32)->DerivedClock{
        //config clock and return.
        let _ = freq;
        DerivedClock{}
    }
    pub fn create_cm0_clock(&self){todo!()}
    pub fn create_cm4_clock(&self){todo!()}

}
