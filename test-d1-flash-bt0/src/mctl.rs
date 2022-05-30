use core::ptr::{read_volatile, write_volatile};

const RAM_BASE: usize = 0x40000000;

// p49 ff
const CCU: usize = 0x0200_1000;
const PLL_CPU_CTRL: usize = CCU + 0x0000;
const PLL_DDR_CTRL: usize = CCU + 0x0010;
const MBUS_CLK: usize = CCU + 0x0540;
const DRAM_CLK: usize = CCU + 0x0800;
const DRAM_BGR: usize = CCU + 0x080c;

/**
 * D1 manual p152 3.4 System Configuration
 *
 * SYS_CFG Base Address 0x03000000
 *
 * | Register Name       | Offset | Description                              |
 * | ------------------- | ------ | ---------------------------------------- |
 * | DSP_BOOT_RAMMAP_REG | 0x0008 | DSP Boot SRAM Remap Control Register     |
 * | VER_REG             | 0x0024 | Version Register                         |
 * | EMAC_EPHY_CLK_REG0  | 0x0030 | EMAC-EPHY Clock Register 0               |
 * | SYS_LDO_CTRL_REG    | 0x0150 | System LDO Control Register              |
 * | RESCAL_CTRL_REG     | 0x0160 | Resistor Calibration Control Register    |
 * | RES240_CTRL_REG     | 0x0168 | 240ohms Resistor Manual Control Register |
 * | RESCAL_STATUS_REG   | 0x016C | Resistor Calibration Status Register     |
 */

const SYS_CFG: usize = 0x0300_0000; // 0x0300_0000 - 0x0300_0FFF
const VER_REG: usize = SYS_CFG + 0x0024;
const EMAC_EPHY_CLK_REG0: usize = SYS_CFG + 0x0030;
const SYS_LDO_CTRL_REG: usize = SYS_CFG + 0x0150;
const RES_CAL_CTRL_REG: usize = SYS_CFG + 0x0160;
const RES240_CTRL_REG: usize = SYS_CFG + 0x0168;
const RES_CAL_STATUS_REG: usize = SYS_CFG + 0x016c;

const ZQ_VALUE: usize = SYS_CFG + 0x0172;
const ZQ_INTERNAL: usize = SYS_CFG + 0x016e;

const FOO_BASE: usize = 0x0701_0000; // TODO: What do we call this?
const ANALOG_SYS_PWROFF_GATING_REG: usize = FOO_BASE + 0x0254;

const SID_INFO: usize = 0x3002228;

// p32 memory mapping
// MSI + MEMC: 0x0310_2000 - 0x0330_1fff
// NOTE: MSI shares the bus clock with CE, DMAC, IOMMU and CPU_SYS; p 38
// TODO: Define *_BASE?
const MSI_MEMC_BASE: usize = 0x0310_2000; // p32 0x0310_2000 - 0x0330_1FFF

// PHY config registers; TODO: fix names
const MC_WORK_MODE_RANK0_LOW: usize = MSI_MEMC_BASE;
const MC_WORK_MODE_RANK0_HIGH: usize = MSI_MEMC_BASE + 0x0004;

const UNKNOWN1: usize = MSI_MEMC_BASE + 0x0008; // 0x3102008
const UNKNOWN7: usize = MSI_MEMC_BASE + 0x000c; // 0x310200c

const DRAM_MASTER_CTL1: usize = MSI_MEMC_BASE + 0x0020;
const DRAM_MASTER_CTL2: usize = MSI_MEMC_BASE + 0x0024;
const DRAM_MASTER_CTL3: usize = MSI_MEMC_BASE + 0x0028;

const UNKNOWN6: usize = MSI_MEMC_BASE + 0x0100; // 0x3102100

// TODO:
// 0x0310_2200
// 0x0310_2210
// 0x0310_2214
// 0x0310_2230
// 0x0310_2234
// 0x0310_2240
// 0x0310_2244
// 0x0310_2260
// 0x0310_2264
// 0x0310_2290
// 0x0310_2294
// 0x0310_2470
// 0x0310_2474
// 0x0310_31c0
// 0x0310_31c8
// 0x0310_31d0

// DATX0IOCR x + 4 * size
// DATX0IOCR - DATX3IOCR: 11 registers per block, blocks 0x20 words apart
const DATX0IOCR: usize = MSI_MEMC_BASE + 0x0310; // 0x3102310
const DATX3IOCR: usize = MSI_MEMC_BASE + 0x0510; // 0x3102510

const PHY_AC_MAP1: usize = 0x3102500;
const PHY_AC_MAP2: usize = 0x3102504;
const PHY_AC_MAP3: usize = 0x3102508;
const PHY_AC_MAP4: usize = 0x310250c;

const MCTL_CLK_EN: usize = MSI_MEMC_BASE + 0x100c; // 0x310300C
const UNKNOWN3: usize = MSI_MEMC_BASE + 0x1010; // 0x3103010

const DRAM_MR0: usize = MSI_MEMC_BASE + 0x1030; // 0x3103030
const DRAM_MR1: usize = MSI_MEMC_BASE + 0x1034; // 0x3103034
const DRAM_MR2: usize = MSI_MEMC_BASE + 0x1038; // 0x3103038
const DRAM_MR3: usize = MSI_MEMC_BASE + 0x103c; // 0x310303c
const DRAM_ODTX: usize = MSI_MEMC_BASE + 0x102c; // 0x310302c

