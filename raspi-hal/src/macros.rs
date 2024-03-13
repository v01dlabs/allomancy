macro_rules! parse_retval {
    ($retval:expr) => {{
        let retval = $retval;

        if retval == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(retval)
        }
    }};
}

// Initialize an array with a type that doesn't implement Copy
macro_rules! init_array {
    ($val:expr, $len:expr) => {{
        // Based on https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element

        use std::mem::{self, MaybeUninit};

        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut data: [MaybeUninit<_>; $len] = unsafe { MaybeUninit::uninit().assume_init() };

        // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
        // assignment instead of `ptr::write` does not cause the old
        // uninitialized value to be dropped. Also if there is a panic during
        // this loop, we have a memory leak, but there is no memory safety
        // issue.
        for elem in &mut data[..] {
            *elem = MaybeUninit::new($val);
        }

        // Everything is initialized. Transmute the array to the
        // initialized type.
        unsafe { mem::transmute::<_, [_; $len]>(data) }
    }};
}

/// Types for the peripheral singletons.
#[macro_export]
macro_rules! peripherals_definition {
    ($($(#[$cfg:meta])? $name:ident),*$(,)?) => {
        /// Types for the peripheral singletons.
        pub mod peripherals {
            $(
                $(#[$cfg])?
                #[allow(non_camel_case_types)]
                #[doc = concat!(stringify!($name), " peripheral")]
                pub struct $name { _private: () }

                $(#[$cfg])?
                impl $name {
                    /// Unsafely create an instance of this peripheral out of thin air.
                    ///
                    /// # Safety
                    ///
                    /// You must ensure that you're only using one instance of this type at a time.
                    #[inline]
                    pub unsafe fn steal() -> Self {
                        Self{ _private: ()}
                    }
                }

                $(#[$cfg])?
                $crate::impl_peripheral!($name);
            )*
        }
    };
}

/// Define the peripherals struct.
#[macro_export]
macro_rules! peripherals_struct {
    ($($(#[$cfg:meta])? $name:ident),*$(,)?) => {
        /// Struct containing all the peripheral singletons.
        ///
        /// To obtain the peripherals, you must initialize the HAL, by calling [`crate::init`].
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(
                #[doc = concat!(stringify!($name), " peripheral")]
                $(#[$cfg])?
                pub $name: peripherals::$name,
            )*
        }

        impl Peripherals {
            ///Returns all the peripherals *once*
            #[inline]
            pub(crate) fn take() -> Self {
                critical_section::with(Self::take_with_cs)
            }

            ///Returns all the peripherals *once*
            #[inline]
            pub(crate) fn take_with_cs(_cs: critical_section::CriticalSection) -> Self {
                #[no_mangle]
                static mut _EMBASSY_DEVICE_PERIPHERALS: bool = false;

                // safety: OK because we're inside a CS.
                unsafe {
                    if _EMBASSY_DEVICE_PERIPHERALS {
                        panic!("init called more than once!")
                    }
                    _EMBASSY_DEVICE_PERIPHERALS = true;
                    Self::steal()
                }
            }
        }

        impl Peripherals {
            /// Unsafely create an instance of this peripheral out of thin air.
            ///
            /// # Safety
            ///
            /// You must ensure that you're only using one instance of this type at a time.
            #[inline]
            pub unsafe fn steal() -> Self {
                Self {
                    $(
                        $(#[$cfg])?
                        $name: peripherals::$name::steal(),
                    )*
                }
            }
        }
    };
}

/// Defining peripheral type.
#[macro_export]
macro_rules! peripherals {
    ($($(#[$cfg:meta])? $name:ident),*$(,)?) => {
        $crate::peripherals_definition!(
            $(
                $(#[$cfg])?
                $name,
            )*
        );
        $crate::peripherals_struct!(
            $(
                $(#[$cfg])?
                $name,
            )*
        );
    };
}

/// Convenience converting into reference.
#[macro_export]
macro_rules! into_ref {
    ($($name:ident),*) => {
        $(
            let mut $name = $name.into_ref();
        )*
    }
}

/// Implement the peripheral trait.
#[macro_export]
macro_rules! impl_peripheral {
    ($type:ident) => {
        impl $crate::Peripheral for $type {
            type P = $type;

            #[inline]
            unsafe fn clone_unchecked(&self) -> Self::P {
                #[allow(clippy::needless_update)]
                $type { ..*self }
            }
        }
    };
}
