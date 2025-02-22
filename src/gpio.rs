use embedded_hal;

#[derive(Debug)]
pub enum GpioError {}

impl embedded_hal::digital::Error for GpioError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

#[macro_export]
macro_rules! gpio {
    ($(
        $GPIOX:ident: $PACGPIOX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $GPIOX {
                pub index: usize,
            }

            impl $GPIOX {
                pub fn new(index: usize) -> Self {
                    Self { index }
                }
            }

            impl $crate::hal::digital::ErrorType for $GPIOX {
                type Error = $crate::gpio::GpioError;
            }
            impl $crate::hal::digital::OutputPin for $GPIOX {

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = !(1 << self.index);
                    riscv::interrupt::machine::free(|| {
                        let val: u32 = reg.out().read().bits() & mask;
                        unsafe {
                            reg.out().write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    riscv::interrupt::machine::free(|| {
                        let val: u32 = reg.out().read().bits() | mask;
                        unsafe {
                            reg.out().write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
            }

            impl $crate::hal::digital::StatefulOutputPin for $GPIOX {
                fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out().read().bits() & mask;
                    Ok(val == 0)
                }
                fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out().read().bits() & mask;
                    Ok(val != 0)
                }
            }
        )+
    }
}