const DRAMTMG0: usize = MSI_MEMC_BASE + 0x1058; // 0x3103058;
const DRAMTMG1: usize = MSI_MEMC_BASE + 0x105c; // 0x310305c;
const DRAMTMG2: usize = MSI_MEMC_BASE + 0x1060; // 0x3103060;
const DRAMTMG3: usize = MSI_MEMC_BASE + 0x1064; // 0x3103064;
const DRAMTMG4: usize = MSI_MEMC_BASE + 0x1068; // 0x3103068;
const DRAMTMG5: usize = MSI_MEMC_BASE + 0x106c; // 0x310306c;
const DRAMTMG6: usize = MSI_MEMC_BASE + 0x1070; // 0x3103070;
const DRAMTMG7: usize = MSI_MEMC_BASE + 0x1074; // 0x3103074;
const DRAMTMG8: usize = MSI_MEMC_BASE + 0x1078; // 0x3103078;
const PITMG0: usize = MSI_MEMC_BASE + 0x1080; // 0x3103080;
const PTR3: usize = MSI_MEMC_BASE + 0x1050; // 0x3103050;
const PTR4: usize = MSI_MEMC_BASE + 0x1054; // 0x3103054;
const RFSHTMG: usize = MSI_MEMC_BASE + 0x1090; // 0x3103090;
const RFSHCTL1: usize = MSI_MEMC_BASE + 0x1094; // 0x3103094;

const IOCVR_LOW: usize = MSI_MEMC_BASE + 0x1110; // 0x3103110
const IOCVR_HIGH: usize = MSI_MEMC_BASE + 0x1114; // 0x3103114
const UNKNOWN9: usize = MSI_MEMC_BASE + 0x1120; // 0x3103120
const UNKNOWN4: usize = MSI_MEMC_BASE + 0x1348; // 0x3103348
const UNKNOWN10: usize = MSI_MEMC_BASE + 0x13c4; // 0x31033c4;
const UNKNOWN5: usize = MSI_MEMC_BASE + 0x13c8; // 0x31033C8

// TODO: *_BASE ?
const MC_WORK_MODE_RANK1_LOW: usize = MSI_MEMC_BASE + 0x10_0000;
const MC_WORK_MODE_RANK1_HIGH: usize = MSI_MEMC_BASE + 0x10_0004;
#[repr(C)]
pub struct dram_parameters {
    pub dram_clk: u32,
    pub dram_type: u32,
    pub dram_zq: u32,
    pub dram_odt_en: u32,
    pub dram_para1: u32,
    pub dram_para2: u32,
    pub dram_mr0: u32,
    pub dram_mr1: u32,
    pub dram_mr2: u32,
    pub dram_mr3: u32,
    pub dram_tpr0: u32,
    pub dram_tpr1: u32,
    pub dram_tpr2: u32,
    pub dram_tpr3: u32,
    pub dram_tpr4: u32,
    pub dram_tpr5: u32,
    pub dram_tpr6: u32,
    pub dram_tpr7: u32,
    pub dram_tpr8: u32,
    pub dram_tpr9: u32,
    pub dram_tpr10: u32,
    pub dram_tpr11: u32,
    pub dram_tpr12: u32,
    pub dram_tpr13: u32,
}

// taken from SPL
const DRAM_PARA: dram_parameters = dram_parameters {
    dram_clk: 0x00000318,
    dram_type: 0x00000003,
    dram_zq: 0x007b7bfb,
    dram_odt_en: 0x00000001,
    dram_para1: 0x000010d2,
    dram_para2: 0x00000000,
    dram_mr0: 0x00001c70,
    dram_mr1: 0x00000042,
    dram_mr2: 0x00000018,
    dram_mr3: 0x00000000,
    dram_tpr0: 0x004a2195,
    dram_tpr1: 0x02423190,
    dram_tpr2: 0x0008b061,
    dram_tpr3: 0xb4787896,
    dram_tpr4: 0x00000000,
    dram_tpr5: 0x48484848,
    dram_tpr6: 0x00000048,
    dram_tpr7: 0x1620121e,
    dram_tpr8: 0x00000000,
    dram_tpr9: 0x00000000,
    dram_tpr10: 0x00000000,
    dram_tpr11: 0x00870000,
    dram_tpr12: 0x00000024,
    dram_tpr13: 0x34050100,
};

fn readl(reg: usize) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}

fn writel(reg: usize, val: u32) {
    unsafe {
        write_volatile(reg as *mut u32, val);
    }
}

fn get_pmu_exists() -> bool {
    return false;
}

fn memcpy_self(dst: &mut [u32; 22], src: &mut [u32; 22], len: usize) {
    for i in 0..len {
        dst[i] = src[i];
    }
}

static mut PHY_CFG0: [u32; 22] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static mut PHY_CFG1: [u32; 22] = [
    1, 9, 3, 7, 8, 18, 4, 13, 5, 6, 10, 2, 14, 12, 0, 0, 21, 17, 20, 19, 11, 22,
];
static mut PHY_CFG2: [u32; 22] = [
    4, 9, 3, 7, 8, 18, 1, 13, 2, 6, 10, 5, 14, 12, 0, 0, 21, 17, 20, 19, 11, 22,
];
static mut PHY_CFG3: [u32; 22] = [
    1, 7, 8, 12, 10, 18, 4, 13, 5, 6, 3, 2, 9, 0, 0, 0, 21, 17, 20, 19, 11, 22,
];
static mut PHY_CFG4: [u32; 22] = [
    4, 12, 10, 7, 8, 18, 1, 13, 2, 6, 3, 5, 9, 0, 0, 0, 21, 17, 20, 19, 11, 22,
];
static mut PHY_CFG5: [u32; 22] = [
    13, 2, 7, 9, 12, 19, 5, 1, 6, 3, 4, 8, 10, 0, 0, 0, 21, 22, 18, 17, 11, 20,
];
static mut PHY_CFG6: [u32; 22] = [
    3, 10, 7, 13, 9, 11, 1, 2, 4, 6, 8, 5, 12, 0, 0, 0, 20, 1, 0, 21, 22, 17,
];
static mut PHY_CFG7: [u32; 22] = [
    3, 2, 4, 7, 9, 1, 17, 12, 18, 14, 13, 8, 15, 6, 10, 5, 19, 22, 16, 21, 20, 11,
];

