/*
 *
 * File handling the creation and management
 * of partition object
 *
 */

use crate::hv_defs::*;
use crate::hv_api::*;

pub struct Partition
{
    pub name: String,
    pub handle: PHandle,
}

impl Partition
{
    fn new(name: String) -> Partition
    {
        let handle: PHandle = create_partition();
        Partition { name, handle }
    }

    // TODO : A voir pour le Result
    fn set_property(self: &Self, property_code: WHvPartitionPropertyCode) 
    {
         
    }

}



