/*
 *
 * File handling the creation and management
 * of partition object
 *
 */

use crate::hv_defs::*;
use crate::hv_api::*;

pub struct VirtualProcessor
{
    pub handle:     usize;
    pub index:      usize;
    pub partition:  PHandle;
}

// must impl:
// run vp
// translate addr
// read/write gva
// dump reg
// set reg
// 