// TODO: verify
// This routine seems to have several remapping tables for 22 lines.
// It is unclear which lines are being remapped. It seems to pick
// table PHY_CFG7 for the Nezha board.
unsafe fn mctl_phy_ac_remapping(para: &mut dram_parameters) {
    // read SID info @ 0x228
    let fuse = (readl(SID_INFO) >> 8) & 0x4;
    println!("ddr_efuse_type: 0x{:x}", fuse);
    if (para.dram_tpr13 >> 18) & 0x3 > 0 {
        memcpy_self(&mut PHY_CFG0, &mut PHY_CFG7, 22);
    } else {
        match fuse {
            8 => memcpy_self(&mut PHY_CFG0, &mut PHY_CFG2, 22),
            9 => memcpy_self(&mut PHY_CFG0, &mut PHY_CFG3, 22),
            10 => memcpy_self(&mut PHY_CFG0, &mut PHY_CFG5, 22),
            11 => memcpy_self(&mut PHY_CFG0, &mut PHY_CFG4, 22),
            13 | 14 => {}
            12 | _ => memcpy_self(&mut PHY_CFG0, &mut PHY_CFG1, 22),
        }
    }

    if para.dram_type == 2 {
        if fuse == 15 {
            return;
        }
        memcpy_self(&mut PHY_CFG0, &mut PHY_CFG6, 22);
    }

    if para.dram_type == 2 || para.dram_type == 3 {
        let val = (PHY_CFG0[4] << 25)
            | (PHY_CFG0[3] << 20)
            | (PHY_CFG0[2] << 15)
            | (PHY_CFG0[1] << 10)
            | (PHY_CFG0[0] << 5);
        writel(PHY_AC_MAP1, val as u32);

        let val = (PHY_CFG0[10] << 25)
            | (PHY_CFG0[9] << 20)
            | (PHY_CFG0[8] << 15)
            | (PHY_CFG0[7] << 10)
            | (PHY_CFG0[6] << 5)
            | PHY_CFG0[5];
        writel(PHY_AC_MAP2, val as u32);

        let val = (PHY_CFG0[15] << 20)
            | (PHY_CFG0[14] << 15)
            | (PHY_CFG0[13] << 10)
            | (PHY_CFG0[12] << 5)
            | PHY_CFG0[11];
        writel(PHY_AC_MAP3, val as u32);

        let val = (PHY_CFG0[21] << 25)
            | (PHY_CFG0[20] << 20)
            | (PHY_CFG0[19] << 15)
            | (PHY_CFG0[18] << 10)
            | (PHY_CFG0[17] << 5)
            | PHY_CFG0[16];
        writel(PHY_AC_MAP4, val as u32);

        let val = (PHY_CFG0[4] << 25)
            | (PHY_CFG0[3] << 20)
            | (PHY_CFG0[2] << 15)
            | (PHY_CFG0[1] << 10)
            | (PHY_CFG0[0] << 5)
            | 1;
        writel(PHY_AC_MAP1, val as u32);
    }
}

unsafe fn dram_vol_set(dram_para: &mut dram_parameters) {
    let vol = match dram_para.dram_type {
        2 => 47, // 1.8V
        3 => 25, // 1.5V
        _ => 0,
    };
    let vol = 25; // XXX
    let mut reg = read_volatile(SYS_LDO_CTRL_REG as *mut u32);
    reg &= !(0xff00);
    reg |= vol << 8;
    reg &= !(0x200000);
    write_volatile(SYS_LDO_CTRL_REG as *mut u32, reg);
    // TODO
    // sdelay(1);
    for _ in 0..1_000 {
        core::arch::asm!("nop")
    }
}

fn set_ddr_voltage(val: usize) -> usize {
    val
}

unsafe fn dram_enable_all_master() {
    write_volatile(DRAM_MASTER_CTL1 as *mut u32, 0xffffffff);
    write_volatile(DRAM_MASTER_CTL2 as *mut u32, 0xff);
    write_volatile(DRAM_MASTER_CTL3 as *mut u32, 0xffff);
    // sdelay(10); // TODO
    for _ in 0..10_000 {
        core::arch::asm!("nop")
    }
}

unsafe fn dram_disable_all_master() {
    write_volatile(DRAM_MASTER_CTL1 as *mut u32, 1);
    write_volatile(DRAM_MASTER_CTL2 as *mut u32, 0);
    write_volatile(DRAM_MASTER_CTL3 as *mut u32, 0);
    // sdelay(10); // TODO
    for _ in 0..10_000 {
        core::arch::asm!("nop")
    }
}

// Purpose of this routine seems to be to initialize the PLL driving
// the MBUS and sdram.
unsafe fn ccm_set_pll_ddr_clk(para: &mut dram_parameters) -> u32 {
    // FIXME: This is a bit weird, especially the scaling down and up etc
    let clk = match para.dram_tpr13 & (1 << 6) {
        0 => para.dram_clk,
        _ => para.dram_tpr9,
    };
    let n = 66; // (clk * 2) / 24; // FIXME: Why is this always 0?
    println!("clk {} / n {}", clk, n);

    // set VCO clock divider
    let mut val = read_volatile(PLL_DDR_CTRL as *mut u32);
    val &= 0xfff800fc; // clear dividers
    val |= (n - 1) << 8; // set PLL division
    val |= 0xc0000000; // enable PLL and LDO
    write_volatile(PLL_DDR_CTRL as *mut u32, val);

    // Restart PLL locking
    val &= 0xdfffffff; // disbable lock
    val |= 0xc0000000; // enable PLL and LDO
    write_volatile(PLL_DDR_CTRL as *mut u32, val);
    val |= 0xe0000000; // re-enable lock
    write_volatile(PLL_DDR_CTRL as *mut u32, val);

    // wait for PLL to lock
    while read_volatile(PLL_DDR_CTRL as *mut u32) == 0 {}

    // sdelay(20); // TODO
    for _ in 0..20_000 {
        core::arch::asm!("nop")
    }

    // enable PLL output
    let val = read_volatile(PLL_CPU_CTRL as *mut u32);
    write_volatile(PLL_CPU_CTRL as *mut u32, val | 0x08000000);

    // turn clock gate on
    let mut val = read_volatile(DRAM_CLK as *mut u32);
    val &= 0xfcfffcfc; // select DDR clk source, n=1, m=1
    val |= 0x80000000; // turn clock on
    write_volatile(DRAM_CLK as *mut u32, val);

    return n * 24;
}

