//! Serial Peripheral Interface (SPI)
use core::marker::PhantomData;

use crate::ccu::{Clocks, Gating, Reset};
use crate::gpio::{
    portc::{PC2, PC3, PC4, PC5},
    Function,
};
use d1_pac::{
    spi0::{
        spi_gcr::{EN_A, MODE_A, TP_EN_A},
        spi_tcr::{CPHA_A, CPOL_A, SPOL_A, SS_OWNER_A, SS_SEL_A},
        RegisterBlock,
    },
    CCU, SPI0,
};

/// D1 SPI peripheral
pub struct Spi<SPI: Instance, PINS> {
    inner: SPI,
    pins: PINS,
    _stub: Stub<SPI>,
}

/// Allows free for Spi safely.
struct Stub<SPI: Instance>(PhantomData<SPI>);

impl<SPI: Instance, PINS> Spi<SPI, PINS> {
    /// Create instance of Spi
    #[inline]
    pub fn new(spi: SPI, pins: PINS, _clocks: &Clocks) -> Self
    where
        PINS: Pins<SPI>,
    {
        // see [xboot](https://github.com/xboot/xboot/blob/master/src/arch/riscv64/mach-d1/driver/spi-d1.c)
        // 1. unwrap parameters
        // todo
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        let ccu = unsafe { &*CCU::ptr() };
        // 配置时钟源和分频
        #[rustfmt::skip]
        ccu.spi0_clk.write(|w| w
            .clk_src_sel().pll_peri_1x()
            .factor_n()   .n1()
            .factor_m()   .variant(6 - 1)
            .clk_gating() .set_bit()
        );
        // 断开接地，连接时钟
        #[rustfmt::skip]
        ccu.spi_bgr.write(|w| w
            .spi0_rst()   .deassert()
            .spi0_gating().set_bit()
        );
        // 3. 软重置，清空 FIFO
        #[rustfmt::skip]
        spi.spi_gcr.write(|w| w
            .srst() .variant(true)
            .tp_en().variant(TP_EN_A::STOP_WHEN_FULL)
            .mode() .variant(MODE_A::MASTER)
            .en()   .variant(EN_A::ENABLE)
        );
        // wait soft reset complete (gcr.srst)
        while spi.spi_gcr.read().srst().bit_is_set() {
            core::hint::spin_loop();
        }
        #[rustfmt::skip]
        spi.spi_fcr.write(|w| w
            .tf_rst().set_bit()
            .rf_rst().set_bit()
        );
        // wait fifo reset complete (fcr.tf_rst|fcr.rf_rst)
        loop {
            let fcr = spi.spi_fcr.read();
            if fcr.tf_rst().bit_is_clear() && fcr.rf_rst().bit_is_clear() {
                break;
            } else {
                core::hint::spin_loop();
            }
        }
        // 4. 配置工作模式
        #[rustfmt::skip]
        spi.spi_tcr.write(|w| w
            .ss_owner().variant(SS_OWNER_A::SPI_CONTROLLER)
            .ss_sel()  .variant(SS_SEL_A::SS0)
            .spol()    .variant(SPOL_A::LOW)
            .cpol()    .variant(CPOL_A::LOW)
            .cpha()    .variant(CPHA_A::P1)
        );
        Spi {
            inner: spi,
            pins,
            _stub: Stub(PhantomData),
        }
    }

    /// 收发
    #[inline]
    pub fn transfer(&self, mosi: impl AsRef<[u8]>, dummy: usize, mut miso: impl AsMut<[u8]>) {
        let spi = &self.inner;
        let x = mosi.as_ref();
        let r = miso.as_mut();

        let lx = x.len() as u32;
        let ld = dummy as u32;
        let lr = r.len() as u32;

        #[rustfmt::skip]
        { // 传输配置
        spi.spi_mbc.write(|w| w.mbc ().variant(lx + ld + lr));
        spi.spi_mtc.write(|w| w.mwtc().variant(lx));
        spi.spi_bcc.write(|w| w.stc ().variant(lx)
                                       .dbc ().variant(ld as _));
        spi.spi_tcr.modify(|r, w| unsafe { w.bits(r.bits()) }.xch().set_bit());
        };
        // 发送
        for b in x {
            while spi.spi_fsr.read().tf_cnt().bits() >= 64 {
                core::hint::spin_loop();
            }
            spi.spi_txd_8().write(|w| unsafe { w.bits(*b) });
        }
        // 跳过不需要的输入
        for _ in 0..lx + ld {
            while spi.spi_fsr.read().rf_cnt().bits() == 0 {
                core::hint::spin_loop();
            }
            let _ = spi.spi_rxd_8().read();
        }
        // 接收
        for b in r {
            while spi.spi_fsr.read().rf_cnt().bits() == 0 {
                core::hint::spin_loop();
            }
            *b = spi.spi_rxd_8().read().bits();
        }
        // 确认传输已结束
        assert!(spi.spi_tcr.read().xch().bit_is_clear());
    }

    /// Close and release peripheral
    #[inline]
    pub fn free(self) -> (SPI, PINS) {
        let Self {
            inner,
            pins,
            _stub: _, // spi is closed via Drop trait of stub
        } = self;
        (inner, pins)
    }
}

// Disable peripheral when drop; next bootloading stage will initialize this again.
impl<SPI: Instance> Drop for Stub<SPI> {
    #[inline]
    fn drop(&mut self) {
        let ccu = unsafe { &*CCU::ptr() };
        SPI::assert_reset(ccu);
        SPI::gating_mask(ccu);
    }
}

pub trait Instance: Gating + Reset + core::ops::Deref<Target = RegisterBlock> {}

impl Instance for d1_pac::SPI0 {}

pub trait Pins<SPI> {}

// parameter order: sck, scs, miso, mosi

impl Pins<SPI0>
    for (
        PC2<Function<2>>,
        PC3<Function<2>>,
        PC4<Function<2>>,
        PC5<Function<2>>,
    )
{
}
