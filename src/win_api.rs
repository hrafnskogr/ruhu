//
// Winapi stuff
//

pub type HResult = u32;

pub trait HRPrint
{
    fn pprint(&self) -> &str;
    fn raw(&self) -> String;
}

impl HRPrint for HResult
{
    fn pprint(&self) -> &str
    {
        match self
        {
            0x00000000 => "S_OK",
            0x00000001 => "S_FALSE",
            0x80004004 => "E_ABORT",
            0x80004005 => "E_FAIL",
            0x80004002 => "E_NOINTERFACE",
            0x80004001 => "E_NOTIMPL",
            0x80004003 => "E_POINTER",
            0x8000FFFF => "E_UNEXPECTED",
            0x80070005 => "E_ACCESSDENIED",
            0x80070006 => "E_HANDLE",
            0x80070057 => "E_INVALIDARG",
            0x8007000E => "E_OUTOFMEMORY",
            _ => "UNDEFINED_ERROR",
        }
    }

    fn raw(&self) -> String
    {
        format!("{:x}", self)
    }
}

extern "system"
{
    fn GetLastError() -> u32;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum HResult_Code
{
    S_OK            = 0x00000000,
    S_FALSE         = 0x00000001,
    E_ABORT         = 0x80004004,
    E_FAIL          = 0x80004005,
    E_NOINTERFACE   = 0x80004002,
    E_NOTIMPL       = 0x80004001,
    E_POINTER       = 0x80004003,
    E_UNEXPECTED    = 0x8000FFFF,
    E_ACCESSDENIED  = 0x80070005,
    E_HANDLE        = 0x80070006,
    E_INVALIDARG    = 0x80070057,
    E_OUTOFMEMORY   = 0x8007000E,
}

pub fn print_win_err()
{
    unsafe { println!("Win Error Code: {}", GetLastError()); }
}

