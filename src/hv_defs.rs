/*
 *
 * Types, structs and enums defintions
 * for the Windows Hyper-V Platform API
 *
 */

use bitf::bitf;

pub type PHandle = usize;
pub type HResult = u32;     // maybe should be an i32 ?


bits!(WHvProcessorVendor, u64, [rax:8.1,rbx:8.8,rcx:8.16]);

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
#[derive(Debug)]
pub struct WHvCapabilityFeatures
{
    PartialUnmap_1:       (),
    LocalApicEmulation_1: (),
    Xsave_1:              (),
    DirtyPageTracking_1:  (),
    SpeculationControl_1: (),
    _reserved_59_1:       (),           
}

//
// Return values for WHvCapabilityCodeExtendedVmExits
//
#[bitf(u64, pp)]
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct WHvProcessorFeatures
{
	// CPUID.01H:ECX.SSE3[bit 0] = 1
	Sse3Support_1: 				        (),
	// CPUID.80000001H:ECX.LAHF-SAHF[bit 0] = 1
	LahfSahfSupport_1: 			        (),
	// CPUID.01H:ECX.SSSE3[bit 9] = 1
	Ssse3Support_1: 				(),
	// CPUID.01H:ECX.SSE4_1[bit 19] = 1
	Sse4_1Support_1: 				(),
	// CPUID.01H:ECX.SSE4_2[bit 20] = 1
	Sse4_2Support_1: 				(),
	// CPUID.80000001H:ECX.SSE4A[bit 6]
	Sse4aSupport_1: 				(),
	// CPUID.80000001H:ECX.XOP[bit 11]
	XopSupport_1: 				        (),
	// CPUID.01H:ECX.POPCNT[bit 23] = 1
	PopCntSupport_1: 				(),
	// CPUID.01H:ECX.CMPXCHG16B[bit 13] = 1
	Cmpxchg16bSupport_1: 			        (),
	// CPUID.80000001H:ECX.AltMovCr8[bit 4]
	Altmovcr8Support_1: 			        (),
	// CPUID.80000001H:ECX.LZCNT[bit 5] = 1
	LzcntSupport_1: 				(),
	// CPUID.80000001H:ECX.MisAlignSse[bit 7]
	MisAlignSseSupport_1: 		                (),
	// CPUID.80000001H:EDX.MmxExt[bit 22]
	MmxExtSupport_1: 				(),
	// CPUID.80000001H:EDX.3DNow[bit 31]
	Amd3DNowSupport_1: 			        (),
	// CPUID.80000001H:EDX.3DNowExt[bit 30]
	ExtendedAmd3DNowSupport_1:            	        (),
	// CPUID.80000001H:EDX.Page1GB[bit 26] = 1
	Page1GbSupport_1: 			        (),
	// CPUID.01H:ECX.AES[bit 25]
	AesSupport_1: 				        (),
	/* CPUID.01H:ECX.PCLMULQDQ[bit 1] = 1 */
	PclmulqdqSupport_1: 		        	(),
	/* CPUID.01H:ECX.PCID[bit 17] */
	PcidSupport_1: 				        (),
	/* CPUID.80000001H:ECX.FMA4[bit 16] = 1 */
	Fma4Support_1: 				        (),
	/* CPUID.01H:ECX.F16C[bit 29] = 1 */
	F16CSupport_1: 				        (),
	/* CPUID.01H:ECX.RDRAND[bit 30] = 1 */
	RdRandSupport_1: 				(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.FSGSBASE[bit 0] */
	RdWrFsGsSupport_1: 		        	(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.SMEP[bit 7] */
	SmepSupport_1: 				        (),
	/* IA32_MISC_ENABLE.FastStringsEnable[bit 0] = 1 */
	EnhancedFastStringSupport_1:          	        (),
	/* CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3] = 1 */
	Bmi1Support_1: 				        (),
	/* CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8] = 1 */
	Bmi2Support_1: 				        (),
	_reserved_2: 					(),
	/* CPUID.01H:ECX.MOVBE[bit 22] = 1 */
	MovbeSupport_1 : 				(),
	_reserved_1: 					(),
	/* CPUID.(EAX=07H, ECX=0H):EBX[bit 13] = 1 */
	DepX87FPUSaveSupport_1: 	        	(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.RDSEED[bit 18] = 1 */
	RdSeedSupport_1: 				(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] */
	AdxSupport_1: 				        (),
	/* CPUID.80000001H:ECX.PREFETCHW[bit 8] = 1 */
	IntelPrefetchSupport_1: 	        	(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.SMAP[bit 20] = 1 */
	SmapSupport_1: 				        (),
	/* CPUID.(EAX=07H, ECX=0H):EBX.HLE[bit 4] = 1 */
	HleSupport_1: 				        (),
	/* CPUID.(EAX=07H, ECX=0H):EBX.RTM[bit 11] = 1 */
	RtmSupport_1: 				        (),
	/* CPUID.80000001H:EDX.RDTSCP[bit 27] = 1 */
	RdtscpSupport_1: 				(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.CLFLUSHOPT[bit 23] */
	ClflushoptSupport_1: 		        	(),
	/* CPUID.(EAX=07H, ECX=0H):EBX.CLWB[bit 24] = 1 */
	ClwbSupport_1: 			    	        (),
	/* CPUID.(EAX=07H, ECX=0H):EBX.SHA[bit 29] */
	ShaSupport_1: 				        (),
	/* CPUID.80000008H:EBX[bit 2] = 1 (AMD only) */
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

#[repr(C)]
pub struct WHvProcessorXsaveFeatures
{
	pub XsaveSupport: 			u8,
	pub XsaveoptSupport: 		u8,
	pub AvxSupport: 			u8,
	pub Avx2Support: 			u8,
	pub FmaSupport: 			u8,
	pub MpxSupport: 			u8,
	pub Avx512Support:	 		u8,
	pub Avx512DQSupport: 		u8,
	pub Avx512CDSupport: 		u8,
	pub Avx512BWSupport: 		u8,
	pub Avx512VLSupport: 		u8,
	pub XsaveCompSupport: 		u8,
	pub XsaveSupervisorSupport: u8,
	pub Xcr1Support: 			u8,
	pub Avx512BitalgSupport: 	u8,
	pub Avx512IfmaSupport:	 	u8,
	pub Avx512VBmiSupport: 		u8,
	pub Avx512VBmi2Support: 	u8,
	pub Avx512VnniSupport: 		u8,
	pub GfniSupport: 			u8,
	pub VaesSupport: 			u8,
	pub Avx512VPopcntdqSupport: u8,
	pub VpclmulqdqSupport: 		u8,
	pub Reserved: 			 	[4;u8],	


}

//
// WHvGetCapability Output buffer
//
pub union WHvCapability
{
	pub HypervisorPresent: 		bool,
	pub Features: 				WHvCapabilityFeatures,
	pub ExtendedVmExits: 		WHvExtendedVmExits,
	pub ProcessorVendor:		WHvProcessorVendor,
	pub ProcessorFeatures:		WHvProcessorFeatures,
	pub ProcessorXsaveFeatures:	WHvProcessorXsaveFeatures,
	pub ProcessorClFlushSize:	u8,
	
}

#[repr(C)]
pub enum WHvPartitionCounterSet
{
	Memory,
}

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub enum WHvPartitionPropertyCode
{
	ExtendedVmExits 		= 0x00000001,
	ExceptionExitBitmap 	= 0x00000002,
	SeparateSecurityDomain	= 0x00000003,
	ProcessorFeatures		= 0x00001001,
	ProcessorClFlushSize	= 0x00001002,
	CpuidExitList			= 0x00001003,
	CpuidResultList			= 0x00001004,
	LocalApicEmulationMode  = 0x00001005,
	ProcessorXsaveFeatures  = 0x00001006,
	ProcessorCount			= 0x00001fff,
}

//
// WHvPartitionPropertyCodeCpuidResultList input buffer list element.
//
#[repr(C)]
#[derive(Debug)]
pub struct WHvX64CpuidResult
{
    Function:	u32,
    Reserved:	[u32;3],
   	Eax: 		u32,
    Ebx: 		u32,
    Ecx: 		u32,
    Edx: 		u32,
}
 
//
// WHvPartitionPropertyCodeExceptionBitmap enumeration values.
//
#[repr(C)]
#[derive(Debug)]
// pub enum WHvExceptionType
pub enum WHvX64ExceptionType
{
    DivideErrorFault 				= 0x0,
    DebugTrapOrFault 				= 0x1,
    BreakpointTrap 					= 0x3,
    OverflowTrap 					= 0x4,
    BoundRangeFault 				= 0x5,
    InvalidOpcodeFault 				= 0x6,
    DeviceNotAvailableFault 		= 0x7,
    DoubleFaultAbort 				= 0x8,
    InvalidTaskStateSegmentFault 	= 0x0A,
    SegmentNotPresentFault 			= 0x0B,
    StackFault 						= 0x0C,
    GeneralProtectionFault 			= 0x0D,
    PageFault 						= 0x0E,
    FloatingPointErrorFault 		= 0x10,
    AlignmentCheckFault 			= 0x11,
    MachineCheckAbort 				= 0x12,
    SimdFloatingPointFault 			= 0x13,
}

#[repr(C)]
#[derive(Debug)]
pub enum WHvX64LocalApicEmulationMode
{
    None,
    XApic,
}

//
// Return value for WHvCapabilityCodeX64MsrExits and input buffer for
// WHvPartitionPropertyCodeX64MsrcExits
//
#[repr(C)]
#[derive(Debug)]
pub struct WHvX64MSRExitBitmap
{
	UnhandledMsrs:	u8,
	TscMsrWrite:	u8,
	TscMsrRead:		u8,
	Reserved:		[u8;61]
}

//
// WHvGetPartitionProperty output buffer / WHvSetPartitionProperty input buffer
//
#[repr(C)]
#[derive(Debug)]
pub union WHvPartitionProperty
{
    ExtendedVmExits:			WHvExtendedVmExits,
    ProcessorFeatures:			WHvProcessorFeatures,
    ProcessorXsaveFeatures: 	WHvProcessorXsaveFeatures,
    ProcessorClFlushSize: 		u8,
    ProcessorCount:				u32,
    CpuidExitList: 				[u32;1],
    CpuidResultList: 			[WHvX64CpuidResult;1],
    ExceptionExitBitmap: 		u64,
    LocalApicEmulationMode: 	WHvX64LocalApicEmulationMode,
    SeparateSecurityDomain: 	bool,
    NestedVirtualization: 		bool,
    WHvX64MsrExitBitmap: 		WHvX64MSRExitBitmap,
    ProcessorClockFrequency:	u64
    InterruptClockFrequency:	u64
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
    TotalRuntime100ns: 		u64,
    HypervisorRuntime100ns: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorInterceptCounter
{
    Count: 		u64,
    Time100ns: 	u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct WHvProcessorInterceptCounters
{
     PageInvalidations: 		WHvProcessorInterceptCounter,
     ControlRegisterAccesses: 	WHvProcessorInterceptCounter,
     IoInstructions: 			WHvProcessorInterceptCounter,
     HaltInstructions: 			WHvProcessorInterceptCounter,
     CpuidInstructions: 		WHvProcessorInterceptCounter,
     MsrAccesses: 				WHvProcessorInterceptCounter,
     OtherIntercepts: 			WHvProcessorInterceptCounter,
     PendingInterrupts: 		WHvProcessorInterceptCounter,
     EmulatedInstructions: 		WHvProcessorInterceptCounter,
     DebugRegisterAccesses: 	WHvProcessorInterceptCounter,
     PageFaultIntercepts: 		WHvProcessorInterceptCounter,
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
#[derive(Debug)y]
pub struct WHvProcessorApicCounters
{
    MmioAccessCount: 	u64,
    EoiAccessCount: 	u64,
    TprAccessCount: 	u64,
    SentIpiCount: 		u64,
    SelfIpiCount: 		u64,
}

/*
 *
 * GetVirtualProcessorRegister definitions
 *
 */

//
// Virtual Processor Register Definitions
//
#[repr(C)]
#[derive(Debug)y]
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
    DeliverabilityNotifications = 0x80000004,
    InternalActivityState 		= 0x80000005,

}

// TODO: use a u128 field ?
#[repr(align(16))]
#[repr(C)]
#[derive(Debug)]
pub struct u128
{
	Lo:	u64,
    Hi:	u64,
}
// TODO: impl: Shl Shr BitAnd BitOrAssign BitAndAssign 

#[repr(C)]
#[derive(Debug)]
pub struct WHvX64FPRegister
{
	Mantissa, 		u64,
	BiasedExponent:	[u64;15],
	Sign: 			[u64;1],
	Reserved:		[u64;48],
}
/*
typedef union WHV_X64_FP_CONTROL_STATUS_REGISTER
{
    struct
    {
        UINT16 FpControl;
        UINT16 FpStatus;
        UINT8  FpTag;
        UINT8  Reserved;
        UINT16 LastFpOp;
        union
        {
            // Long Mode
            UINT64 LastFpRip;

            // 32 Bit Mode
            struct
            {
                UINT32 LastFpEip;
                UINT16 LastFpCs;
            };
        };
    };

    WHV_UINT128 AsUINT128;
} WHV_X64_FP_CONTROL_STATUS_REGISTER;

typedef union WHV_X64_XMM_CONTROL_STATUS_REGISTER
{
    struct
    {
        union
        {
            // Long Mode
            UINT64 LastFpRdp;

            // 32 Bit Mode
            struct
            {
                UINT32 LastFpDp;
                UINT16 LastFpDs;
            };
        };
        UINT32 XmmStatusControl;
        UINT32 XmmStatusControlMask;
    };

    WHV_UINT128 AsUINT128;
} WHV_X64_XMM_CONTROL_STATUS_REGISTER;

typedef struct WHV_X64_SEGMENT_REGISTER
{
    UINT64 Base;
    UINT32 Limit;
    UINT16 Selector;

    union
    {
        struct
        {
            UINT16 SegmentType:4;
            UINT16 NonSystemSegment:1;
            UINT16 DescriptorPrivilegeLevel:2;
            UINT16 Present:1;
            UINT16 Reserved:4;
            UINT16 Available:1;
            UINT16 Long:1;
            UINT16 Default:1;
            UINT16 Granularity:1;
        };

        UINT16 Attributes;
    };
} WHV_X64_SEGMENT_REGISTER;

typedef struct WHV_X64_TABLE_REGISTER
{
    UINT16     Pad[3];
    UINT16     Limit;
    UINT64     Base;
} WHV_X64_TABLE_REGISTER;

typedef union WHV_X64_INTERRUPT_STATE_REGISTER
{
    struct
    {
        UINT64 InterruptShadow:1;
        UINT64 NmiMasked:1;
        UINT64 Reserved:62;
    };

    UINT64 AsUINT64;
} WHV_X64_INTERRUPT_STATE_REGISTER;

typedef union WHV_X64_PENDING_INTERRUPTION_REGISTER
{
    struct
    {
        UINT32 InterruptionPending:1;
        UINT32 InterruptionType:3;  // WHV_X64_PENDING_INTERRUPTION_TYPE
        UINT32 DeliverErrorCode:1;
        UINT32 InstructionLength:4;
        UINT32 NestedEvent:1;
        UINT32 Reserved:6;
        UINT32 InterruptionVector:16;
        UINT32 ErrorCode;
    };

    UINT64 AsUINT64;
} WHV_X64_PENDING_INTERRUPTION_REGISTER;

C_ASSERT(sizeof(WHV_X64_PENDING_INTERRUPTION_REGISTER) == sizeof(UINT64));

typedef union WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER
{
    struct
    {
        UINT64 NmiNotification:1;
        UINT64 InterruptNotification:1;
        UINT64 InterruptPriority:4;
        UINT64 Reserved:58;
    };

    UINT64 AsUINT64;
} WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER;

C_ASSERT(sizeof(WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER) == sizeof(UINT64));


typedef enum WHV_X64_PENDING_EVENT_TYPE
{
    WHvX64PendingEventException = 0,
    WHvX64PendingEventExtInt    = 5,
} WHV_X64_PENDING_EVENT_TYPE;

typedef union WHV_X64_PENDING_EXCEPTION_EVENT
{
    struct
    {
        UINT32 EventPending         : 1;
        UINT32 EventType            : 3; // Must be WHvX64PendingEventException
        UINT32 Reserved0            : 4;

        UINT32 DeliverErrorCode     : 1;
        UINT32 Reserved1            : 7;
        UINT32 Vector               : 16;
        UINT32 ErrorCode;
        UINT64 ExceptionParameter;
    };

    WHV_UINT128 AsUINT128;
} WHV_X64_PENDING_EXCEPTION_EVENT;

C_ASSERT(sizeof(WHV_X64_PENDING_EXCEPTION_EVENT) == sizeof(WHV_UINT128));

typedef union WHV_X64_PENDING_EXT_INT_EVENT
{
    struct
    {
        UINT64 EventPending     : 1;
        UINT64 EventType        : 3; // Must be WHvX64PendingEventExtInt
        UINT64 Reserved0        : 4;
        UINT64 Vector           : 8;
        UINT64 Reserved1        : 48;

        UINT64 Reserved2;
    };

    WHV_UINT128 AsUINT128;
} WHV_X64_PENDING_EXT_INT_EVENT;

C_ASSERT(sizeof(WHV_X64_PENDING_EXT_INT_EVENT) == sizeof(WHV_UINT128));

//
// Register values
//
typedef union WHV_REGISTER_VALUE
{
    WHV_UINT128 Reg128;
    UINT64 Reg64;
    UINT32 Reg32;
    UINT16 Reg16;
    UINT8 Reg8;
    WHV_X64_FP_REGISTER Fp;
    WHV_X64_FP_CONTROL_STATUS_REGISTER FpControlStatus;
    WHV_X64_XMM_CONTROL_STATUS_REGISTER XmmControlStatus;
    WHV_X64_SEGMENT_REGISTER Segment;
    WHV_X64_TABLE_REGISTER Table;
    WHV_X64_INTERRUPT_STATE_REGISTER InterruptState;
    WHV_X64_PENDING_INTERRUPTION_REGISTER PendingInterruption;
    WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER DeliverabilityNotifications;
    WHV_X64_PENDING_EXCEPTION_EVENT ExceptionEvent;
    WHV_X64_PENDING_EXT_INT_EVENT ExtIntEvent;
} WHV_REGISTER_VALUE;
*/
