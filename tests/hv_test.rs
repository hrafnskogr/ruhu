use ruhu::hv_defs::*;
use std::mem;


#[test]
fn defs_types_size()
{
    assert_eq!(mem::size_of::<WHvCapabilityFeatures>(),                         mem::size_of::<u64>(),  "1");
    assert_eq!(mem::size_of::<WHvExtendedVmExits>(),                            mem::size_of::<u64>(),  "2");
    assert_eq!(mem::size_of::<WHvProcessorFeatures>(),                          mem::size_of::<u64>(),  "3");
    assert_eq!(mem::size_of::<WHvProcessorXsaveFeatures>(),                     mem::size_of::<u64>(),  "4");
    assert_eq!(mem::size_of::<WHvCapability>(),                                 mem::size_of::<u64>(),  "5");
    assert_eq!(mem::size_of::<WHvPartitionMemoryCounters>(),                    24,                     "6");
    assert_eq!(mem::size_of::<WHvX64CpuidResult>(),                             32,                     "7");
    assert_eq!(mem::size_of::<WHvX64MSRExitBitmap>(),                           mem::size_of::<u64>(),  "8");
    assert_eq!(mem::size_of::<WHvProcessorRuntimeCounters>(),                   mem::size_of::<u128>(), "9");
    assert_eq!(mem::size_of::<WHvProcessorInterceptCounter>(),                  mem::size_of::<u128>(), "10");
    assert_eq!(mem::size_of::<WHvProcessorInterceptCounters>(),                 176,                    "11");
    assert_eq!(mem::size_of::<WHvProcessorEventCounters>(),                     24,                     "12");
    assert_eq!(mem::size_of::<WHvProcessorApicCounters>(),                      40,                     "13");
    assert_eq!(mem::size_of::<WHvUint128>(),                                    mem::size_of::<u128>(), "14");
    assert_eq!(mem::size_of::<WHvX64FPRegister>(),                              mem::size_of::<u128>(), "15");
    assert_eq!(mem::size_of::<WHvX64FPControlStatusRegister>(),                 mem::size_of::<u128>(), "16");
    assert_eq!(mem::size_of::<LastFpRipStruct>(),                               mem::size_of::<u64>(),  "17");
    assert_eq!(mem::size_of::<LastFpRegisterStruct>(),                          mem::size_of::<u64>(),  "18");
    assert_eq!(mem::size_of::<WHvX64XMMControlStatusRegister>(),                mem::size_of::<u128>(), "19");
    assert_eq!(mem::size_of::<XmmStruct>(),                                     mem::size_of::<u128>(), "20");
    assert_eq!(mem::size_of::<LastFpRdpStruct>(),                               mem::size_of::<u64>(),  "21");
    assert_eq!(mem::size_of::<LastFpStruct>(),                                  mem::size_of::<u64>(),  "22");
    assert_eq!(mem::size_of::<WHvX64SegmentRegister>(),                         mem::size_of::<u128>(), "23");
    assert_eq!(mem::size_of::<WHvX64SegmentRegisterAttrs>(),                    mem::size_of::<u16>(),  "24");
    assert_eq!(mem::size_of::<WHvX64TableRegister>(),                           mem::size_of::<u128>(), "25");
    assert_eq!(mem::size_of::<WHvX64InterrupStateRegister>(),                   mem::size_of::<u64>(),  "26");
    assert_eq!(mem::size_of::<WHvX64PendingInterruptionRegister>(),             mem::size_of::<u64>(),  "27");
    assert_eq!(mem::size_of::<WHvX64DeliverabilityNotificationsRegister>(),     mem::size_of::<u64>(),  "28");
    assert_eq!(mem::size_of::<WHvX64PendingExceptionEvent>(),                   mem::size_of::<u128>(), "29");
    assert_eq!(mem::size_of::<WHvX64PendingExtIntEvent>(),                      mem::size_of::<u128>(), "30");
    assert_eq!(mem::size_of::<WHvRegisterValue>(),                              mem::size_of::<u128>(), "31");
    assert_eq!(mem::size_of::<WHvInterruptControl>(),                           mem::size_of::<u128>(), "32");
}




