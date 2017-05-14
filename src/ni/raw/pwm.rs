use std::os::raw::{c_void, c_ushort, c_uchar};
use ni::raw::fpga::fpga_wrapper::RioStatusPointer;
use ni::raw::fpga::fpga_types;

//#[repr(c_void)]
struct tPWM;

pub struct PWMPointer(*mut tPWM);

extern {
	pub fn tPWM__new() -> PWMPointer;
	pub fn tPWM__delete(tPWM: PWMPointer);

	pub fn tPWM__create(status: RioStatusPointer) -> PWMPointer;

	pub fn tPWM__writeConfig_Period(value: c_ushort, status: RioStatusPointer, This: PWMPointer);
	pub fn tPWM__writeConfig_MinHigh(value: c_ushort, status: RioStatusPointer, This: PWMPointer);

	pub fn tPWM__readLoopTiming(status: RioStatusPointer, This: PWMPointer) -> c_ushort;

	pub fn tPWM__writeHdr(reg_index: c_uchar, value: c_ushort, status: RioStatusPointer, This: PWMPointer);
	pub fn tPWM__writeMXP(reg_index: c_uchar, value: c_ushort, status: RioStatusPointer, This: PWMPointer);

	pub fn tPWM__writePeriodScaleHdr(bitfield_index: c_uchar, value: c_ushort, status: RioStatusPointer, This: PWMPointer);
	pub fn tPWM__writePeriodScaleMXP(bitfield_index: c_uchar, value: c_ushort, status: RioStatusPointer, This: PWMPointer);
}

pub fn writeConfig_Period(value: c_ushort, This: PWMPointer) -> RioStatusPointer{
	let status: RioStatusPointer;
	unsafe{tPWM__writeConfig_Period(value, status, This);}
	return status;

}
pub fn writeConfig_MinHigh(value: c_ushort, This: PWMPointer) -> RioStatusPointer{
	let status: RioStatusPointer;
	unsafe{tPWM__writeConfig_MinHigh(value, status, This);}
	return status;

}