// TODO: verify this
unsafe fn mctl_sys_init(para: &mut dram_parameters) {
    // TODO: What is s1 for?
    // s1 = 0x02001000

    // assert MBUS reset
    let val = read_volatile(MBUS_CLK as *mut u32);
    write_volatile(MBUS_CLK as *mut u32, val & 0xbfffffff);

    // turn off sdram clock gate, assert sdram reset
    let mut val = read_volatile(DRAM_BGR as *mut u32);
    val &= 0xfffffffe;
    write_volatile(DRAM_BGR as *mut u32, val);
    val &= 0xfffefffe;
    write_volatile(DRAM_BGR as *mut u32, val);

    // turn off bit 30 [??]
    let mut val = read_volatile(DRAM_CLK as *mut u32);
    write_volatile(DRAM_CLK as *mut u32, val & 0xbfffffff);
    // and toggle dram clock gating off + trigger update
    val &= 0x7fffffff;
    write_volatile(DRAM_CLK as *mut u32, val);
    val |= 0x08000000;
    write_volatile(DRAM_CLK as *mut u32, val);
    // sdelay(10); // TODO
    for _ in 0..10_000 {
        core::arch::asm!("nop")
    }

    println!("ccm_set_pll_ddr_clk");
    // set ddr pll clock
    // NOTE: This passes an additional `0` in the original, but it's unused
    let val = ccm_set_pll_ddr_clk(para);
    para.dram_clk = val >> 1;
    println!("ddr_clk {}", val >> 1);
    // sdelay(100); // TODO
    for _ in 0..100_000 {
        core::arch::asm!("nop")
    }
    println!("disable_all_master");
    dram_disable_all_master();

    println!("SDRAM reset");
    // release sdram reset
    let val = read_volatile(DRAM_BGR as *mut u32);
    write_volatile(DRAM_BGR as *mut u32, val | 0x00010000);

    // release MBUS reset
    let val = read_volatile(MBUS_CLK as *mut u32);
    write_volatile(MBUS_CLK as *mut u32, val | 0x40000000);

    // turn bit 30 back on [?]
    let val = read_volatile(DRAM_CLK as *mut u32);
    write_volatile(DRAM_CLK as *mut u32, val | 0x40000000);

    println!("SDRAM clock gate ON");
    // turn on sdram clock gate
    let val = read_volatile(DRAM_BGR as *mut u32);
    write_volatile(DRAM_BGR as *mut u32, val | 0x0000001); // (1<<0);

    // turn dram clock gate on, trigger sdr clock update
    let mut val = read_volatile(DRAM_CLK as *mut u32);
    val |= 0x80000000;
    write_volatile(DRAM_CLK as *mut u32, val);
    val |= 0x88000000;
    write_volatile(DRAM_CLK as *mut u32, val);
    // sdelay(5); // TODO
    for _ in 0..5_000 {
        core::arch::asm!("nop")
    }

    // mCTL clock enable
    write_volatile(MCTL_CLK_EN as *mut u32, 0x00008000);
    // sdelay(10); // TODO
    for _ in 0..10_000 {
        core::arch::asm!("nop")
    }
}

// Set the Vref mode for the controller
unsafe fn mctl_vrefzq_init(para: &mut dram_parameters) {
    if (para.dram_tpr13 & (1 << 17)) == 0 {
        let val = read_volatile(IOCVR_LOW as *mut u32) & 0x80808080; // IOCVR0
        write_volatile(IOCVR_LOW as *mut u32, val | para.dram_tpr5 as u32);

        if (para.dram_tpr13 & (1 << 16)) == 0 {
            let val = read_volatile(IOCVR_HIGH as *mut u32) & 0xffffff80; // IOCVR1
            write_volatile(IOCVR_HIGH as *mut u32, val | para.dram_tpr6 as u32 & 0x7f);
        }
    }
}

// The main purpose of this routine seems to be to copy an address configuration
// from the dram_para1 and dram_para2 fields to the PHY configuration registers
// (0x3102000, 0x3102004).
fn mctl_com_init(para: &mut dram_parameters) {
    // purpose ??
    let mut val = readl(UNKNOWN1) & 0xffffc0ff;
    val |= 0x2000;
    writel(UNKNOWN1, val);

    // Set sdram type and word width
    let mut val = readl(MC_WORK_MODE_RANK0_LOW) & 0xff000fff;
    val |= (para.dram_type & 0x7) << 16; // DRAM type
    val |= (!para.dram_para2 & 0x1) << 12; // DQ width
    if para.dram_type != 6 && para.dram_type != 7 {
        val |= ((para.dram_tpr13 >> 5) & 0x1) << 19; // 2T or 1T
        val |= 0x400000;
    } else {
        val |= 0x480000; // type 6 and 7 must use 1T
    }
    writel(MC_WORK_MODE_RANK0_LOW, val);

    // init rank / bank / row for single/dual or two different ranks
    let val = para.dram_para2;
    // ((val & 0x100) && (((val >> 12) & 0xf) != 1)) ? 32 : 16;
    let rank = if (val & 0x100) != 0 {
        match (val >> 12) & 0xf {
            1 => 1,
            _ => 2,
        }
    } else {
        16
    };

    for i in 0..rank {
        let ptr = MC_WORK_MODE_RANK0_LOW + i * 4;
        let mut val = readl(ptr) & 0xfffff000;

        val |= (para.dram_para2 >> 12) & 0x3; // rank
        val |= ((para.dram_para1 >> (i * 16 + 12)) << 2) & 0x4; // bank - 2
        val |= (((para.dram_para1 >> (i * 16 + 4)) - 1) << 4) & 0xff; // row - 1

        // convert from page size to column addr width - 3
        val |= match (para.dram_para1 >> i * 16) & 0xf {
            8 => 0xa00,
            4 => 0x900,
            2 => 0x800,
            1 => 0x700,
            _ => 0x600,
        };
        writel(ptr, val);
    }

    // set ODTMAP based on number of ranks in use
    let val = match readl(MC_WORK_MODE_RANK0_LOW) & 0x1 {
        0 => 0x201,
        _ => 0x303,
    };
    writel(UNKNOWN9, val);

    // set mctl reg 3c4 to zero when using half DQ
    if para.dram_para2 & (1 << 0) > 0 {
        writel(UNKNOWN10, 0);
    }

    // purpose ??
    if para.dram_tpr4 > 0 {
        let mut val = readl(MC_WORK_MODE_RANK0_LOW);
        val |= (para.dram_tpr4 << 25) & 0x06000000;
        writel(MC_WORK_MODE_RANK0_LOW, val);

        let mut val = readl(MC_WORK_MODE_RANK0_HIGH);
        val |= ((para.dram_tpr4 >> 2) << 12) & 0x001ff000;
        writel(MC_WORK_MODE_RANK0_HIGH, val);
    }
}

