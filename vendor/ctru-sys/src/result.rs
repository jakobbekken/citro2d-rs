//! Ports of macros in libctru's result.h.

use crate::Result;

pub fn R_SUCCEEDED(res: Result) -> bool {
    res >= 0
}

pub fn R_FAILED(res: Result) -> bool {
    res < 0
}

pub fn R_LEVEL(res: Result) -> libc::c_uchar {
    ((res >> 27) & 0x1F) as _
}

pub fn R_SUMMARY(res: Result) -> libc::c_uchar {
    ((res >> 21) & 0x3F) as _
}

pub fn R_MODULE(res: Result) -> libc::c_uchar {
    ((res >> 10) & 0xFF) as _
}

pub fn R_DESCRIPTION(res: Result) -> libc::c_ushort {
    (res & 0x3FF) as _
}

pub fn MAKERESULT(level: Result, summary: Result, module: Result, description: Result) -> Result {
    ((level & 0x1F) << 27)
        | ((summary & 0x3F) << 21)
        | ((module & 0xFF) << 10)
        | (description & 0x3FF)
}
