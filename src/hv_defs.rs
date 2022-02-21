/*
 *
 * Types, structs and enums defintions
 * for the Windows Hyper-V Platform API
 *
 */

#![allow(non_snake_case)]


use bitf::bitf;


pub type PHandle = usize;
pub type HResult = u32;     // maybe should be an i32 ?

pub type WHvGuestPhysicalAddress = u64;
pub type WHvGuestVirtualAddress = u64;


#[repr(C)]
pub enum WHvCapabilityCode
{
    // Capabilities of the API implementation
    HypervisorPresent       = 0x00000000,
    CodeFeatures            = 0x00000001,
    ExtendedVmExits         = 0x00000002,

    // Capabilities of the system's processor
    ProcessorVendor         = 0x00001000,
    ProcessorFeature        = 0x00001001,
    ProcessorClFlushSize    = 0x00001002,
    ProcessorXsaveFeatures  = 0x00001003,
}

//
// Return values for WHvCapabilityCodeProcessorVendor
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WHvProcessorVendor
{
    AMD   = 0x0000,
    Intel = 0x0001,
    Hygon = 0x0002,
}

//
// Return values for WHvCapabilityCodeFeatures
//
#[bitf(u64, pp)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WHvCapabilityFeatures
{
    PartialUnmap_1:       (),
    LocalApicEmulation_1: (),
    Xsave_1:              (),
    DirtyPageTracking_1:  (),
    SpeculationControl_1: (),
    _reserved_59:         (),           
}

//
// Return values for WHvCapabilityCodeExtendedVmExits
//
#[bitf(u64, pp)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WHvExtendedVmExits
{
    X64CpuidExit_1:   (),			// RunVpExitReasonX64CPUID supported
    X64MSRExit_1:     (),			// RunVpExitX64ReasonMSRAccess supported
    X64RdtscExit_1:   (),			// WHvRunVpExitReasonX64Rdtsc supported
    _reserved_61:     (),
}

//
// Return values for WHvCapabilityCodeProcessorFeatures
//
#[bitf(u64, pp)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WHvProcessorFeatures
{
	Sse3Support_1: 				        (),
	LahfSahfSupport_1: 			        (),
	Ssse3Support_1: 				(),
	Sse4_1Support_1: 				(),
	Sse4_2Support_1: 				(),
	Sse4aSupport_1: 				(),
	XopSupport_1: 				        (),
	PopCntSupport_1: 				(),
	Cmpxchg16bSupport_1: 			        (),
	Altmovcr8Support_1: 			        (),
	LzcntSupport_1: 				(),
	MisAlignSseSupport_1: 		                (),
	MmxExtSupport_1: 				(),
	Amd3DNowSupport_1: 			        (),
	ExtendedAmd3DNowSupport_1:            	        (),
	Page1GbSupport_1: 			        (),
	AesSupport_1: 				        (),
	PclmulqdqSupport_1: 		        	(),
	PcidSupport_1: 				        (),
	Fma4Support_1: 				        (),
	F16CSupport_1: 				        (),
	RdRandSupport_1: 				(),
	RdWrFsGsSupport_1: 		        	(),
	SmepSupport_1: 				        (),
	EnhancedFastStringSupport_1:          	        (),
	Bmi1Support_1: 				        (),
	Bmi2Support_1: 				        (),
	_reserved_2: 					(),
	MovbeSupport_1 : 				(),
	_reserved_1: 					(),
	DepX87FPUSaveSupport_1: 	        	(),
	RdSeedSupport_1: 				(),
	AdxSupport_1: 				        (),
	IntelPrefetchSupport_1: 	        	(),
	SmapSupport_1: 				        (),
	HleSupport_1: 				        (),
	RtmSupport_1: 				        (),
	RdtscpSupport_1: 				(),
	ClflushoptSupport_1: 		        	(),
	ClwbSupport_1: 			    	        (),
	ShaSupport_1: 				        (),
	X87PointersSavedSupport_1:            	        (),
	InvpcidSupport_1: 		        	(),
	IbrsSupport_1: 				        (),
	StibpSupport_1: 				(),
	IbpbSupport_1: 				        (),
	_reserved_1: 					(),
	SsbdSupport_1:				        (),
	FastShortRepMovSupport_1:             	        (),
	_reserved_1: 					(),
	RdclNo_1: 					(),
	IbrsAllSupport_1: 		        	(),
	_reserved_1: 					(),
	SsbNo_1: 					(),
	RsbANo_1: 					(),
	_reserved_8: 					(),
}