// TODO: Check that division works properly!
fn auto_cal_timing(time: u32, freq: u32) -> u32 {
    let t = time * freq;
    let what = if (t % 1000) != 0 { 1 } else { 0 };
    (t / 1000) + what
}

// Main purpose of the auto_set_timing routine seems to be to calculate all
// timing settings for the specific type of sdram used. Read together with
// an sdram datasheet for context on the various variables.
fn auto_set_timing_para(para: &mut dram_parameters) {
    let dfreq = para.dram_clk;
    let dtype = para.dram_type;
    let tpr13 = para.dram_tpr13;

    //println!("type  = {}\n", dtype);
    //println!("tpr13 = {}\n", tpr13);

    // FIXME: Half of this is unused, wat?!
    let mut tccd: u32 = 0; // 88(sp)
    let mut trrd: u32 = 0; // s7
    let mut trcd: u32 = 0; // s3
    let mut trc: u32 = 0; // s9
    let mut tfaw: u32 = 0; // s10
    let mut tras: u32 = 0; // s11
    let mut trp: u32 = 0; // 0(sp)
    let mut twtr: u32 = 0; // s1
    let mut twr: u32 = 0; // s6
    let mut trtp: u32 = 0; // 64(sp)
    let mut txp: u32 = 0; // a6
    let mut trefi: u32 = 0; // s2
    let mut trfc: u32 = 0; // a5 / 8(sp)

    if para.dram_tpr13 & 0x2 != 0 {
        //dram_tpr0
        tccd = (para.dram_tpr0 >> 21) & 0x7; // [23:21]
        tfaw = (para.dram_tpr0 >> 15) & 0x3f; // [20:15]
        trrd = (para.dram_tpr0 >> 11) & 0xf; // [14:11]
        trcd = (para.dram_tpr0 >> 6) & 0x1f; // [10:6 ]
        trc = (para.dram_tpr0 >> 0) & 0x3f; // [ 5:0 ]

        //dram_tpr1
        txp = (para.dram_tpr1 >> 23) & 0x1f; // [27:23]
        twtr = (para.dram_tpr1 >> 20) & 0x7; // [22:20]
        trtp = (para.dram_tpr1 >> 15) & 0x1f; // [19:15]
        twr = (para.dram_tpr1 >> 11) & 0xf; // [14:11]
        trp = (para.dram_tpr1 >> 6) & 0x1f; // [10:6 ]
        tras = (para.dram_tpr1 >> 0) & 0x3f; // [ 5:0 ]

        //dram_tpr2
        trfc = (para.dram_tpr2 >> 12) & 0x1ff; // [20:12]
        trefi = (para.dram_tpr2 >> 0) & 0xfff; // [11:0 ]
    } else {
        let frq2 = dfreq >> 1; // s0
        match dtype {
            3 => {
                // DDR3
                trfc = auto_cal_timing(350, frq2);
                trefi = auto_cal_timing(7800, frq2) / 32 + 1; // XXX
                twr = auto_cal_timing(8, frq2);
                trcd = auto_cal_timing(15, frq2);
                twtr = twr + 2; // + 2 ? XXX
                if twr < 2 {
                    twtr = 2
                };
                twr = trcd;
                if trcd < 2 {
                    twr = 2
                };
                if dfreq <= 800 {
                    tfaw = auto_cal_timing(50, frq2);
                    trrd = auto_cal_timing(10, frq2);
                    if trrd < 2 {
                        trrd = 2
                    };
                    trc = auto_cal_timing(53, frq2);
                    tras = auto_cal_timing(38, frq2);
                    txp = trrd; // 10
                    trp = trcd; // 15
                }
            }
            2 => {
                // DDR2
                tfaw = auto_cal_timing(50, frq2);
                trrd = auto_cal_timing(10, frq2);
                trcd = auto_cal_timing(20, frq2);
                trc = auto_cal_timing(65, frq2);
                twtr = auto_cal_timing(8, frq2);
                trp = auto_cal_timing(15, frq2);
                tras = auto_cal_timing(45, frq2);
                trefi = auto_cal_timing(7800, frq2) / 32;
                trfc = auto_cal_timing(328, frq2);
                txp = 2;
                twr = trp; // 15
            }
            6 => {
                // LPDDR2
                tfaw = auto_cal_timing(50, frq2);
                if tfaw < 4 {
                    tfaw = 4
                };
                trrd = auto_cal_timing(10, frq2);
                if trrd == 0 {
                    trrd = 1
                };
                trcd = auto_cal_timing(24, frq2);
                if trcd < 2 {
                    trcd = 2
                };
                trc = auto_cal_timing(70, frq2);
                txp = auto_cal_timing(8, frq2);
                if txp == 0 {
                    txp = 1;
                    twtr = 2;
                } else {
                    twtr = txp;
                    if txp < 2 {
                        txp = 2;
                        twtr = 2;
                    }
                }
                twr = auto_cal_timing(15, frq2);
                if twr < 2 {
                    twr = 2
                };
                trp = auto_cal_timing(17, frq2);
                tras = auto_cal_timing(42, frq2);
                trefi = auto_cal_timing(3900, frq2) / 32;
                trfc = auto_cal_timing(210, frq2);
            }
            7 => {
                // LPDDR3
                tfaw = auto_cal_timing(50, frq2);
                if tfaw < 4 {
                    tfaw = 4
                };
                trrd = auto_cal_timing(10, frq2);
                if trrd == 0 {
                    trrd = 1
                };
                trcd = auto_cal_timing(24, frq2);
                if trcd < 2 {
                    trcd = 2
                };
                trc = auto_cal_timing(70, frq2);
                twtr = auto_cal_timing(8, frq2);
                if twtr < 2 {
                    twtr = 2
                };
                twr = auto_cal_timing(15, frq2);
                if twr < 2 {
                    twr = 2
                };
                trp = auto_cal_timing(17, frq2);
                tras = auto_cal_timing(42, frq2);
                trefi = auto_cal_timing(3900, frq2) / 32;
                trfc = auto_cal_timing(210, frq2);
                txp = twtr;
            }
            _ => {
                // default
                trfc = 128;
                trp = 6;
                trefi = 98;
                txp = 10;
                twr = 8;
                twtr = 3;
                tras = 14;
                tfaw = 16;
                trc = 20;
                trcd = 6;
                trrd = 3;
            }
        }
        //assign the value back to the DRAM structure
        tccd = 2;
        trtp = 4; // not in .S ?
        para.dram_tpr0 = (trc << 0) as u32
            | (trcd << 6) as u32
            | (trrd << 11) as u32
            | (tfaw << 15) as u32
            | (tccd << 21) as u32;
        para.dram_tpr1 = (tras << 0) as u32
            | (trp << 6) as u32
            | (twr << 11) as u32
            | (trtp << 15) as u32
            | (twtr << 20) as u32
            | (txp << 23) as u32;
        para.dram_tpr2 = (trefi << 0) as u32 | (trfc << 12) as u32;
    }

    let tcksrx: u32; // t1
    let tckesr: u32; // t4;
    let mut trd2wr: u32; // t6
    let trasmax: u32; // t3;
    let twtp: u32; // s6 (was twr!)
    let mut tcke: u32; // s8
    let tmod: u32; // t0
    let tmrd: u32; // t5
    let tmrw: u32; // a1
    let t_rdata_en: u32; // a4 (was tcwl!)
    let tcl: u32; // a0
    let wr_latency: u32; // a7
    let tcwl: u32; // first a4, then a5
    let mr3: u32; // s0
    let mr2: u32; // t2
    let mr1: u32; // s1
    let mr0: u32; // a3
    let dmr3: u32; // 72(sp)

    //let trtp:u32;	// 64(sp)
    let dmr1: u32; // 56(sp)
    let twr2rd: u32; // 48(sp)
    let tdinit3: u32; // 40(sp)
    let tdinit2: u32; // 32(sp)
    let tdinit1: u32; // 24(sp)
    let tdinit0: u32; // 16(sp)

    dmr1 = para.dram_mr1;
    dmr3 = para.dram_mr3;

    match dtype {
        2 =>
        // DDR2
        //	L59:
        {
            trasmax = dfreq / 30;
            if dfreq < 409 {
                tcl = 3;
                t_rdata_en = 1;
                mr0 = 0x06a3;
            } else {
                t_rdata_en = 2;
                tcl = 4;
                mr0 = 0x0e73;
            }
            tmrd = 2;
            twtp = twr as u32 + 5;
            tcksrx = 5;
            tckesr = 4;
            trd2wr = 4;
            tcke = 3;
            tmod = 12;
            wr_latency = 1;
            mr3 = 0;
            mr2 = 0;
            tdinit0 = 200 * dfreq + 1;
            tdinit1 = 100 * dfreq / 1000 + 1;
            tdinit2 = 200 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            tmrw = 0;
            twr2rd = twtr as u32 + 5;
            tcwl = 0;
            mr1 = dmr1;
        }
        3 =>
        // DDR3
        //	L57:
        {
            trasmax = dfreq / 30;
            if dfreq <= 800 {
                mr0 = 0x1c70;
                tcl = 6;
                wr_latency = 2;
                tcwl = 4;
                mr2 = 24;
            } else {
                mr0 = 0x1e14;
                tcl = 7;
                wr_latency = 3;
                tcwl = 5;
                mr2 = 32;
            }

            twtp = tcwl + 2 + twtr as u32; // WL+BL/2+tWTR
            trd2wr = tcwl + 2 + twr as u32; // WL+BL/2+tWR
            twr2rd = tcwl + twtr as u32; // WL+tWTR

            tdinit0 = 500 * dfreq + 1; // 500 us
            tdinit1 = 360 * dfreq / 1000 + 1; // 360 ns
            tdinit2 = 200 * dfreq + 1; // 200 us
            tdinit3 = 1 * dfreq + 1; //   1 us

            if ((tpr13 >> 2) & 0x03) == 0x01 || dfreq < 912 {
                mr1 = dmr1;
                t_rdata_en = tcwl; // a5 <- a4
                tcksrx = 5;
                tckesr = 4;
                trd2wr = 5;
            } else {
                mr1 = dmr1;
                t_rdata_en = tcwl; // a5 <- a4
                tcksrx = 5;
                tckesr = 4;
                trd2wr = 6;
            }
            tcke = 3; // not in .S ?
            tmod = 12;
            tmrd = 4;
            tmrw = 0;
            mr3 = 0;
        }
        6 =>
        // LPDDR2
        //	L61:
        {
            trasmax = dfreq / 60;
            mr3 = dmr3;
            twtp = twr as u32 + 5;
            mr2 = 6;
            //  mr1 = 5; // TODO: this is just overwritten (?!)
            tcksrx = 5;
            tckesr = 5;
            trd2wr = 10;
            tcke = 2;
            tmod = 5;
            tmrd = 5;
            tmrw = 3;
            tcl = 4;
            wr_latency = 1;
            t_rdata_en = 1;
            tdinit0 = 200 * dfreq + 1;
            tdinit1 = 100 * dfreq / 1000 + 1;
            tdinit2 = 11 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            twr2rd = twtr as u32 + 5;
            tcwl = 2;
            mr1 = 195;
            mr0 = 0;
        }

        7 =>
        // LPDDR3
        {
            trasmax = dfreq / 60;
            if dfreq < 800 {
                tcwl = 4;
                wr_latency = 3;
                t_rdata_en = 6;
                mr2 = 12;
            } else {
                tcwl = 3;
                // tcke = 6; // FIXME: This is always overwritten
                wr_latency = 2;
                t_rdata_en = 5;
                mr2 = 10;
            }
            twtp = tcwl + 5;
            tcl = 7;
            mr3 = dmr3;
            tcksrx = 5;
            tckesr = 5;
            trd2wr = 13;
            tcke = 3;
            tmod = 12;
            tdinit0 = 400 * dfreq + 1;
            tdinit1 = 500 * dfreq / 1000 + 1;
            tdinit2 = 11 * dfreq + 1;
            tdinit3 = 1 * dfreq + 1;
            tmrd = 5;
            tmrw = 5;
            twr2rd = tcwl + twtr as u32 + 5;
            mr1 = 195;
            mr0 = 0;
        }

        _ =>
        //	L84:
        {
            twr2rd = 8; // 48(sp)
            tcksrx = 4; // t1
            tckesr = 3; // t4
            trd2wr = 4; // t6
            trasmax = 27; // t3
            twtp = 12; // s6
            tcke = 2; // s8
            tmod = 6; // t0
            tmrd = 2; // t5
            tmrw = 0; // a1
            tcwl = 3; // a5
            tcl = 3; // a0
            wr_latency = 1; // a7
            t_rdata_en = 1; // a4
            mr3 = 0; // s0
            mr2 = 0; // t2
            mr1 = 0; // s1
            mr0 = 0; // a3
            tdinit3 = 0; // 40(sp)
            tdinit2 = 0; // 32(sp)
            tdinit1 = 0; // 24(sp)
            tdinit0 = 0; // 16(sp)
        }
    }
    // L60:
    /*
    if trtp < tcl - trp + 2 {
        trtp = tcl - trp + 2;
    }
    */
    // FIXME: This always overwrites the above (?!)
    trtp = 4;

    // Update mode block when permitted
    if (para.dram_mr0 & 0xffff0000) == 0 {
        para.dram_mr0 = mr0
    };
    if (para.dram_mr1 & 0xffff0000) == 0 {
        para.dram_mr1 = mr1
    };
    if (para.dram_mr2 & 0xffff0000) == 0 {
        para.dram_mr2 = mr2
    };
    if (para.dram_mr3 & 0xffff0000) == 0 {
        para.dram_mr3 = mr3
    };

    // Set mode registers
    writel(DRAM_MR0, para.dram_mr0);
    writel(DRAM_MR1, para.dram_mr1);
    writel(DRAM_MR2, para.dram_mr2);
    writel(DRAM_MR3, para.dram_mr3);
    writel(DRAM_ODTX, (para.dram_odt_en >> 4) & 0x3); // ??

    let mut val: u32;
    // Set dram timing DRAMTMG0 - DRAMTMG5
    val = (twtp << 24) | (tfaw << 16) as u32 | (trasmax << 8) | (tras << 0) as u32;
    writel(DRAMTMG0, val);
    val = (txp << 16) as u32 | (trtp << 8) as u32 | (trc << 0) as u32;
    writel(DRAMTMG1, val);
    val = (tcwl << 24) | (tcl << 16) as u32 | (trd2wr << 8) | (twr2rd << 0);
    writel(DRAMTMG2, val);
    val = (tmrw << 16) | (tmrd << 12) | (tmod << 0);
    writel(DRAMTMG3, val);
    val = (trcd << 24) as u32 | (tccd << 16) as u32 | (trrd << 8) as u32 | (trp << 0) as u32;
    writel(DRAMTMG4, val);
    val = (tcksrx << 24) | (tcksrx << 16) | (tckesr << 8) | (tcke << 0);
    writel(DRAMTMG5, val);

    // Set two rank timing
    val = readl(DRAMTMG8);
    val &= 0x0fff0000;
    val |= if para.dram_clk < 800 {
        0xf0006600
    } else {
        0xf0007600
    };
    val |= 0x10;
    writel(DRAMTMG8, val);

    // Set phy interface time PITMG0, PTR3, PTR4
    val = (0x2 << 24) | (t_rdata_en << 16) | (0x1 << 8) | (wr_latency << 0);
    writel(PITMG0, val);
    writel(PTR3, (tdinit0 << 0) | (tdinit1 << 20));
    writel(PTR4, (tdinit2 << 0) | (tdinit3 << 20));

    // Set refresh timing and mode
    writel(RFSHTMG, (trefi << 16) as u32 | (trfc << 0) as u32);
    writel(RFSHCTL1, 0x0fff0000 & (trefi << 15) as u32);
}

