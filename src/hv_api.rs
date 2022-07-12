use crate::hv_defs::*;
use crate::win_api::*;

// TODO: verify the types of the ...BufferSizeInBytes variables

#[link(name="WinHvPlatform")]
extern "system"
{
    fn WHvCancelRunVirtualProcessor(Partition: PHandle,
                                    VpIndex: u32,
                                    Flags: u32) -> HResult;
    
    fn WHvCreatePartition(Partition: *mut PHandle) -> HResult;
    
    fn WHvCreateVirtualProcessor(Partition: PHandle, 
                                 VpIndex: u32,
                                 Flags: u32) -> HResult;
    
    fn WHvDeletePartition(Partition: PHandle) -> HResult;
    
    fn WHvDeleteVirtualProcessor(Partition: PHandle,
                                 VpIndex: u32) -> HResult;
    
    fn WHvGetCapability(CapabilityCode: WHvCapabilityCode,
			CapabilityBuffer: *mut WHvCapability,
			CapabilityBufferSizeInBytes: u32,
			WrittenSizeInBytes: *mut u32) -> HResult;
    
    fn WHvGetPartitionCounters(Partition: PHandle,
			       CounterSet: WHvPartitionCounterSet,
			       Buffer: *mut usize,
			       BufferSizeInBytes: u32,
			       BytesWritten: *mut u32) -> HResult;
    
    fn WHvGetPartitionProperty(Partition: PHandle,
			       PropertyCode: WHvPartitionPropertyCode,
			       PropertyBuffer: *mut usize,
			       PropertyBufferSizeInBytes: u32,
			       WrittenSizeInBytes: *mut u32) -> HResult;
    
    fn WHvGetVirtualProcessorCounters(Partition: PHandle,
				      VpIndex: u32,
				      CounterSet: WHvProcessorCounterSet,
				      Buffer: *mut usize,
				      BufferSizeInBytes: u32,
				      BytesWritten: *mut u32) -> HResult;
    
    fn WHvGetVirtualProcessorInterruptControllerState(Partition: PHandle,
				                      VpIndex: u32,	
						      State: *mut usize,
						      StateSize: u32,
						      WrittenSize: *mut u32) -> HResult;

    fn WHvGetVirtualProcessorRegisters(Partition: PHandle,
				       VpIndex: u32,
				       RegisterNames: *const WHvRegisterName,
				       RegisterCount: u32,
				       RegisterValues: *mut WHvRegisterValue) -> HResult;

    fn WHvGetVirtualProcessorXsaveState(Partition: PHandle,
                                        VpIndex: u32,
                                        Buffer: *mut usize,
                                        BufferSizeInBytes: u32,
                                        BytesWritten: *mut u32);

    fn WHvMapGpaRange(Partition: PHandle,
                      SourceAddress:    *const usize,
                      GuestAddress:     WHvGuestPhysicalAddress,
                      SizeInBytes:      u64,
                      Flags:            WHvMapGpaRangeFlags) -> HResult;

    fn WHvQueryGpaRangeDirtyBitmap(Partition: PHandle,
                                   GuestAddress: WHvGuestPhysicalAddress,
                                   RangeSizeInBytes: u64,
                                   Bitmap: *mut u64,
                                   BitmapSizeInBytes: u32) -> HResult;

    fn WHvRequestInterrupt(Partition: PHandle,
                           Interrupt: *const usize,                 // pointer to WHvInterruptControl structure
                           InterruptControlSize: u32) -> HResult;

    fn WHvResumePartitionTime(Partition: PHandle) -> HResult;

    fn WHvSetPartitionProperty(Partition: PHandle,
                               PropertyCode: WHvPartitionPropertyCode,
                               PropertyBuffer: *mut WHvPartitionProperty,
                               PropertyBufferSizeInBytes: usize) -> HResult;
   
    fn WHvSetupPartition(Partition: PHandle) -> HResult;

    fn WHvSetVirtualProcessorInterruptControllerState(Partition: PHandle,
                                                      VpIndex: u32,
                                                      State: *mut usize,
                                                      StateSize: u32) -> HResult;

    fn WHvSetVirtualProcessorRegisters(Partition: PHandle,
                                       VpIndex: u32,
                                       RegisterNames: *const usize,
                                       RegisterCount: u32,
                                       RegisterValues: *const usize) -> HResult;

    fn WHvSetVirtualProcessorXsaveState(Partition: PHandle,
                                        VpIndex: u32,
                                        Buffer: *const usize,
                                        BufferSizeInBytes: u32) -> HResult;
    
    fn WHvSuspendPartitionTime(Partition: PHandle) -> HResult;

    fn WHvTranslateGva(Partition: PHandle,
                       VpIndex: u32,
                       Gva: WHvGuestVirtualAddress,
                       TranslateFlags: WHvTranslateGVAFlags,
                       TranslationResult: *mut WHvTranslateGVAResult,
                       Gpa: *mut WHvGuestPhysicalAddress) -> HResult;
    
    fn WHvUnmapGpaRange(Partition: PHandle,
                        GuestAddress: WHvGuestPhysicalAddress,
                        SizeInBytes: u64) -> HResult;

}

pub fn is_hypervisor_present() -> bool
{
    unsafe
    {
        let buffer_ptr: *mut WHvCapability = Box::into_raw(Box::new(WHvCapability::default())) as *mut WHvCapability;
        let buffer_size: u32 = std::mem::size_of::<WHvCapability>() as u32;
        let wrt_ptr: *mut u32 = Box::into_raw(Box::new(0)) as *mut u32;

        let res = WHvGetCapability(WHvCapabilityCode::HypervisorPresent, 
                                   buffer_ptr, 
                                   buffer_size, 
                                   wrt_ptr);
       
        (*buffer_ptr).HypervisorPresent
    }
}

pub fn create_partition() -> PHandle
{
    unsafe
    {
        let phandle: *mut PHandle = Box::into_raw(Box::new(0usize)) as *mut PHandle;
        let res = WHvCreatePartition(phandle);

        *phandle
    }
}

pub fn set_partition_property(partition: PHandle, property_code: WHvPartitionPropertyCode, property_buffer: *mut WHvPartitionProperty) -> HResult
{
    unsafe
    {
        WHvSetPartitionProperty(partition, property_code, property_buffer, std::mem::size_of::<WHvPartitionProperty>())
    }
}
