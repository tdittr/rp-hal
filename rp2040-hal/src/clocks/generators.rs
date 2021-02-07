use super::xosc::{Stable, XOsc};
use private::Clocks;

pub struct Splitted {
    // pub clk_gpout0: ClkGpOut0,
    // pub clk_gpout1: ClkGpOut1,
    // pub clk_gpout2: ClkGpOut2,
    // pub clk_gpout3: ClkGpOut3,
    pub clk_ref: ClkRef,
    pub clk_sys: ClkSys,
    // pub clk_peri: ClkPeri,
    // pub clk_usb: ClkUsb,
    // pub clk_adc: ClkAdc,
    // pub clk_rtc: ClkRtc,
    // pub freq_cnt: FrequencyCounter,
    // pub resus: Resus,
    _private: (),
}

impl Splitted {
    pub fn split(p: pac::CLOCKS) -> Splitted {
        Splitted {
            clk_ref: ClkRef {
                inner: Clocks::new(&p),
            },
            clk_sys: ClkSys {
                inner: Clocks::new(&p),
            },
            _private: (),
        }
    }
}

pub struct ClkRef {
    inner: Clocks,
}

pub struct ClkSys {
    inner: Clocks,
}

impl ClkSys {
    fn switch_to_ref(&mut self) {
        self.inner.clk_sys_ctrl.write(|w| w.src().clk_ref())
    }

    fn switch_to_aux(&mut self) {
        self.inner
            .clk_sys_ctrl
            .write(|w| w.src().clksrc_clk_sys_aux())
    }
}

impl<R> SourceFrom<XOsc<Stable, R>> for ClkSys {
    fn source(&mut self, _src: XOsc<Stable, R>) {
        self.switch_to_ref();
        self.inner.clk_sys_ctrl.write(|w| w.auxsrc().xosc_clksrc());
        self.switch_to_aux();
    }
}

pub trait SourceFrom<From> {
    fn source(&mut self, src: From);
}

// We split up the clocks peripheral, such that there is only ever one part that can access any register.
mod private {
    use core::ops::Deref;

    pub struct Clocks {
        _private: (),
    }

    impl Clocks {
        pub fn new(_: &pac::CLOCKS) -> Self {
            Clocks { _private: () }
        }
    }

    impl Deref for Clocks {
        type Target = pac::clocks::RegisterBlock;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*pac::CLOCKS::ptr() }
        }
    }
}