// Perform an init of the controller. This is actually done 3 times. The first
// time to establish the number of ranks and DQ width. The second time to
// establish the actual ram size. The third time is final one, with the final
// settings.
fn mctl_core_init(para: &mut dram_parameters) -> Result<(), &'static str> {
    unsafe {
        println!("sys_init");
        mctl_sys_init(para);
        println!("vrefzq_init");
        mctl_vrefzq_init(para);
    }
    println!("com_init");
    mctl_com_init(para);
    unsafe {
        mctl_phy_ac_remapping(para);
    }
    auto_set_timing_para(para);
    // return mctl_channel_init(0, para);
    return Ok(());
    // return Err("DRAM initialisation error : 0"); // TODO
}

fn auto_scan_dram_size(para: &mut dram_parameters) -> Result<(), &'static str> {
    mctl_core_init(para)?; // TODO: Why is this called all the time?

    let maxrank = match para.dram_para2 & 0xf000 {
        0 => 1,
        _ => 2,
    };
    let mc_work_mode = MC_WORK_MODE_RANK0_LOW;
    let offs = 0;

    println!("DRAM write test");
    // write test pattern
    unsafe {
        for i in 0..64 {
            let ptr: u32 = RAM_BASE as u32 + 4 * i;
            write_volatile(
                (RAM_BASE as u32 + ptr) as *mut u32,
                if i & 1 > 0 { ptr } else { !ptr },
            );
        }
    }

    println!("DRAM read test");
    unsafe {
        for i in 0..64 {
            let ptr: u32 = RAM_BASE as u32 + 4 * i;
            let r = read_volatile((RAM_BASE as u32 + ptr) as *mut u32);
            println!("{:#x}", r);
        }
    }

    // TODO: do the rest

    return Err("auto scan dram size failed !");
}

