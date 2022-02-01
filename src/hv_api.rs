use crate::hv_defs::*;

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
                      GuestAddress:     WHvGuestPhysicialAddress,
                      SizeInBytes:      u64,
                      Flags:            WHvMapGpaRangeFlags) -> HResult;

    fn WHvQueryGpaRangeDirtyBitmap(Partition: PHandle,
                                   GuestAddress: WHvGuestPhysicialAddress,
                                   RangeSizeInBytes: u64,
                                   Bitmap: *mut u64,
                                   BitmapSizeInBytes: u32) -> HResult;

    fn WHvRequestInterrupt(Partition: PHandle,
                           Interrupt: *const usize,                 // pointer to WHvInterruptControl structure
                           InterruptControlSize: u32) -> HResult;

    fn WHvResumePartitionTime(Partition: PHandle) -> HResult;

    
    

}

