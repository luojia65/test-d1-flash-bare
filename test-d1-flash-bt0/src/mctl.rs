use core::ptr::{read_volatile, write_volatile};

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

const SYS_CFG: usize = 0x03000000; // 0x0300)0000 - 0x0300_0FFF
const VER_REG: usize = SYS_CFG + 0x0024;
const EMAC_EPHY_CLK_REG0: usize = SYS_CFG + 0x0030;
const SYS_LDO_CTRL_REG: usize = SYS_CFG + 0x0150;
const RES_CAL_CTRL_REG: usize = SYS_CFG + 0x0160;
const RES240_CTRL_REG: usize = SYS_CFG + 0x0168;
const RES_CAL_STATUS_REG: usize = SYS_CFG + 0x016c;

const ZQ_VALUE: usize = SYS_CFG + 0x0172;
const ZQ_INTERNAL: usize = SYS_CFG + 0x016e;

const FOO_BASE: usize = 0x7010000; // TODO: What do we call this?
const ANALOG_SYS_PWROFF_GATING_REG: usize = FOO_BASE + 0x0254;

// NOTE: MSI shares the bus clock with CE, DMAC, IOMMU and CPU_SYS; p 38
// TODO: Define *_BASE?
const MSI_MEMC_BASE: usize = 0x3102000; // p32 0x0310_2000 - 0x0330_1FFF
const MC_WORK_MODE_RANK0_LOW: usize = MSI_MEMC_BASE;
const MC_WORK_MODE_RANK0_HIGH: usize = MSI_MEMC_BASE + 0x0004;
const MC_WORK_MODE_RANK1_LOW: usize = MSI_MEMC_BASE + 0x100000;
const MC_WORK_MODE_RANK1_HIGH: usize = MSI_MEMC_BASE + 0x100004;
const UNKNOWN1: usize = MSI_MEMC_BASE + 0x0008; // 0x3102008
const UNKNOWN7: usize = MSI_MEMC_BASE + 0x000c; // 0x310200c
const UNKNOWN6: usize = MSI_MEMC_BASE + 0x0100; // 0x3102100

// TODO:
// 0x3102200
// 0x3102210
// 0x3102214
// 0x3102230
// 0x3102234
// 0x3102240
// 0x3102244
// 0x3102260
// 0x3102264
// 0x3102290
// 0x3102294
// 0x3102470
// 0x3102474
// 0x31031c0
// 0x31031c8
// 0x31031d0
const MCTL_CLK_EN: usize = MSI_MEMC_BASE + 0x100c; // 0x310300C
const UNKNOWN3: usize = MSI_MEMC_BASE + 0x1010; // 0x3103010
                                                // DATX0IOCR x + 4 * size
                                                // DATX0IOCR - DATX3IOCR: 11 registers per block, blocks 0x20 words apart
const DATX0IOCR: usize = MSI_MEMC_BASE + 0x0310; // 0x3102310
const DATX3IOCR: usize = MSI_MEMC_BASE + 0x0510; // 0x3102510
const IOCVR_LOW: usize = MSI_MEMC_BASE + 0x1110; // 0x3103110
const IOCVR_HIGH: usize = MSI_MEMC_BASE + 0x1114; // 0x3103114
const UNKNOWN4: usize = MSI_MEMC_BASE + 0x1348; // 0x3103348
const UNKNOWN5: usize = MSI_MEMC_BASE + 0x13c8; // 0x31033C8

#[repr(C)]
pub struct dram_parameters {
    pub dram_clk: usize,
    pub dram_type: usize,
    pub dram_zq: usize,
    pub dram_odt_en: usize,
    pub dram_para1: usize,
    pub dram_para2: usize,
    pub dram_mr0: usize,
    pub dram_mr1: usize,
    pub dram_mr2: usize,
    pub dram_mr3: usize,
    pub dram_tpr0: usize,
    pub dram_tpr1: usize,
    pub dram_tpr2: usize,
    pub dram_tpr3: usize,
    pub dram_tpr4: usize,
    pub dram_tpr5: usize,
    pub dram_tpr6: usize,
    pub dram_tpr7: usize,
    pub dram_tpr8: usize,
    pub dram_tpr9: usize,
    pub dram_tpr10: usize,
    pub dram_tpr11: usize,
    pub dram_tpr12: usize,
    pub dram_tpr13: usize,
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

fn get_pmu_exists() -> bool {
    return false;
}

fn dram_vol_set(dram_para: dram_parameters) {
    let vol = match dram_para.dram_type {
        2 => 47, // 1.8V
        3 => 25, // 1.5V
        _ => 0,
    };
    let vol = 25; // XXX
    let mut reg = unsafe { read_volatile(SYS_LDO_CTRL_REG as *mut u32) };
    reg &= !(0xff00);
    reg |= vol << 8;
    reg &= !(0x200000);
    unsafe {
        write_volatile(SYS_LDO_CTRL_REG as *mut u32, reg);
    }
    // TODO
    // sdelay(1);
}

fn init_dram(dram_para: dram_parameters) -> usize {
    // STEP 1: ZQ, gating, calibration and voltage
    // Test ZQ status
    if dram_para.dram_tpr13 & (1 << 16) > 0 {
        println!("DRAM only have internal ZQ!!");
        unsafe {
            write_volatile(
                RES_CAL_CTRL_REG as *mut u32,
                read_volatile(RES_CAL_CTRL_REG as *mut u32) | 0x100,
            )
        };
        println!("Rust 🦀 ");
        unsafe {
            write_volatile(RES240_CTRL_REG as *mut u32, 0);
        }
        println!("Rust 🦀 ");
        for _ in 0..20_000_000 {}
    } else {
        // TODO: gating, calibration
        let zq_val = unsafe { read_volatile(ZQ_VALUE as *mut u32) };
        println!("ZQ value = 0x{:#02x}***********", zq_val);
    }

    // Set voltage
    let rc = get_pmu_exists();
    println!("get_pmu_exist() = {}\n", rc);

    if !rc {
        dram_vol_set(dram_para);
    }

    return 0;
}

pub fn init() -> usize {
    println!("DRAM INIT");
    return init_dram(DRAM_PARA);
}