fn dqs_gate_detect(para: &mut dram_parameters) -> Result<(), &'static str> {
    return Ok(());
}

fn auto_scan_dram_rank_width(para: &mut dram_parameters) -> Result<(), &'static str> {
    let s1 = para.dram_tpr13;
    let s2 = para.dram_para1;

    para.dram_para1 = 0x00b000b0;
    para.dram_para2 = (para.dram_para2 & 0xfffffff0) | 0x1000;
    para.dram_tpr13 = (s1 & 0xfffffff7) | 0x5; // set DQS probe mode

    mctl_core_init(para)?; // TODO: Why is this called all the time?

    let unknown3 = unsafe { read_volatile(UNKNOWN3 as *mut u32) };
    if unknown3 & (1 << 20) == 0 {
        return Ok(());
    }
    dqs_gate_detect(para)?;

    para.dram_tpr13 = s1;
    para.dram_para1 = s2;
    return Err("auto scan dram rank & width failed !");
}

/* STEP 2 */
/// This routine determines the SDRAM topology.
///
/// It first establishes the number of ranks and the DQ width. Then it scans the
/// SDRAM address lines to establish the size of each rank. It then updates
/// `dram_tpr13` to reflect that the sizes are now known: a re-init will not
/// repeat the autoscan.
fn auto_scan_dram_config(para: &mut dram_parameters) -> Result<(), &'static str> {
    println!("DRAM 14");
    if para.dram_tpr13 & (1 << 14) == 0 {
        println!("DRAM 14 no");
        auto_scan_dram_rank_width(para)?
    }
    println!("DRAM 0");
    if para.dram_tpr13 & (1 << 0) == 0 {
        println!("DRAM 0 no");
        // This is not run with current hardcoded params
        auto_scan_dram_size(para)?
    }
    println!("DRAM 15");
    if (para.dram_tpr13 & (1 << 15)) == 0 {
        println!("DRAM 15 no; adjusting");
        para.dram_tpr13 |= 0x6003;
    }
    Ok(())
}

