use core::ptr::{read_volatile, write_volatile};
use d1_pac::ccu::DRAM_CLK;

const RAM_BASE: usize = 0x40000000;

// p49 ff
const CCU: usize = 0x0200_1000;
const PLL_CPU_CTRL: usize = CCU + 0x0000;
const PLL_DDR_CTRL: usize = CCU + 0x0010;
const MBUS_CLK: usize = CCU + 0x0540;
const DRAM_CLK_REG: usize = CCU + 0x0800;
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
const UNKNOWN6: usize = MSI_MEMC_BASE + 0x0100; // 0x3102100

const DRAM_MASTER_CTL1: usize = MSI_MEMC_BASE + 0x0020;
const DRAM_MASTER_CTL2: usize = MSI_MEMC_BASE + 0x0024;
const DRAM_MASTER_CTL3: usize = MSI_MEMC_BASE + 0x0028;

// DATX0IOCR x + 4 * size
// DATX0IOCR - DATX3IOCR: 11 registers per block, blocks 0x20 words apart
const DATX0IOCR: usize = MSI_MEMC_BASE + 0x0310; // 0x3102310
const DATX3IOCR: usize = MSI_MEMC_BASE + 0x0510; // 0x3102510

const MCTL_CLK_EN: usize = MSI_MEMC_BASE + 0x100c; // 0x310300C
const UNKNOWN3: usize = MSI_MEMC_BASE + 0x1010; // 0x3103010

const IOCVR_LOW: usize = MSI_MEMC_BASE + 0x1110; // 0x3103110
const IOCVR_HIGH: usize = MSI_MEMC_BASE + 0x1114; // 0x3103114
const UNKNOWN9: usize = MSI_MEMC_BASE + 0x1120; // 0x3103120
const UNKNOWN4: usize = MSI_MEMC_BASE + 0x1348; // 0x3103348
const UNKNOWN10: usize = MSI_MEMC_BASE + 0x13c4; // 0x31033c4;
const UNKNOWN5: usize = MSI_MEMC_BASE + 0x13c8; // 0x31033C8

// TODO: *_BASE ?
const MC_WORK_MODE_RANK1_LOW: usize = MSI_MEMC_BASE + 0x10_0000;
const MC_WORK_MODE_RANK1_HIGH: usize = MSI_MEMC_BASE + 0x10_0004;
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
unsafe fn ccm_set_pll_ddr_clk(para: &mut dram_parameters, clk: &DRAM_CLK) -> u32 {
    // FIXME: This is a bit weird, especially the scaling down and up etc
    let clk_rate = match para.dram_tpr13 & (1 << 6) {
        0 => para.dram_clk,
        _ => para.dram_tpr9,
    };
    let n = 66; // (clk_rate * 2) / 24; // FIXME: Why is this always 0?
    println!("clk {} / n {}", clk_rate, n);

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
    clk.write(|w| {
        // select DDR clk source, n=1, m=1
        w.clk_src_sel().pll_ddr();
        w.dram_div2().n1();
        w.dram_div1().bits(0);
        // turn clock on
        w.clk_gating().on()
    });

    return n * 24;
}

// TODO: verify this
unsafe fn mctl_sys_init(para: &mut dram_parameters, clk: &DRAM_CLK) {
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
    // read_volatile(DRAM_CLK_REG as *mut u32);
    let val = clk.read().bits();
    write_volatile(DRAM_CLK_REG as *mut u32, val & 0xbfffffff);
    // and toggle dram clock gating off + trigger update
    clk.write(|w| {
        w.clk_gating().clear_bit();
        w.sdrclk_upd().set_bit()
    });
    // sdelay(10); // TODO
    for _ in 0..10_000 {
        core::arch::asm!("nop")
    }

    println!("ccm_set_pll_ddr_clk");
    // set ddr pll clock
    // NOTE: This passes an additional `0` in the original, but it's unused
    let val = ccm_set_pll_ddr_clk(para, clk);
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
    // let val = read_volatile(DRAM_CLK_REG as *mut u32);
    let val = clk.read().bits();
    write_volatile(DRAM_CLK_REG as *mut u32, val | 0x40000000);

    println!("SDRAM clock gate ON");
    // turn on sdram clock gate
    let val = read_volatile(DRAM_BGR as *mut u32);
    write_volatile(DRAM_BGR as *mut u32, val | 0x0000001); // (1<<0);

    // turn dram clock gate on, trigger sdr clock update
    clk.write(|w| {
        w.clk_gating().set_bit();
        w.sdrclk_upd().set_bit()
    });
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

// Perform an init of the controller. This is actually done 3 times. The first
// time to establish the number of ranks and DQ width. The second time to
// establish the actual ram size. The third time is final one, with the final
// settings.
fn mctl_core_init(para: &mut dram_parameters, clk: &DRAM_CLK) -> Result<(), &'static str> {
    unsafe {
        println!("sys_init");
        mctl_sys_init(para, clk);
        println!("vrefzq_init");
        mctl_vrefzq_init(para);
    }
    println!("com_init");
    mctl_com_init(para);
    // mctl_phy_ac_remapping(para);
    // auto_set_timing_para(para);
    // return mctl_channel_init(0, para);
    return Ok(());
    // return Err("DRAM initialisation error : 0"); // TODO
}

fn auto_scan_dram_size(para: &mut dram_parameters, clk: &DRAM_CLK) -> Result<(), &'static str> {
    mctl_core_init(para, clk)?; // TODO: Why is this called all the time?

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

fn auto_scan_dram_rank_width(
    para: &mut dram_parameters,
    clk: &DRAM_CLK,
) -> Result<(), &'static str> {
    let s1 = para.dram_tpr13;
    let s2 = para.dram_para1;

    para.dram_para1 = 0x00b000b0;
    para.dram_para2 = (para.dram_para2 & 0xfffffff0) | 0x1000;
    para.dram_tpr13 = (s1 & 0xfffffff7) | 0x5; // set DQS probe mode

    mctl_core_init(para, clk)?; // TODO: Why is this called all the time?

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
fn auto_scan_dram_config(para: &mut dram_parameters, clk: &DRAM_CLK) -> Result<(), &'static str> {
    println!("DRAM 14");
    if para.dram_tpr13 & (1 << 14) == 0 {
        println!("DRAM 14 no");
        auto_scan_dram_rank_width(para, clk)?
    }
    println!("DRAM 0");
    if para.dram_tpr13 & (1 << 0) == 0 {
        println!("DRAM 0 no");
        // This is not run with current hardcoded params
        auto_scan_dram_size(para, clk)?
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
pub unsafe fn init_dram(para: &mut dram_parameters, clk: &DRAM_CLK) -> usize {
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
        if let Err(msg) = auto_scan_dram_config(para, clk) {
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
    if let Err(msg) = mctl_core_init(para, clk) {
        println!("[ERROR DEBUG] {}", msg);
        return 0;
    };

    return 0;
}

pub fn init(clk: &DRAM_CLK) -> usize {
    println!("DRAM INIT");
    return unsafe { init_dram(&mut DRAM_PARA, clk) };
}
