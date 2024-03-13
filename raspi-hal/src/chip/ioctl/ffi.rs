// Copyright (c) 2018 The rust-gpio-cdev Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::chip::ioctl::IoctlKind;
use crate::chip::ioctl::errors;


pub const GPIOHANDLES_MAX: usize = 64;
pub const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;
pub const GPIO_MAX_NAME_SIZE: usize = 32;

// struct gpiochip_info
#[repr(C)]
pub struct gpiochip_info {
    pub name: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub label: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub lines: u32,
}

#[repr(C)]
pub struct gpioline_info {
    pub line_offset: u32,
    pub flags: u32,
    pub name: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub consumer: [libc::c_char; GPIO_MAX_NAME_SIZE],
}

#[repr(C)]
pub struct gpiohandle_request {
    pub lineoffsets: [u32; GPIOHANDLES_MAX],
    pub flags: u32,
    pub default_values: [u8; GPIOHANDLES_MAX],
    pub consumer_label: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub lines: u32,
    pub fd: libc::c_int,
}

#[repr(C)]
pub struct gpio_v2_line_request {
    pub lineoffsets: [u32; GPIOHANDLES_MAX],
    pub consumer: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub config: gpio_v2_line_config,
    pub num_lines: u32,
    pub event_buffer_size: u32,
    /* Pad to fill implicit padding and reserve space for future use. */
    pub padding: [u32; 5],
    pub fd: libc::c_int,
}

#[repr(C)]
pub struct gpio_v2_line_config {
    pub flags: u64,
    pub num_attrs: u32,
    pub padding: [u32; 5],
    pub attrs: [gpio_v2_line_config_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],

}

#[repr(C)]
pub enum gpio_v2_line_attr_id {
	GPIO_V2_LINE_ATTR_ID_FLAGS		= 1,
	GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES	= 2,
	GPIO_V2_LINE_ATTR_ID_DEBOUNCE		= 3,
}


/**
 * struct gpio_v2_line_attribute - a configurable attribute of a line
 * @id: attribute identifier with value from &enum gpio_v2_line_attr_id
 * @padding: reserved for future use and must be zero filled
 * @flags: if id is %GPIO_V2_LINE_ATTR_ID_FLAGS, the flags for the GPIO
 * line, with values from &enum gpio_v2_line_flag, such as
 * %GPIO_V2_LINE_FLAG_ACTIVE_LOW, %GPIO_V2_LINE_FLAG_OUTPUT etc, added
 * together.  This overrides the default flags contained in the &struct
 * gpio_v2_line_config for the associated line.
 * @values: if id is %GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES, a bitmap
 * containing the values to which the lines will be set, with each bit
 * number corresponding to the index into &struct
 * gpio_v2_line_request.offsets.
 * @debounce_period_us: if id is %GPIO_V2_LINE_ATTR_ID_DEBOUNCE, the
 * desired debounce period, in microseconds
 * C Struct defition:
 * struct gpio_v2_line_attribute {
 *      __u32 id;
 *      __u32 padding;
 * 	    union {
 * 		    __aligned_u64 flags;
 * 		    __aligned_u64 values;
 * 		    __u32 debounce_period_us;
 * 	    };
 * };
 */
#[repr(C)]
pub struct gpio_v2_line_config_attribute {
    /// the configurable attribute
	pub attr: gpio_v2_line_attribute,
    /// bitmap identifying the lines to which the attribute applies
    pub mask: u64, 
}


#[repr(C)]
pub struct gpio_v2_line_attribute {
	pub id: u32,
	pub padding: u32,
	pub union_flags_values_debounce: u64,
}