/// # Safety
///
/// No warranty. Use at own risk. Be lucky to get values from vendor.
pub unsafe fn init_dram(para: &mut dram_parameters) -> usize {
    // STEP 1: ZQ, gating, calibration and voltage
    // Test ZQ status
    if para.dram_tpr13 & (1 << 16) > 0 {
        println!("DRAM only have internal ZQ!!");
        write_volatile(
            RES_CAL_CTRL_REG as *mut u32,
            read_volatile(RES_CAL_CTRL_REG as *mut u32) | 0x100,
        );
        write_volatile(RES240_CTRL_REG as *mut u32, 0);
        println!("Rust 🦀 ");
        for _ in 0..20_000_000 {}
    } else {
        // TODO: gating, calibration
        let zq_val = read_volatile(ZQ_VALUE as *mut u32);
        println!("ZQ value = 0x{:#02x}***********", zq_val);
    }

    // Set voltage
    let rc = get_pmu_exists();
    println!("get_pmu_exist() = {}\n", rc);

    if !rc {
        dram_vol_set(para);
    } else {
        if para.dram_type == 2 {
            set_ddr_voltage(1800);
        } else if para.dram_type == 3 {
            set_ddr_voltage(1500);
        }
    }

    // STEP 2: CONFIG
    // Set SDRAM controller auto config
    if (para.dram_tpr13 & 0x1) == 0 {
        if let Err(msg) = auto_scan_dram_config(para) {
            println!("[ERROR DEBUG] {}", msg);
            return 0;
        }
    }

    // Print header message (too late)
    println!("DRAM BOOT DRIVE INFO: {}", "V0.24");
    println!("DRAM CLK = {} MHz", para.dram_clk);
    println!("DRAM Type = {} (2:DDR2,3:DDR3)", para.dram_type);
    if (para.dram_odt_en & 0x1) == 0 {
        println!("DRAMC read ODT  off.");
    } else {
        println!("DRAMC ZQ value: 0x{:x}", para.dram_zq);
    }

    // report ODT
    if (para.dram_mr1 & 0x44) == 0 {
        println!("DRAM ODT off.");
    } else {
        println!("DRAM ODT value: 0x{:x}.", para.dram_mr1);
    }

    // Init core, final run
    if let Err(msg) = mctl_core_init(para) {
        println!("[ERROR DEBUG] {}", msg);
        return 0;
    };

    return 0;
}

pub fn init() -> usize {
    println!("DRAM INIT");
    return unsafe { init_dram(&mut DRAM_PARA) };
}