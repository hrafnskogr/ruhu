use ruhu::hv_defs::{WHvPartitionProperty, WHvPartitionPropertyCode};
use ruhu::win_api::*;
use ruhu::hv_api::*;

pub fn main()
{
    let res = is_hypervisor_present();
    println!("Is hypervisor present: {}", res);

    /*
    for _ in 0..10
    {
        println!("{:x}", create_partition());
    }*/

    let phandle = create_partition();


    let mut property_buffer: WHvPartitionProperty = WHvPartitionProperty::default();
    property_buffer.ProcessorCount = 1;

    let property_ptr = Box::into_raw(Box::new(property_buffer)) as *mut WHvPartitionProperty;

    let res = set_partition_property(phandle, WHvPartitionPropertyCode::ProcessorCount, property_ptr);

    println!("{}", res);

    property_buffer.ExtendedVmExits.set_ExceptionExit(1);

    // create partition
    // set partition properties
    //  processor count
    //  ExtendedVmExits.ExceptionExit true
    //  ExceptionExitBitmap : BreakpointTrap / PageFault / General Protection Fault
    // setup partition
    // Create vp

    // create a partition file
    // create a cpu

}