#[repr(C)]
pub struct gpio_v2_line_info {
	pub name: [libc::c_char; GPIO_MAX_NAME_SIZE],
	pub consumer: [libc::c_char; GPIO_MAX_NAME_SIZE],
	pub line_offset: u32,
	pub num_attrs: u32,
	pub flags: u64,
	pub attrs: [gpio_v2_line_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
	/* Space reserved for future use. */
	pub padding: [u32;4],
}

#[repr(C)]
pub struct gpiohandle_data {
    pub values: [u8; GPIOHANDLES_MAX],
}

#[repr(C)]
pub struct  gpio_v2_line_values {
    pub bits: u64,
    pub mask: u64,
}

#[repr(C)]
pub struct gpioevent_request {
    pub lineoffset: u32,
    pub handleflags: u32,
    pub eventflags: u32,
    pub consumer_label: [libc::c_char; GPIO_MAX_NAME_SIZE],
    pub fd: libc::c_int,
}

#[repr(C)]
pub struct gpioevent_data {
    pub timestamp: u64,
    pub id: u32,
}

#[repr(C)]
pub struct gpio_v2_line_event {
    pub timestamp: u64,
    pub id: u32,
    pub offset: u32,
    pub seqno: u32,
    pub line_seqno: u32,
    /* Space reserved for future use. */
    pub padding: [u32; 6],
}

#[repr(C)]
pub struct gpio_v2_line_info_changed {
    pub info: gpio_v2_line_info,
    pub timestamp: u64,
    pub event_type: u32,
    /* Pad struct to 64-bit boundary and reserve space for future use. */
    pub padding: [u32; 5],
}

macro_rules! wrap_ioctl {
    ($ioctl_macro:ident!($name:ident, $ioty:expr, $nr:expr, $ty:ident), $ioctl_error_type:expr) => {
        mod $name {
            $ioctl_macro!($name, $ioty, $nr, super::$ty);
        }

        pub(crate) fn $name(fd: libc::c_int, data: &mut $ty) -> errors::Result<libc::c_int> {
            unsafe {
                $name::$name(fd, data).map_err(|e| errors::ioctl_err($ioctl_error_type, e))
            }
        }
    };
}

wrap_ioctl!(
    ioctl_read!(gpio_get_chipinfo_ioctl, 0xB4, 0x01, gpiochip_info),
    IoctlKind::ChipInfo
);
wrap_ioctl!(
    ioctl_readwrite!(gpio_get_lineinfo_ioctl, 0xB4, 0x02, gpioline_info),
    IoctlKind::LineInfo
);
wrap_ioctl!(
    ioctl_readwrite!(gpio_get_linehandle_ioctl, 0xB4, 0x03, gpiohandle_request),
    IoctlKind::LineHandle
);
wrap_ioctl!(
    ioctl_readwrite!(gpio_get_lineevent_ioctl, 0xB4, 0x04, gpioevent_request),
    IoctlKind::LineEvent
);

wrap_ioctl!(
    ioctl_readwrite!(
        gpiohandle_get_line_values_ioctl,
        0xB4,
        0x08,
        gpiohandle_data
    ),
    IoctlKind::GetLine
);
wrap_ioctl!(
    ioctl_readwrite!(
        gpiohandle_set_line_values_ioctl,
        0xB4,
        0x09,
        gpiohandle_data
    ),
    IoctlKind::SetLine
);

// V2 ABI IOCTLs
wrap_ioctl!(
    ioctl_readwrite!(gpio_v2_get_lineinfo_ioctl, 0xB4, 0x05, gpio_v2_line_info),
    IoctlKind::LineInfoV2
);
wrap_ioctl!(
    ioctl_readwrite!(gpio_v2_get_line_ioctl, 0xB4, 0x07, gpio_v2_line_request),
    IoctlKind::LineV2
);

wrap_ioctl!(
    ioctl_readwrite!(gpio_v2_line_set_config_ioctl, 0xB4, 0x0D, gpio_v2_line_config),
    IoctlKind::LineConfigV2
);

//wrap_ioctl!(
//    ioctl_readwrite!(gpio_lineinfo_unwatch_ioctl, 0xB4, 0x0C, u32),
//    IoctlKind::LineInfoUnwatch
//);
wrap_ioctl!(
    ioctl_readwrite!(gpio_v2_lineinfo_watch_ioctl, 0xB4, 0x06, gpio_v2_line_info),
    IoctlKind::LineInfoWatch
);

wrap_ioctl!(
    ioctl_readwrite!(
        gpio_v2_get_line_get_values_ioctl,
        0xB4,
        0x0E,
        gpio_v2_line_values
    ),
    IoctlKind::GetLineV2
);
wrap_ioctl!(
    ioctl_readwrite!(
        gpio_v2_line_get_values_ioctl,
        0xB4,
        0x0F,
        gpio_v2_line_values
    ),
    IoctlKind::SetLineV2
);