#[bitf(u64, pp)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WHvProcessorXsaveFeatures
{
	XsaveSupport_1: 		(),
	XsaveoptSupport_1: 		(),
	AvxSupport_1: 			(),
	Avx2Support_1: 			(),
	FmaSupport_1: 			(),
	MpxSupport_1: 			(),
	Avx512Support_1:	 	(),
	Avx512DQSupport_1: 		(),
	Avx512CDSupport_1: 		(),
	Avx512BWSupport_1: 		(),
	Avx512VLSupport_1: 		(),
	XsaveCompSupport_1: 		(),
	XsaveSupervisorSupport_1:       (),
	Xcr1Support_1: 			(),
	Avx512BitalgSupport_1: 	        (),
	Avx512IfmaSupport_1:	 	(),
	Avx512VBmiSupport_1: 		(),
	Avx512VBmi2Support_1: 	        (),
	Avx512VnniSupport_1: 		(),
	GfniSupport_1: 			(),
	VaesSupport_1: 			(),
	Avx512VPopcntdqSupport_1:       (),
	VpclmulqdqSupport_1: 		(),
	_reserved_4: 			(),	
}

//
// WHvGetCapability Output buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union WHvCapability
{
	pub HypervisorPresent: 		bool,
	pub Features: 			WHvCapabilityFeatures,
	pub ExtendedVmExits: 		WHvExtendedVmExits,
	pub ProcessorVendor:		WHvProcessorVendor,
	pub ProcessorFeatures:		WHvProcessorFeatures,
	pub ProcessorXsaveFeatures:	WHvProcessorXsaveFeatures,
	pub ProcessorClFlushSize:	u8,
	
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WHvPartitionCounterSet
{
	Memory,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvPartitionMemoryCounters
{
    pub Mapped4KPageCount: u64,
    pub Mapped2MPageCount: u64,
    pub Mapped1GPageCount: u64,
}

/*
 *
 * Partition Property Data Types
 *
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum WHvPartitionPropertyCode
{
	ExtendedVmExits 	= 0x00000001,
	ExceptionExitBitmap 	= 0x00000002,
	SeparateSecurityDomain	= 0x00000003,
	ProcessorFeatures	= 0x00001001,
	ProcessorClFlushSize	= 0x00001002,
	CpuidExitList		= 0x00001003,
	CpuidResultList		= 0x00001004,
	LocalApicEmulationMode  = 0x00001005,
	ProcessorXsaveFeatures  = 0x00001006,
	ProcessorCount		= 0x00001fff,
}

//
// WHvPartitionPropertyCodeCpuidResultList input buffer list element.
//
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64CpuidResult
{
    Function:	u32,
    Reserved:	[u32;3],
    Eax: 	u32,
    Ebx: 	u32,
    Ecx: 	u32,
    Edx: 	u32,
}
 
//
// WHvPartitionPropertyCodeExceptionBitmap enumeration values.
//

pub type WHvExceptionType = WHvX64ExceptionType;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum WHvX64ExceptionType
{
    DivideErrorFault 			= 0x0,
    DebugTrapOrFault 			= 0x1,
    BreakpointTrap 			= 0x3,
    OverflowTrap 			= 0x4,
    BoundRangeFault 			= 0x5,
    InvalidOpcodeFault 			= 0x6,
    DeviceNotAvailableFault 		= 0x7,
    DoubleFaultAbort 			= 0x8,
    InvalidTaskStateSegmentFault 	= 0x0A,
    SegmentNotPresentFault 		= 0x0B,
    StackFault 				= 0x0C,
    GeneralProtectionFault 		= 0x0D,
    PageFault 				= 0x0E,
    FloatingPointErrorFault 		= 0x10,
    AlignmentCheckFault 		= 0x11,
    MachineCheckAbort 			= 0x12,
    SimdFloatingPointFault 		= 0x13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WHvX64LocalApicEmulationMode
{
    ModeNone,
    ModeXApic,
}

//
// Return value for WHvCapabilityCodeX64MsrExits and input buffer for
// WHvPartitionPropertyCodeX64MsrcExits
//
#[bitf(u64)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WHvX64MSRExitBitmap
{
	UnhandledMsrs_1:    (),
	TscMsrWrite_1:	    (),
	TscMsrRead_1:	    (),
	Reserved_61:	    (),
}

//
// WHvGetPartitionProperty output buffer / WHvSetPartitionProperty input buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union WHvPartitionProperty
{
    ExtendedVmExits:		WHvExtendedVmExits,
    ProcessorFeatures:		WHvProcessorFeatures,
    ProcessorXsaveFeatures: 	WHvProcessorXsaveFeatures,
    ProcessorClFlushSize: 	u8,
    ProcessorCount:		u32,
    CpuidExitList: 		[u32;1],
    CpuidResultList: 		[WHvX64CpuidResult;1],
    ExceptionExitBitmap: 	u64,
    LocalApicEmulationMode: 	WHvX64LocalApicEmulationMode,
    SeparateSecurityDomain: 	bool,
    NestedVirtualization: 	bool,
    WHvX64MsrExitBitmap: 	WHvX64MSRExitBitmap,
    ProcessorClockFrequency:	u64,
    InterruptClockFrequency:	u64,
}

#[repr(C)]
#[derive(Debug)]
pub enum WHvProcessorCounterSet
{
    Runtime,
    Intercepts,
    Events,
    Apic,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorRuntimeCounters
{
    TotalRuntime100ns: 	    u64,
    HypervisorRuntime100ns: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorInterceptCounter
{
    Count: 	u64,
    Time100ns: 	u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorInterceptCounters
{
     PageInvalidations: 	WHvProcessorInterceptCounter,
     ControlRegisterAccesses: 	WHvProcessorInterceptCounter,
     IoInstructions: 		WHvProcessorInterceptCounter,
     HaltInstructions: 		WHvProcessorInterceptCounter,
     CpuidInstructions: 	WHvProcessorInterceptCounter,
     MsrAccesses: 		WHvProcessorInterceptCounter,
     OtherIntercepts: 		WHvProcessorInterceptCounter,
     PendingInterrupts: 	WHvProcessorInterceptCounter,
     EmulatedInstructions: 	WHvProcessorInterceptCounter,
     DebugRegisterAccesses: 	WHvProcessorInterceptCounter,
     PageFaultIntercepts: 	WHvProcessorInterceptCounter,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorEventCounters
{
    PageFaultCount: u64,
    ExceptionCount: u64,
    InterruptCount: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorApicCounters
{
    MmioAccessCount: 	u64,
    EoiAccessCount: 	u64,
    TprAccessCount: 	u64,
    SentIpiCount: 	u64,
    SelfIpiCount: 	u64,
}

/*
 *
 * GetVirtualProcessorRegister data types definitions
 *
 */

//
// Virtual Processor Register Definitions
//
#[repr(C)]
#[derive(Debug)]
pub enum WHvRegisterName
{
    // X64 General purpose registers
    Rax              = 0x00000000,
    Rcx              = 0x00000001,
    Rdx              = 0x00000002,
    Rbx              = 0x00000003,
    Rsp              = 0x00000004,
    Rbp              = 0x00000005,
    Rsi              = 0x00000006,
    Rdi              = 0x00000007,
    R8               = 0x00000008,
    R9               = 0x00000009,
    R10              = 0x0000000A,
    R11              = 0x0000000B,
    R12              = 0x0000000C,
    R13              = 0x0000000D,
    R14              = 0x0000000E,
    R15              = 0x0000000F,
    Rip              = 0x00000010,
    Rflags           = 0x00000011,

    // X64 Segment registers
    Es               = 0x00000012,
    Cs               = 0x00000013,
    Ss               = 0x00000014,
    Ds               = 0x00000015,
    Fs               = 0x00000016,
    Gs               = 0x00000017,
    Ldtr             = 0x00000018,
    Tr               = 0x00000019,

    // X64 Table registers
    Idtr             = 0x0000001A,
    Gdtr             = 0x0000001B,

    // X64 Control Registers
    Cr0              = 0x0000001C,
    Cr2              = 0x0000001D,
    Cr3              = 0x0000001E,
    Cr4              = 0x0000001F,
    Cr8              = 0x00000020,

    // X64 Debug Registers
    Dr0              = 0x00000021,
    Dr1              = 0x00000022,
    Dr2              = 0x00000023,
    Dr3              = 0x00000024,
    Dr6              = 0x00000025,
    Dr7              = 0x00000026,

    // X64 Extended Control Registers
    XCr0             = 0x00000027,

    // X64 Floating Point and Vector Registers
    Xmm0             = 0x00001000,
    Xmm1             = 0x00001001,
    Xmm2             = 0x00001002,
    Xmm3             = 0x00001003,
    Xmm4             = 0x00001004,
    Xmm5             = 0x00001005,
    Xmm6             = 0x00001006,
    Xmm7             = 0x00001007,
    Xmm8             = 0x00001008,
    Xmm9             = 0x00001009,
    Xmm10            = 0x0000100A,
    Xmm11            = 0x0000100B,
    Xmm12            = 0x0000100C,
    Xmm13            = 0x0000100D,
    Xmm14            = 0x0000100E,
    Xmm15            = 0x0000100F,
    FpMmx0           = 0x00001010,
    FpMmx1           = 0x00001011,
    FpMmx2           = 0x00001012,
    FpMmx3           = 0x00001013,
    FpMmx4           = 0x00001014,
    FpMmx5           = 0x00001015,
    FpMmx6           = 0x00001016,
    FpMmx7           = 0x00001017,
    FpControlStatus  = 0x00001018,
    XmmControlStatus = 0x00001019,

    // X64 MSRs
    Tsc              = 0x00002000,
    Efer             = 0x00002001,
    KernelGsBase     = 0x00002002,
    ApicBase         = 0x00002003,
    Pat              = 0x00002004,
    SysenterCs       = 0x00002005,
    SysenterEip      = 0x00002006,
    SysenterEsp      = 0x00002007,
    Star             = 0x00002008,
    Lstar            = 0x00002009,
    Cstar            = 0x0000200A,
    Sfmask           = 0x0000200B,

    MsrMtrrCap         = 0x0000200D,
    MsrMtrrDefType     = 0x0000200E,

    MsrMtrrPhysBase0   = 0x00002010,
    MsrMtrrPhysBase1   = 0x00002011,
    MsrMtrrPhysBase2   = 0x00002012,
    MsrMtrrPhysBase3   = 0x00002013,
    MsrMtrrPhysBase4   = 0x00002014,
    MsrMtrrPhysBase5   = 0x00002015,
    MsrMtrrPhysBase6   = 0x00002016,
    MsrMtrrPhysBase7   = 0x00002017,
    MsrMtrrPhysBase8   = 0x00002018,
    MsrMtrrPhysBase9   = 0x00002019,
    MsrMtrrPhysBaseA   = 0x0000201A,
    MsrMtrrPhysBaseB   = 0x0000201B,
    MsrMtrrPhysBaseC   = 0x0000201C,
    MsrMtrrPhysBaseD   = 0x0000201D,
    MsrMtrrPhysBaseE   = 0x0000201E,
    MsrMtrrPhysBaseF   = 0x0000201F,

    MsrMtrrPhysMask0   = 0x00002040,
    MsrMtrrPhysMask1   = 0x00002041,
    MsrMtrrPhysMask2   = 0x00002042,
    MsrMtrrPhysMask3   = 0x00002043,
    MsrMtrrPhysMask4   = 0x00002044,
    MsrMtrrPhysMask5   = 0x00002045,
    MsrMtrrPhysMask6   = 0x00002046,
    MsrMtrrPhysMask7   = 0x00002047,
    MsrMtrrPhysMask8   = 0x00002048,
    MsrMtrrPhysMask9   = 0x00002049,
    MsrMtrrPhysMaskA   = 0x0000204A,
    MsrMtrrPhysMaskB   = 0x0000204B,
    MsrMtrrPhysMaskC   = 0x0000204C,
    MsrMtrrPhysMaskD   = 0x0000204D,
    MsrMtrrPhysMaskE   = 0x0000204E,
    MsrMtrrPhysMaskF   = 0x0000204F,

    MsrMtrrFix64k00000 = 0x00002070,
    MsrMtrrFix16k80000 = 0x00002071,
    MsrMtrrFix16kA0000 = 0x00002072,
    MsrMtrrFix4kC0000  = 0x00002073,
    MsrMtrrFix4kC8000  = 0x00002074,
    MsrMtrrFix4kD0000  = 0x00002075,
    MsrMtrrFix4kD8000  = 0x00002076,
    MsrMtrrFix4kE0000  = 0x00002077,
    MsrMtrrFix4kE8000  = 0x00002078,
    MsrMtrrFix4kF0000  = 0x00002079,
    MsrMtrrFix4kF8000  = 0x0000207A,

    TscAux           = 0x0000207B,
    SpecCtrl         = 0x00002084,
    PredCmd          = 0x00002085,
    TscVirtualOffset = 0x00002087,

    // APIC state (also accessible via WHv(Get/Set)VirtualProcessorInterruptControllerState)
    WHvX64RegisterApicId           = 0x00003002,
    WHvX64RegisterApicVersion      = 0x00003003,

    // Interrupt / Event Registers
    PendingInterruption 		= 0x80000000,
    InterruptState      		= 0x80000001,
    PendingEvent        		= 0x80000002,
    DeliverabilityNotifications         = 0x80000004,
    InternalActivityState 		= 0x80000005,

}

#[repr(align(16))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvUint128
{
    pub val: u128,
}

trait LoHi<T>
{
    fn lo(self: &Self) -> T;
    fn hi(self: &Self) -> T;
}

impl LoHi<u64> for WHvUint128
{
    fn lo(self: &Self) -> u64
    {
        let mask: u128 = u64::MAX as u128;
        (self.val & mask) as u64 
    }

    fn hi(self: &Self) -> u64
    {
        let mask: u128 = (u64::MAX as u128) << 64;
        ((self.val & mask) >> 64) as u64
    }
}

#[bitf(u128)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64FPRegister
{
	Mantissa_64: 		(),
	BiasedExponent_15:      (),
	Sign_1: 		(),
	_reserved_48:		(),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WHvX64FPControlStatusRegister
{
    FpControl:  u16,
    FpStatus:   u16,
    FpTag:      u8,
    Reserved:   u8,
    LastFpOp:   u16,
    LastFpRip:  LastFpRipStruct,
    AsUINT128:  WHvUint128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LastFpRipStruct
{
    LastFpRip:      u64,
    LastFpRegister: LastFpRegisterStruct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LastFpRegisterStruct
{
    LaspFpEip:  u32,
    LastFpCs:   u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WHvX64XMMControlStatusRegister
{
    XmmControl: XmmStruct,
    AsUINT128:  WHvUint128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XmmStruct
{
    LastFpRdp:              LastFpRdpStruct,
    XmmStatusControl:       u32,
    XmmStatusControlMask:   u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LastFpRdpStruct
{
    LastFpRdp:  u64,
    LastFp:     LastFpStruct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LastFpStruct
{
    LastFpDp:   u32,
    LastFpDs:   u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64SegmentRegister
{
    Base:           u64,
    Limit:          u32,
    Selector:       u16,
    Attributes:     WHvX64SegmentRegisterAttrs,
}

#[bitf(u16)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WHvX64SegmentRegisterAttrs
{
    SegmentType_4:              (),
    NonSystemSegment_1:         (),
    DescriptorPrivilegeLevel_2: (),
    Present_1:                  (),
    _reserved_4:                (),
    Available_1:                (),
    Long_1:                     (),
    Default_1:                  (),
    Granularity_1:              (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64TableRegister
{
    Pad:    [u16;3],
    Limit:  u16,
    Base:   u64,
}

#[bitf(u64)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64InterrupStateRegister
{
    InterruptShadow_1:      (),
    NmiMasked_1:            (),
    _reserved_62:           (),
}

#[bitf(u64)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64PendingInterruptionRegister
{
    InterruptionPending_1:      u32,
    InterruptionType_3:         u32,     // TODO: replace by a WHV_X64_PENDING_INTERRUPTION_TYPE return type
    DeliverErrorCode_1:         u32,
    InstructionLength_4:        u32,
    NestedEvent_1:              u32,
    _reserved_6:                u32,
    InterruptionVector_16:      u32,
    ErrorCode_32:               u32,
}

#[bitf(u64)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64DeliverabilityNotificationsRegister
{
    NmiNotification_1:          (),
    InterruptNotification_1:    (),
    InterruptPriority_4:        (),
    _reserved_58:               (),
}

#[repr(C)]
#[derive(Debug)]
pub enum WHvX64PendingEventType
{
    Exception   =   0,
    ExtInt      =   5,
}

#[bitf(u128)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64PendingExceptionEvent
{
    EventPending_1:         u32,
    EventType_3:            u32, //TODO: Should be WhvX64PendingEventException
    _reserved0_4:           (),
    DeliverErrorCode_1:     u32,
    _reserved1_7:           (),
    Vector_16:              u32,
    ErrorCode_32:           u32,
    ExceptionParameter_64:  u64,
}

#[bitf(u128)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WHvX64PendingExtIntEvent
{
    EventPending_1:     u64,
    EventType_3:        u64,    //TODO: should be a WHvX64PendingEventExtInt
    _reserved0_4:       u64,
    Vector_8:           u64,
    _reserved1_48:      u64,
    _reserved2_64:      u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union WHvRegisterValue
{
    pub Reg128:                         WHvUint128,
    pub Reg64:                          u64,
    pub Reg32:                          u32,
    pub Reg16:                          u16,
    pub Reg8:                           u8,
    pub Fp:                             WHvX64FPRegister,
    pub FpControlStatus:                WHvX64FPControlStatusRegister,
    pub XmmControlStatus:               WHvX64XMMControlStatusRegister,
    pub Segment:                        WHvX64SegmentRegister,
    pub Table:                          WHvX64TableRegister,
    pub InterruptState:                 WHvX64InterrupStateRegister,
    pub PendingInterruption:            WHvX64PendingInterruptionRegister,
    pub DeliverabilityNotifications:    WHvX64DeliverabilityNotificationsRegister,
    pub ExceptionEvent:                 WHvX64PendingExceptionEvent,
    pub ExtIntEvent:                    WHvX64PendingExtIntEvent,
}

/*
 * WHvMapGpaRange
 */
#[bitf(u32)]
#[repr(C)]
pub struct WHvMapGpaRangeFlags
{
    RangeNone_1:        (),
    Read_1:             (),
    Write_1:            (),
    Execute_1:          (),
    TrackDirtyPages_1:  (),
}


/*
 * WHvRequestInterrupt
 */
#[repr(C)]
pub enum WHvInterruptType
{
    Fixed           = 0,
    LowestPriority  = 1,
    TypeNmi         = 4,
    TypeInit        = 5,
    TypeSipi        = 6,
    TypeLocalInt1   = 9
}

#[repr(C)]
pub enum WHvInterruptDestinationMode
{
    Physical,
    Logical,
}

#[repr(C)]
pub enum WHvInterruptTriggerMode
{
    Edge,
    Level,
}

#[bitf(u128)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WHvInterruptControl
{
    Type_8:             u64,
    DestinationMode_4:  u64,
    TriggerMode_4:      u64,
    _reserved_48:       (),
    Destination_32:     u32,
    Vector_32:          u32,
}

/*
 * WHvRunVirtualProcessor
 */
// Exit Reasons
#[repr(C)]
pub enum WHvRunVpExitReason
{
    ReasonNone              = 0x00000000,
    
    // Standard exits caused by operations of the virtual processor
    MemoryAccess            = 0x00000001,
    X64IoPortAccess         = 0x00000002,
    UnrecoverableException  = 0x00000004,
    InvalidVpRegisterValue  = 0x00000005,
    UnsupportedFeature      = 0x00000006,
    X64InterruptWindow      = 0x00000007,
    X64Halt                 = 0x00000008,
    X64ApicEoi              = 0x00000009,

    // Additional exits that can be configured through partition properties
    X64MsrAccess            = 0x00001000,
    X64Cpuid                = 0x00001001,
    Exception               = 0x00001002,

    // Exits caused by the host
    Canceled                = 0x00002001,
}


//
// Execution state of the virtual processor
//
#[bitf(u16)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WHvX64VpExecutionState
{
    Cpl_2:                  (),
    Cr0Pe_1:                (),
    Cr0Am_1:                (),
    EferLma_1:              (),
    DebugActive_1:          (),
    InterruptionPending_1:  (),
    _reserved0_5:           (),
    InterruptShadow_1:      (),
    _reserved1_3:           (),
}

//
// Execution context of a virtual processor at the time of an exit
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WHvVpExitContext
{
    ExecutionState: WHvX64VpExecutionState,
    InstLenCr8:     InstructionLenCr8,
    Reserved:       u8,
    Reserved2:      u32,
    Cs:             WHvX64SegmentRegister,
    Rip:            u64,
    RFlags:         u64,
}

// included in WHvVpExitContext
#[bitf(u8)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InstructionLenCr8
{
    InstructionLength_4:    (),
    Cr8_4:                  (),
}

//
// Context data for a VM exit caused by a memory access (WHvRunVpExitReasonMemoryAccess)
//
#[repr(C)]
pub enum WHvMemoryAccessType
{
    Read    = 0,
    Write   = 1,
    Execute = 2,
}

#[bitf(u32)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WHvMemoryAccessInfo
{
    AccessType_2:       (),
    GpaUnmapped_1:      (),
    GvaValid_1:         (),
    _reserved_28:       (),
}

#[repr(C)]
pub struct WHvMemoryAccessContext
{
    // Context of the virtual processor
    InstructionByteCount:   u8,
    InstructionBytes:       [u8;16],

    // Memory access info
    AccessInfo:             WHvMemoryAccessInfo,
    Gpa:                    WHvGuestPhysicalAddress,
    Gva:                    WHvGuestVirtualAddress,
}

//
// Context data for an exit caused by an I/O port access (WHvRunVpExitReasonX64IOPortAccess)
//
#[bitf(u32)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WHvX64IOPortAccessInfo
{
    IsWrite_1:      (),
    AccessSize_3:   (),
    StringOp_1:     (),
    RepPrefix_1:    (),
    _reserved_26:   (),
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WHvX64IOPortAccessContext
{
    // Context of the virtual processor
    InstructionByteCount:   u8,
    InstructionBytes:       [u8;16],

    // I/O Port Access Info
    AccessInfo:     WHvX64IOPortAccessInfo,
    PortNumber:     u16,
    Rax:            u64,
    Rcx:            u64,
    Rsi:            u64,
    Rdi:            u64,
    Ds:             WHvX64SegmentRegister,
    Es:             WHvX64SegmentRegister,
}

//
// Context data for an exit caused by an MSR access (WHvRunVpExitReasonX64MSRAccess)
//
#[bitf(u32)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvX64MSRAccessInfo
{
    IsWrite_1:      (),
    _reserved_31:   (),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvX64MSRAccessContext
{
    AccessInfo:     WHvX64MSRAccessInfo,
    MsrNumber:      u32,
    Rax:            u64,
    Rdx:            u64,
}

//
// Context data for an exit caused by a CPUID call (WHvRunVpExitReasonX64CPUID)
//
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvX64CPUIDAccessContext
{
    Rax:                u64,
    Rcx:                u64,
    Rdx:                u64,
    Rbx:                u64,
    DefaultResultRax:   u64,
    DefaultResultRcx:   u64,
    DefaultResultRdx:   u64,
    DefaultResultRbx:   u64,
}

//
// Context data for an exit caused by an exception generated by the virtual processor
// (WHvRunVpExitReasonException)
//

#[bitf(u32)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WHvVpExceptionInfo
{
    ErrorCodeValid_1:       (),
    SoftwareException_1:    (),
    _reserved_30:           (),
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WHvVpExceptionContext
{
    InstructionByteCount:   u8,
    Reserved:               [u8;3],
    InstructionBytes:       [u8;16],

    // Exception Info
    ExceptionInfo:          WHvVpExceptionInfo,
    ExceptionType:          u8,     // WHvExceptionType
    Reserved2:              [u8;3],
    ErrorCode:              u32,
    ExceptionParameter:     u64,
}

//
// Context data for an exit caused by an interrupt delivery window cancellation from the host
// (WHvRunVpExitReasonX64InterruptWindow)
// 

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum WHvX64PendingInterruptionType
{
    Interrupt   = 1,
    Nmi         = 2,
    Exception   = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvX64InterruptionDeliverableContext
{
    DeliverableType:    WHvX64PendingInterruptionType,
}

//
// Context data for an exit caused by the use of an unsupported processor feature
// (WHvRunVpExitReasonUnsupportedFeature)
//

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum WHvX64UnsupportedFeatureCode
{
    Intercept       = 1,
    TaskSwitchTss   = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvX64UnsupportedFeatureContext
{
    FeatureCode:        WHvX64UnsupportedFeatureCode,
    FeatureParameter:   u64,
}

//
// Context data for an exit caused by a cancellation from the host (WHvRunVpExitReasonCanceled)
//

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum WHvRunVpCancelReason
{
    WHvRunVpCancelReasonUser = 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WHvRunVpCanceledContext
{
    CancelReason:   WHvRunVpCancelReason,
}

//
// WHvTranslateGva
//

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum WHvTranslateGVAFlags
{
    None                =   0x00000000,
    ValidateRead        =   0x00000001,
    ValidateWrite       =   0x00000002,
    ValidateExecute     =   0x00000004,
    PrivilegeExempt     =   0x00000008,
    SetPageTableBits    =   0x00000010,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum WHvTranslateGVAResultCode
{
    Success = 0,

    // Translation failures
    PageNotPresent = 1,
    PrivilegeViolation = 2,
    InvalidPageTableFlags = 3,
    
    // GPA Access failures
    GpaUnmapped = 4,
    GpaNoReadAccess = 5,
    GpaNoWriteAccess = 6,
    GpaIllegalOverlayAccess = 7,
    Intercept = 8,
}

// Output buffer of WHvTranslateGva
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WHvTranslateGVAResult
{
    ResultCode:     WHvTranslateGVAResultCode,
    Reserved:       u32,
}






