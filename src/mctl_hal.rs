#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(asm, register_tool)]
use std::arch::asm;
extern "C" {
    fn sdelay(loops: libc::c_ulong);
    fn printf(fmt: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __u32 = libc::c_uint;
pub type u32_0 = __u32;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __DRAM_PARA {
    pub dram_clk: libc::c_uint,
    pub dram_type: libc::c_uint,
    pub dram_zq: libc::c_uint,
    pub dram_odt_en: libc::c_uint,
    pub dram_para1: libc::c_uint,
    pub dram_para2: libc::c_uint,
    pub dram_mr0: libc::c_uint,
    pub dram_mr1: libc::c_uint,
    pub dram_mr2: libc::c_uint,
    pub dram_mr3: libc::c_uint,
    pub dram_tpr0: libc::c_uint,
    pub dram_tpr1: libc::c_uint,
    pub dram_tpr2: libc::c_uint,
    pub dram_tpr3: libc::c_uint,
    pub dram_tpr4: libc::c_uint,
    pub dram_tpr5: libc::c_uint,
    pub dram_tpr6: libc::c_uint,
    pub dram_tpr7: libc::c_uint,
    pub dram_tpr8: libc::c_uint,
    pub dram_tpr9: libc::c_uint,
    pub dram_tpr10: libc::c_uint,
    pub dram_tpr11: libc::c_uint,
    pub dram_tpr12: libc::c_uint,
    pub dram_tpr13: libc::c_uint,
}
pub type dram_para_t = __DRAM_PARA;
#[inline(always)]
unsafe extern "C" fn rv_writel(mut val: u32_0, mut addr: *mut libc::c_void) {
    asm!("fence ow,ow", options(preserves_flags));
    *(addr as *mut libc::c_uint) = val;
}
#[inline(always)]
unsafe extern "C" fn rv_readl(mut addr: *const libc::c_void) -> u32_0 {
    let mut val: u32_0 = 0;
    val = *(addr as *mut libc::c_uint);
    asm!("fence ir,ir", options(preserves_flags));
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn set_ddr_voltage(mut val: libc::c_int) -> libc::c_int {
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn handler_super_standby() {}
#[no_mangle]
pub unsafe extern "C" fn get_pmu_exists() -> libc::c_int {
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn memcpy_self(
    mut dst: *mut libc::c_char,
    mut src: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i != len {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn dram_udelay(mut d1: libc::c_uint) {
    let mut d2: libc::c_ulonglong = d1 as libc::c_ulonglong;
    sdelay(d2 as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn dram_vol_set(mut para: *mut dram_para_t) {
    let mut reg: libc::c_int = 0;
    let mut vol: libc::c_int = 0 as libc::c_int;
    match (*para).dram_type {
        2 => {
            vol = 47 as libc::c_int;
        }
        3 => {
            vol = 25 as libc::c_int;
        }
        _ => {
            vol = 0 as libc::c_int;
        }
    }
    vol = 25 as libc::c_int;
    reg = rv_readl(
        (0x3000000 as libc::c_int as *mut libc::c_char).offset(0x150 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) as libc::c_int;
    reg &= !(0xff00 as libc::c_int);
    reg |= vol << 8 as libc::c_int;
    reg &= !(0x200000 as libc::c_int);
    rv_writel(
        reg as u32_0,
        (0x3000000 as libc::c_int as *mut libc::c_char).offset(0x150 as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
    sdelay(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn paraconfig(
    mut para: *mut libc::c_uint,
    mut mask: libc::c_uint,
    mut value: libc::c_uint,
) {
    *para &= !mask;
    *para |= value;
}
#[no_mangle]
pub unsafe extern "C" fn dram_enable_all_master() {
    rv_writel(
        -(1 as libc::c_int) as u32_0,
        0x3102020 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0xff as libc::c_int as u32_0,
        0x3102024 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0xffff as libc::c_int as u32_0,
        0x3102028 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn dram_disable_all_master() {
    rv_writel(
        1 as libc::c_int as u32_0,
        0x3102020 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0 as libc::c_int as u32_0,
        0x3102024 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0 as libc::c_int as u32_0,
        0x3102028 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn eye_delay_compensation(mut para: *mut dram_para_t) {
    let mut val: libc::c_uint = 0;
    let mut ptr: libc::c_uint = 0;
    ptr = 0x3103310 as libc::c_int as libc::c_uint;
    while ptr != 0x3103334 as libc::c_int as libc::c_uint {
        val = rv_readl(ptr as intptr_t as *const libc::c_void);
        val |= (*para).dram_tpr11 << 9 as libc::c_int & 0x1e00 as libc::c_int as libc::c_uint;
        val |= (*para).dram_tpr12 << 1 as libc::c_int & 0x1e as libc::c_int as libc::c_uint;
        rv_writel(val, ptr as intptr_t as *mut libc::c_void);
        ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    ptr = 0x3103390 as libc::c_int as libc::c_uint;
    while ptr != 0x31033b4 as libc::c_int as libc::c_uint {
        val = rv_readl(ptr as intptr_t as *const libc::c_void);
        val |= ((*para).dram_tpr11 >> 4 as libc::c_int) << 9 as libc::c_int
            & 0x1e00 as libc::c_int as libc::c_uint;
        val |= ((*para).dram_tpr12 >> 4 as libc::c_int) << 1 as libc::c_int
            & 0x1e as libc::c_int as libc::c_uint;
        rv_writel(val, ptr as intptr_t as *mut libc::c_void);
        ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    val = rv_readl(0x3103100 as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xfbffffff as libc::c_uint;
    rv_writel(
        val,
        0x3103100 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103334 as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 16 as libc::c_int) << 9 as libc::c_int
        & 0x1e00 as libc::c_int as libc::c_uint;
    val |= ((*para).dram_tpr12 >> 16 as libc::c_int) << 1 as libc::c_int
        & 0x1e as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103334 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103338 as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 16 as libc::c_int) << 9 as libc::c_int
        & 0x1e00 as libc::c_int as libc::c_uint;
    val |= ((*para).dram_tpr12 >> 16 as libc::c_int) << 1 as libc::c_int
        & 0x1e as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103338 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x31033b4 as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 20 as libc::c_int) << 9 as libc::c_int
        & 0x1e00 as libc::c_int as libc::c_uint;
    val |= ((*para).dram_tpr12 >> 20 as libc::c_int) << 1 as libc::c_int
        & 0x1e as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x31033b4 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x31033b8 as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 20 as libc::c_int) << 9 as libc::c_int
        & 0x1e00 as libc::c_int as libc::c_uint;
    val |= ((*para).dram_tpr12 >> 20 as libc::c_int) << 1 as libc::c_int
        & 0x1e as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x31033b8 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x310333c as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 16 as libc::c_int) << 25 as libc::c_int
        & 0x1e000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x310333c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x31033bc as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr11 >> 20 as libc::c_int) << 25 as libc::c_int
        & 0x1e000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x31033bc as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103100 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x4000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103100 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(1 as libc::c_int as libc::c_ulong);
    ptr = 0x3103240 as libc::c_int as libc::c_uint;
    while ptr != 0x310327c as libc::c_int as libc::c_uint {
        val = rv_readl(ptr as intptr_t as *const libc::c_void);
        val |= ((*para).dram_tpr10 >> 4 as libc::c_int) << 8 as libc::c_int
            & 0xf00 as libc::c_int as libc::c_uint;
        rv_writel(val, ptr as intptr_t as *mut libc::c_void);
        ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    ptr = 0x3103228 as libc::c_int as libc::c_uint;
    while ptr != 0x3103240 as libc::c_int as libc::c_uint {
        val = rv_readl(ptr as intptr_t as *const libc::c_void);
        val |= ((*para).dram_tpr10 >> 4 as libc::c_int) << 8 as libc::c_int
            & 0xf00 as libc::c_int as libc::c_uint;
        rv_writel(val, ptr as intptr_t as *mut libc::c_void);
        ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    val = rv_readl(0x3103218 as libc::c_int as intptr_t as *const libc::c_void);
    val |= (*para).dram_tpr10 << 8 as libc::c_int & 0xf00 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103218 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x310321c as libc::c_int as intptr_t as *const libc::c_void);
    val |= (*para).dram_tpr10 << 8 as libc::c_int & 0xf00 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x310321c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103280 as libc::c_int as intptr_t as *const libc::c_void);
    val |= ((*para).dram_tpr10 >> 12 as libc::c_int) << 8 as libc::c_int
        & 0xf00 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103280 as libc::c_int as intptr_t as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bit_delay_compensation() {
    let data0: [libc::c_uint; 44] = [
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    ];
    let data1: [libc::c_uint; 44] = [
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    ];
    let mut start: *mut libc::c_uint = (0x3102000 as libc::c_int as *mut libc::c_char)
        .offset(0x310 as libc::c_int as isize)
        as *mut libc::c_uint;
    let mut end: *mut libc::c_uint = (0x3102000 as libc::c_int as *mut libc::c_char)
        .offset(0x510 as libc::c_int as isize)
        as *mut libc::c_uint;
    let mut datxiocr: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut rval: libc::c_uint = 0;
    rval = rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x100 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) & 0x3ffffff as libc::c_int as libc::c_uint;
    rv_writel(
        rval,
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x100 as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
    i = 0 as libc::c_int as libc::c_uint;
    datxiocr = start;
    while datxiocr != end {
        j = 0 as libc::c_int as libc::c_uint;
        k = i;
        while j != 11 as libc::c_int as libc::c_uint {
            rval = rv_readl(*datxiocr.offset(j as isize) as intptr_t as *const libc::c_void);
            rval = rval.wrapping_add(data1[k as usize] << 8 as libc::c_int);
            rval = rval.wrapping_add(data0[k as usize]);
            rv_writel(
                rval,
                *datxiocr.offset(j as isize) as intptr_t as *mut libc::c_void,
            );
            j = j.wrapping_add(1);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(11 as libc::c_int as libc::c_uint);
        datxiocr = datxiocr.offset(0x20 as libc::c_int as isize);
    }
    rval = rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x100 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) | 0x4000000 as libc::c_int as libc::c_uint;
    rv_writel(
        rval,
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x100 as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_master_priority_pad(mut para: *mut dram_para_t) {
    let mut val: libc::c_uint = 0;
    val = rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0xc as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) & 0xfffff000 as libc::c_uint;
    val |= ((*para).dram_clk >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    rv_writel(
        val,
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0xc as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x1000 as libc::c_int as u32_0,
        0x3102200 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x1000009 as libc::c_int as u32_0,
        0x3102210 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x500100 as libc::c_int as u32_0,
        0x3102214 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x200000d as libc::c_int as u32_0,
        0x3102230 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x600100 as libc::c_int as u32_0,
        0x3102234 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x1000009 as libc::c_int as u32_0,
        0x3102240 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x500100 as libc::c_int as u32_0,
        0x3102244 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x640209 as libc::c_int as u32_0,
        0x3102260 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x200040 as libc::c_int as u32_0,
        0x3102264 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x1000009 as libc::c_int as u32_0,
        0x3102290 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x400080 as libc::c_int as u32_0,
        0x3102294 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0 as libc::c_int as u32_0,
        0x3102470 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0 as libc::c_int as u32_0,
        0x3102474 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0xf802f05 as libc::c_int as u32_0,
        0x31031c0 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0xf0000ff as libc::c_int as u32_0,
        0x31031c8 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        0x3f00005f as libc::c_int as u32_0,
        0x31031d0 as libc::c_int as intptr_t as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn auto_cal_timing(
    mut time: libc::c_uint,
    mut freq: libc::c_uint,
) -> libc::c_int {
    let mut t: libc::c_uint = time.wrapping_mul(freq);
    return t
        .wrapping_div(1000 as libc::c_int as libc::c_uint)
        .wrapping_add(
            (if t.wrapping_rem(1000 as libc::c_int as libc::c_uint)
                != 0 as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn auto_set_timing_para(mut para: *mut dram_para_t) {
    let mut freq: libc::c_uint = 0;
    let mut type_0: libc::c_uint = 0;
    let mut tpr13: libc::c_uint = 0;
    let mut reg_val: libc::c_uint = 0;
    let mut tccd: libc::c_uchar = 0;
    let mut trrd: libc::c_uchar = 0;
    let mut trcd: libc::c_uchar = 0;
    let mut trc: libc::c_uchar = 0;
    let mut tfaw: libc::c_uchar = 0;
    let mut tras: libc::c_uchar = 0;
    let mut trp: libc::c_uchar = 0;
    let mut twtr: libc::c_uchar = 0;
    let mut twr: libc::c_uchar = 0;
    let mut trtp: libc::c_uchar = 0;
    let mut txp: libc::c_uchar = 0;
    let mut trefi: libc::c_ushort = 0;
    let mut trfc: libc::c_ushort = 0;
    freq = (*para).dram_clk;
    type_0 = (*para).dram_type;
    tpr13 = (*para).dram_tpr13;
    if (*para).dram_tpr13 & 0x2 as libc::c_int as libc::c_uint != 0 {
        tccd = ((*para).dram_tpr0 >> 21 as libc::c_int & 0x7 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        tfaw = ((*para).dram_tpr0 >> 15 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trrd = ((*para).dram_tpr0 >> 11 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trcd = ((*para).dram_tpr0 >> 6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trc = ((*para).dram_tpr0 >> 0 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        txp = ((*para).dram_tpr1 >> 23 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        twtr = ((*para).dram_tpr1 >> 20 as libc::c_int & 0x7 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trtp = ((*para).dram_tpr1 >> 15 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        twr = ((*para).dram_tpr1 >> 11 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trp = ((*para).dram_tpr1 >> 6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        tras = ((*para).dram_tpr1 >> 0 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        trfc = ((*para).dram_tpr2 >> 12 as libc::c_int & 0x1ff as libc::c_int as libc::c_uint)
            as libc::c_ushort;
        trefi = ((*para).dram_tpr2 >> 0 as libc::c_int & 0xfff as libc::c_int as libc::c_uint)
            as libc::c_ushort;
    } else {
        let mut frq2: libc::c_uint = freq >> 1 as libc::c_int;
        if type_0 == 3 as libc::c_int as libc::c_uint {
            trfc = auto_cal_timing(350 as libc::c_int as libc::c_uint, frq2) as libc::c_ushort;
            trefi = (auto_cal_timing(7800 as libc::c_int as libc::c_uint, frq2) / 32 as libc::c_int
                + 1 as libc::c_int) as libc::c_ushort;
            twr = auto_cal_timing(8 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trcd = auto_cal_timing(15 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            twtr = (twr as libc::c_int + 2 as libc::c_int) as libc::c_uchar;
            if (twr as libc::c_int) < 2 as libc::c_int {
                twtr = 2 as libc::c_int as libc::c_uchar;
            }
            twr = trcd;
            if (trcd as libc::c_int) < 2 as libc::c_int {
                twr = 2 as libc::c_int as libc::c_uchar;
            }
            if freq <= 800 as libc::c_int as libc::c_uint {
                tfaw = auto_cal_timing(50 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                trrd = auto_cal_timing(10 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                if (trrd as libc::c_int) < 2 as libc::c_int {
                    trrd = 2 as libc::c_int as libc::c_uchar;
                }
                trc = auto_cal_timing(53 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                tras = auto_cal_timing(38 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                txp = trrd;
                trp = trcd;
            } else {
                tfaw = auto_cal_timing(35 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                trrd = auto_cal_timing(10 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                if (trrd as libc::c_int) < 2 as libc::c_int {
                    trrd = 2 as libc::c_int as libc::c_uchar;
                }
                trcd = auto_cal_timing(14 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                trc = auto_cal_timing(48 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                tras = auto_cal_timing(34 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
                txp = trrd;
                trp = trcd;
            }
        } else if type_0 == 2 as libc::c_int as libc::c_uint {
            tfaw = auto_cal_timing(50 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trrd = auto_cal_timing(10 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trcd = auto_cal_timing(20 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trc = auto_cal_timing(65 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            twtr = auto_cal_timing(8 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trp = auto_cal_timing(15 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            tras = auto_cal_timing(45 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trefi = (auto_cal_timing(7800 as libc::c_int as libc::c_uint, frq2) / 32 as libc::c_int)
                as libc::c_ushort;
            trfc = auto_cal_timing(328 as libc::c_int as libc::c_uint, frq2) as libc::c_ushort;
            txp = 2 as libc::c_int as libc::c_uchar;
            twr = trp;
        } else if type_0 == 6 as libc::c_int as libc::c_uint {
            tfaw = auto_cal_timing(50 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (tfaw as libc::c_int) < 4 as libc::c_int {
                tfaw = 4 as libc::c_int as libc::c_uchar;
            }
            trrd = auto_cal_timing(10 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if trrd as libc::c_int == 0 as libc::c_int {
                trrd = 1 as libc::c_int as libc::c_uchar;
            }
            trcd = auto_cal_timing(24 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (trcd as libc::c_int) < 2 as libc::c_int {
                trcd = 2 as libc::c_int as libc::c_uchar;
            }
            trc = auto_cal_timing(70 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            txp = auto_cal_timing(8 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if txp as libc::c_int == 0 as libc::c_int {
                txp = 1 as libc::c_int as libc::c_uchar;
                twtr = 2 as libc::c_int as libc::c_uchar;
            } else {
                twtr = txp;
                if (txp as libc::c_int) < 2 as libc::c_int {
                    txp = 2 as libc::c_int as libc::c_uchar;
                    twtr = 2 as libc::c_int as libc::c_uchar;
                }
            }
            twr = auto_cal_timing(15 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (twr as libc::c_int) < 2 as libc::c_int {
                twr = 2 as libc::c_int as libc::c_uchar;
            }
            trp = auto_cal_timing(17 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            tras = auto_cal_timing(42 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trefi = (auto_cal_timing(3900 as libc::c_int as libc::c_uint, frq2) / 32 as libc::c_int)
                as libc::c_ushort;
            trfc = auto_cal_timing(210 as libc::c_int as libc::c_uint, frq2) as libc::c_ushort;
        } else if type_0 == 7 as libc::c_int as libc::c_uint {
            tfaw = auto_cal_timing(50 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (tfaw as libc::c_int) < 4 as libc::c_int {
                tfaw = 4 as libc::c_int as libc::c_uchar;
            }
            trrd = auto_cal_timing(10 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if trrd as libc::c_int == 0 as libc::c_int {
                trrd = 1 as libc::c_int as libc::c_uchar;
            }
            trcd = auto_cal_timing(24 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (trcd as libc::c_int) < 2 as libc::c_int {
                trcd = 2 as libc::c_int as libc::c_uchar;
            }
            trc = auto_cal_timing(70 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            twtr = auto_cal_timing(8 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (twtr as libc::c_int) < 2 as libc::c_int {
                twtr = 2 as libc::c_int as libc::c_uchar;
            }
            twr = auto_cal_timing(15 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            if (twr as libc::c_int) < 2 as libc::c_int {
                twr = 2 as libc::c_int as libc::c_uchar;
            }
            trp = auto_cal_timing(17 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            tras = auto_cal_timing(42 as libc::c_int as libc::c_uint, frq2) as libc::c_uchar;
            trefi = (auto_cal_timing(3900 as libc::c_int as libc::c_uint, frq2) / 32 as libc::c_int)
                as libc::c_ushort;
            trfc = auto_cal_timing(210 as libc::c_int as libc::c_uint, frq2) as libc::c_ushort;
            txp = twtr;
        } else {
            trfc = 128 as libc::c_int as libc::c_ushort;
            trp = 6 as libc::c_int as libc::c_uchar;
            trefi = 98 as libc::c_int as libc::c_ushort;
            txp = 10 as libc::c_int as libc::c_uchar;
            twr = 8 as libc::c_int as libc::c_uchar;
            twtr = 3 as libc::c_int as libc::c_uchar;
            tras = 14 as libc::c_int as libc::c_uchar;
            tfaw = 16 as libc::c_int as libc::c_uchar;
            trc = 20 as libc::c_int as libc::c_uchar;
            trcd = 6 as libc::c_int as libc::c_uchar;
            trrd = 3 as libc::c_int as libc::c_uchar;
        }
        tccd = 2 as libc::c_int as libc::c_uchar;
        trtp = 4 as libc::c_int as libc::c_uchar;
        (*para).dram_tpr0 = ((trc as libc::c_int) << 0 as libc::c_int
            | (trcd as libc::c_int) << 6 as libc::c_int
            | (trrd as libc::c_int) << 11 as libc::c_int
            | (tfaw as libc::c_int) << 15 as libc::c_int
            | (tccd as libc::c_int) << 21 as libc::c_int)
            as libc::c_uint;
        (*para).dram_tpr1 = ((tras as libc::c_int) << 0 as libc::c_int
            | (trp as libc::c_int) << 6 as libc::c_int
            | (twr as libc::c_int) << 11 as libc::c_int
            | (trtp as libc::c_int) << 15 as libc::c_int
            | (twtr as libc::c_int) << 20 as libc::c_int
            | (txp as libc::c_int) << 23 as libc::c_int)
            as libc::c_uint;
        (*para).dram_tpr2 = ((trefi as libc::c_int) << 0 as libc::c_int
            | (trfc as libc::c_int) << 12 as libc::c_int)
            as libc::c_uint;
    }
    let mut tcksrx: libc::c_uint = 0;
    let mut tckesr: libc::c_uint = 0;
    let mut trd2wr: libc::c_uint = 0;
    let mut trasmax: libc::c_uint = 0;
    let mut twtp: libc::c_uint = 0;
    let mut tcke: libc::c_uint = 0;
    let mut tmod: libc::c_uint = 0;
    let mut tmrd: libc::c_uint = 0;
    let mut tmrw: libc::c_uint = 0;
    let mut t_rdata_en: libc::c_uint = 0;
    let mut tcl: libc::c_uint = 0;
    let mut wr_latency: libc::c_uint = 0;
    let mut tcwl: libc::c_uint = 0;
    let mut mr3: libc::c_uint = 0;
    let mut mr2: libc::c_uint = 0;
    let mut mr1: libc::c_uint = 0;
    let mut mr0: libc::c_uint = 0;
    let mut dmr3: libc::c_uint = 0;
    let mut dmr1: libc::c_uint = 0;
    let mut twr2rd: libc::c_uint = 0;
    let mut tdinit3: libc::c_uint = 0;
    let mut tdinit2: libc::c_uint = 0;
    let mut tdinit1: libc::c_uint = 0;
    let mut tdinit0: libc::c_uint = 0;
    dmr1 = (*para).dram_mr1;
    dmr3 = (*para).dram_mr3;
    match type_0 {
        2 => {
            trasmax = freq.wrapping_div(30 as libc::c_int as libc::c_uint);
            if freq < 409 as libc::c_int as libc::c_uint {
                tcl = 3 as libc::c_int as libc::c_uint;
                t_rdata_en = 1 as libc::c_int as libc::c_uint;
                mr0 = 0x6a3 as libc::c_int as libc::c_uint;
            } else {
                t_rdata_en = 2 as libc::c_int as libc::c_uint;
                tcl = 4 as libc::c_int as libc::c_uint;
                mr0 = 0xe73 as libc::c_int as libc::c_uint;
            }
            tmrd = 2 as libc::c_int as libc::c_uint;
            twtp = (twr as libc::c_int + 5 as libc::c_int) as libc::c_uint;
            tcksrx = 5 as libc::c_int as libc::c_uint;
            tckesr = 4 as libc::c_int as libc::c_uint;
            trd2wr = 4 as libc::c_int as libc::c_uint;
            tcke = 3 as libc::c_int as libc::c_uint;
            tmod = 12 as libc::c_int as libc::c_uint;
            wr_latency = 1 as libc::c_int as libc::c_uint;
            mr3 = 0 as libc::c_int as libc::c_uint;
            mr2 = 0 as libc::c_int as libc::c_uint;
            tdinit0 = (200 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit1 = (100 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_div(1000 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit2 = (200 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit3 = (1 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tmrw = 0 as libc::c_int as libc::c_uint;
            twr2rd = (twtr as libc::c_int + 5 as libc::c_int) as libc::c_uint;
            tcwl = 0 as libc::c_int as libc::c_uint;
            mr1 = dmr1;
        }
        3 => {
            trasmax = freq.wrapping_div(30 as libc::c_int as libc::c_uint);
            if freq <= 800 as libc::c_int as libc::c_uint {
                mr0 = 0x1c70 as libc::c_int as libc::c_uint;
                tcl = 6 as libc::c_int as libc::c_uint;
                wr_latency = 2 as libc::c_int as libc::c_uint;
                tcwl = 4 as libc::c_int as libc::c_uint;
                mr2 = 24 as libc::c_int as libc::c_uint;
            } else {
                mr0 = 0x1e14 as libc::c_int as libc::c_uint;
                tcl = 7 as libc::c_int as libc::c_uint;
                wr_latency = 3 as libc::c_int as libc::c_uint;
                tcwl = 5 as libc::c_int as libc::c_uint;
                mr2 = 32 as libc::c_int as libc::c_uint;
            }
            twtp = tcwl
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_add(twtr as libc::c_uint);
            trd2wr = tcwl
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_add(twr as libc::c_uint);
            twr2rd = tcwl.wrapping_add(twtr as libc::c_uint);
            tdinit0 = (500 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit1 = (360 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_div(1000 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit2 = (200 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit3 = (1 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            if tpr13 >> 2 as libc::c_int & 0x3 as libc::c_int as libc::c_uint
                == 0x1 as libc::c_int as libc::c_uint
                || freq < 912 as libc::c_int as libc::c_uint
            {
                mr1 = dmr1;
                t_rdata_en = tcwl;
                tcksrx = 5 as libc::c_int as libc::c_uint;
                tckesr = 4 as libc::c_int as libc::c_uint;
                trd2wr = 5 as libc::c_int as libc::c_uint;
            } else {
                mr1 = dmr1;
                t_rdata_en = tcwl;
                tcksrx = 5 as libc::c_int as libc::c_uint;
                tckesr = 4 as libc::c_int as libc::c_uint;
                trd2wr = 6 as libc::c_int as libc::c_uint;
            }
            tcke = 3 as libc::c_int as libc::c_uint;
            tmod = 12 as libc::c_int as libc::c_uint;
            tmrd = 4 as libc::c_int as libc::c_uint;
            tmrw = 0 as libc::c_int as libc::c_uint;
            mr3 = 0 as libc::c_int as libc::c_uint;
        }
        6 => {
            trasmax = freq.wrapping_div(60 as libc::c_int as libc::c_uint);
            mr3 = dmr3;
            twtp = (twr as libc::c_int + 5 as libc::c_int) as libc::c_uint;
            mr2 = 6 as libc::c_int as libc::c_uint;
            mr1 = 5 as libc::c_int as libc::c_uint;
            tcksrx = 5 as libc::c_int as libc::c_uint;
            tckesr = 5 as libc::c_int as libc::c_uint;
            trd2wr = 10 as libc::c_int as libc::c_uint;
            tcke = 2 as libc::c_int as libc::c_uint;
            tmod = 5 as libc::c_int as libc::c_uint;
            tmrd = 5 as libc::c_int as libc::c_uint;
            tmrw = 3 as libc::c_int as libc::c_uint;
            tcl = 4 as libc::c_int as libc::c_uint;
            wr_latency = 1 as libc::c_int as libc::c_uint;
            t_rdata_en = 1 as libc::c_int as libc::c_uint;
            tdinit0 = (200 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit1 = (100 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_div(1000 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit2 = (11 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit3 = (1 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            twr2rd = (twtr as libc::c_int + 5 as libc::c_int) as libc::c_uint;
            tcwl = 2 as libc::c_int as libc::c_uint;
            mr1 = 195 as libc::c_int as libc::c_uint;
            mr0 = 0 as libc::c_int as libc::c_uint;
        }
        7 => {
            trasmax = freq.wrapping_div(60 as libc::c_int as libc::c_uint);
            if freq < 800 as libc::c_int as libc::c_uint {
                tcwl = 4 as libc::c_int as libc::c_uint;
                wr_latency = 3 as libc::c_int as libc::c_uint;
                t_rdata_en = 6 as libc::c_int as libc::c_uint;
                mr2 = 12 as libc::c_int as libc::c_uint;
            } else {
                tcwl = 3 as libc::c_int as libc::c_uint;
                tcke = 6 as libc::c_int as libc::c_uint;
                wr_latency = 2 as libc::c_int as libc::c_uint;
                t_rdata_en = 5 as libc::c_int as libc::c_uint;
                mr2 = 10 as libc::c_int as libc::c_uint;
            }
            twtp = tcwl.wrapping_add(5 as libc::c_int as libc::c_uint);
            tcl = 7 as libc::c_int as libc::c_uint;
            mr3 = dmr3;
            tcksrx = 5 as libc::c_int as libc::c_uint;
            tckesr = 5 as libc::c_int as libc::c_uint;
            trd2wr = 13 as libc::c_int as libc::c_uint;
            tcke = 3 as libc::c_int as libc::c_uint;
            tmod = 12 as libc::c_int as libc::c_uint;
            tdinit0 = (400 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit1 = (500 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_div(1000 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit2 = (11 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tdinit3 = (1 as libc::c_int as libc::c_uint)
                .wrapping_mul(freq)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            tmrd = 5 as libc::c_int as libc::c_uint;
            tmrw = 5 as libc::c_int as libc::c_uint;
            twr2rd = tcwl
                .wrapping_add(twtr as libc::c_uint)
                .wrapping_add(5 as libc::c_int as libc::c_uint);
            mr1 = 195 as libc::c_int as libc::c_uint;
            mr0 = 0 as libc::c_int as libc::c_uint;
        }
        _ => {
            twr2rd = 8 as libc::c_int as libc::c_uint;
            tcksrx = 4 as libc::c_int as libc::c_uint;
            tckesr = 3 as libc::c_int as libc::c_uint;
            trd2wr = 4 as libc::c_int as libc::c_uint;
            trasmax = 27 as libc::c_int as libc::c_uint;
            twtp = 12 as libc::c_int as libc::c_uint;
            tcke = 2 as libc::c_int as libc::c_uint;
            tmod = 6 as libc::c_int as libc::c_uint;
            tmrd = 2 as libc::c_int as libc::c_uint;
            tmrw = 0 as libc::c_int as libc::c_uint;
            tcwl = 3 as libc::c_int as libc::c_uint;
            tcl = 3 as libc::c_int as libc::c_uint;
            wr_latency = 1 as libc::c_int as libc::c_uint;
            t_rdata_en = 1 as libc::c_int as libc::c_uint;
            mr3 = 0 as libc::c_int as libc::c_uint;
            mr2 = 0 as libc::c_int as libc::c_uint;
            mr1 = 0 as libc::c_int as libc::c_uint;
            mr0 = 0 as libc::c_int as libc::c_uint;
            tdinit3 = 0 as libc::c_int as libc::c_uint;
            tdinit2 = 0 as libc::c_int as libc::c_uint;
            tdinit1 = 0 as libc::c_int as libc::c_uint;
            tdinit0 = 0 as libc::c_int as libc::c_uint;
        }
    }
    if (trtp as libc::c_uint)
        < tcl
            .wrapping_sub(trp as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint)
    {
        trtp = tcl
            .wrapping_sub(trp as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_uchar;
    }
    trtp = 4 as libc::c_int as libc::c_uchar;
    if (*para).dram_mr0 & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        (*para).dram_mr0 = mr0;
    }
    if (*para).dram_mr1 & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        (*para).dram_mr1 = mr1;
    }
    if (*para).dram_mr2 & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        (*para).dram_mr2 = mr2;
    }
    if (*para).dram_mr3 & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        (*para).dram_mr3 = mr3;
    }
    rv_writel(
        (*para).dram_mr0,
        0x3103030 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        (*para).dram_mr1,
        0x3103034 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        (*para).dram_mr2,
        0x3103038 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        (*para).dram_mr3,
        0x310303c as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        (*para).dram_odt_en >> 4 as libc::c_int & 0x3 as libc::c_int as libc::c_uint,
        0x310302c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    reg_val = twtp << 24 as libc::c_int
        | ((tfaw as libc::c_int) << 16 as libc::c_int) as libc::c_uint
        | trasmax << 8 as libc::c_int
        | ((tras as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    rv_writel(
        reg_val,
        0x3103058 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = ((txp as libc::c_int) << 16 as libc::c_int
        | (trtp as libc::c_int) << 8 as libc::c_int
        | (trc as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    rv_writel(
        reg_val,
        0x310305c as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = tcwl << 24 as libc::c_int
        | tcl << 16 as libc::c_int
        | trd2wr << 8 as libc::c_int
        | twr2rd << 0 as libc::c_int;
    rv_writel(
        reg_val,
        0x3103060 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = tmrw << 16 as libc::c_int | tmrd << 12 as libc::c_int | tmod << 0 as libc::c_int;
    rv_writel(
        reg_val,
        0x3103064 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = ((trcd as libc::c_int) << 24 as libc::c_int
        | (tccd as libc::c_int) << 16 as libc::c_int
        | (trrd as libc::c_int) << 8 as libc::c_int
        | (trp as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    rv_writel(
        reg_val,
        0x3103068 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = tcksrx << 24 as libc::c_int
        | tcksrx << 16 as libc::c_int
        | tckesr << 8 as libc::c_int
        | tcke << 0 as libc::c_int;
    rv_writel(
        reg_val,
        0x310306c as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val =
        rv_readl(0x3103078 as libc::c_int as *mut libc::c_void as intptr_t as *const libc::c_void);
    reg_val &= 0xfff0000 as libc::c_int as libc::c_uint;
    reg_val |= if (*para).dram_clk < 800 as libc::c_int as libc::c_uint {
        0xf0006600 as libc::c_uint
    } else {
        0xf0007600 as libc::c_uint
    };
    reg_val |= 0x10 as libc::c_int as libc::c_uint;
    rv_writel(
        reg_val,
        0x3103078 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = ((0x2 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
        | t_rdata_en << 16 as libc::c_int
        | ((0x1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
        | wr_latency << 0 as libc::c_int;
    rv_writel(
        reg_val,
        0x3103080 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        tdinit0 << 0 as libc::c_int | tdinit1 << 20 as libc::c_int,
        0x3103050 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    rv_writel(
        tdinit2 << 0 as libc::c_int | tdinit3 << 20 as libc::c_int,
        0x3103054 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val = ((trefi as libc::c_int) << 16 as libc::c_int
        | (trfc as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    rv_writel(
        reg_val,
        0x3103090 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
    reg_val =
        (0xfff0000 as libc::c_int & (trefi as libc::c_int) << 15 as libc::c_int) as libc::c_uint;
    rv_writel(
        reg_val,
        0x3103094 as libc::c_int as *mut libc::c_void as intptr_t as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ccm_get_sscg() {}
#[no_mangle]
pub unsafe extern "C" fn ccm_set_pll_sscg() {}
#[no_mangle]
pub unsafe extern "C" fn ccm_set_pll_ddr_clk(
    mut index: libc::c_int,
    mut para: *mut dram_para_t,
) -> libc::c_int {
    let mut val: libc::c_uint = 0;
    let mut clk: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    clk = if (*para).dram_tpr13 & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint != 0 {
        (*para).dram_tpr9
    } else {
        (*para).dram_clk
    };
    n = clk
        .wrapping_mul(2 as libc::c_int as libc::c_uint)
        .wrapping_div(24 as libc::c_int as libc::c_uint);
    val = rv_readl(0x2001010 as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xfff800fc as libc::c_uint;
    val |= n.wrapping_sub(1 as libc::c_int as libc::c_uint) << 8 as libc::c_int;
    val |= 0xc0000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001010 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val &= 0xdfffffff as libc::c_uint;
    val |= 0xc0000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001010 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val |= 0xe0000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001010 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    while rv_readl(0x2001010 as libc::c_int as intptr_t as *const libc::c_void)
        & 0x10000000 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {}
    sdelay(20 as libc::c_int as libc::c_ulong);
    val = rv_readl(0x2001000 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x8000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x2001000 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x2001800 as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xfcfffcfc as libc::c_uint;
    val |= 0x80000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    return n.wrapping_mul(24 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mctl_sys_init(mut para: *mut dram_para_t) {
    let mut val: libc::c_uint = 0;
    val = rv_readl(0x2001540 as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xbfffffff as libc::c_uint;
    rv_writel(
        val,
        0x2001540 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x200180c as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xfffffffe as libc::c_uint;
    rv_writel(
        val,
        0x200180c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val &= 0xfffefffe as libc::c_uint;
    rv_writel(
        val,
        0x200180c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x2001800 as libc::c_int as intptr_t as *const libc::c_void);
    rv_writel(
        val & 0xbfffffff as libc::c_uint,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val &= 0x7fffffff as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val |= 0x8000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
    val = ccm_set_pll_ddr_clk(0 as libc::c_int, para) as libc::c_uint;
    (*para).dram_clk = val >> 1 as libc::c_int;
    sdelay(100 as libc::c_int as libc::c_ulong);
    dram_disable_all_master();
    val = rv_readl(0x200180c as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x10000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x200180c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x2001540 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x40000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x2001540 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x2001800 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x40000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(5 as libc::c_int as libc::c_ulong);
    val = rv_readl(0x200180c as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x1 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x200180c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x2001800 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x80000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val |= 0x88000000 as libc::c_uint;
    rv_writel(
        val,
        0x2001800 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(5 as libc::c_int as libc::c_ulong);
    rv_writel(
        0x8000 as libc::c_int as u32_0,
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x100c as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mctl_com_init(mut para: *mut dram_para_t) {
    let mut val: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    val = rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x8 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) & 0xffffc0ff as libc::c_uint;
    val |= 0x2000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x8 as libc::c_int as isize)
            as intptr_t as *mut libc::c_void,
    );
    val =
        rv_readl(0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void)
            & 0xff000fff as libc::c_uint;
    val |= ((*para).dram_type & 0x7 as libc::c_int as libc::c_uint) << 16 as libc::c_int;
    val |= (!(*para).dram_para2 & 0x1 as libc::c_int as libc::c_uint) << 12 as libc::c_int;
    if (*para).dram_type != 6 as libc::c_int as libc::c_uint
        && (*para).dram_type != 7 as libc::c_int as libc::c_uint
    {
        val |= ((*para).dram_tpr13 >> 5 as libc::c_int & 0x1 as libc::c_int as libc::c_uint)
            << 19 as libc::c_int;
        val |= 0x400000 as libc::c_int as libc::c_uint;
    } else {
        val |= 0x480000 as libc::c_int as libc::c_uint;
    }
    rv_writel(
        val,
        0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *mut libc::c_void,
    );
    val = (*para).dram_para2;
    end = (if val & 0x100 as libc::c_int as libc::c_uint != 0
        && val >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint
            != 1 as libc::c_int as libc::c_uint
    {
        32 as libc::c_int
    } else {
        16 as libc::c_int
    }) as libc::c_uint;
    ptr = 0x3102000 as libc::c_int as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i as libc::c_uint != end {
        val = rv_readl(ptr as intptr_t as *const libc::c_void) & 0xfffff000 as libc::c_uint;
        val |= (*para).dram_para2 >> 12 as libc::c_int & 0x3 as libc::c_int as libc::c_uint;
        val |= ((*para).dram_para1 >> i + 12 as libc::c_int) << 2 as libc::c_int
            & 0x4 as libc::c_int as libc::c_uint;
        val |= ((*para).dram_para1 >> i + 4 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            << 4 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint;
        match (*para).dram_para1 >> i & 0xf as libc::c_int as libc::c_uint {
            8 => {
                val |= 0xa00 as libc::c_int as libc::c_uint;
            }
            4 => {
                val |= 0x900 as libc::c_int as libc::c_uint;
            }
            2 => {
                val |= 0x800 as libc::c_int as libc::c_uint;
            }
            1 => {
                val |= 0x700 as libc::c_int as libc::c_uint;
            }
            _ => {
                val |= 0x600 as libc::c_int as libc::c_uint;
            }
        }
        rv_writel(val, ptr as intptr_t as *mut libc::c_void);
        ptr = ptr.offset(4 as libc::c_int as isize);
        i += 16 as libc::c_int;
    }
    val = (if rv_readl(
        0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void,
    ) & 0x1 as libc::c_int as libc::c_uint
        != 0
    {
        0x303 as libc::c_int
    } else {
        0x201 as libc::c_int
    }) as libc::c_uint;
    rv_writel(
        val,
        0x3103120 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if (*para).dram_para2 & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        rv_writel(
            0 as libc::c_int as u32_0,
            0x31033c4 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    if (*para).dram_tpr4 != 0 {
        val = rv_readl(
            0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void,
        );
        val |= (*para).dram_tpr4 << 25 as libc::c_int & 0x6000000 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *mut libc::c_void,
        );
        val = rv_readl(0x3102004 as libc::c_int as intptr_t as *const libc::c_void);
        val |= ((*para).dram_tpr4 >> 2 as libc::c_int) << 12 as libc::c_int
            & 0x1ff000 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x3102004 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn mctl_phy_ac_remapping(mut para: *mut dram_para_t) {
    let mut cfg0: [libc::c_char; 22] = [
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    static mut cfg1: [libc::c_char; 22] = [
        1 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        14 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
    ];
    static mut cfg2: [libc::c_char; 22] = [
        4 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        14 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
    ];
    static mut cfg3: [libc::c_char; 22] = [
        1 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
    ];
    static mut cfg4: [libc::c_char; 22] = [
        4 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
    ];
    static mut cfg5: [libc::c_char; 22] = [
        13 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
    ];
    static mut cfg6: [libc::c_char; 22] = [
        3 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
    ];
    static mut cfg7: [libc::c_char; 22] = [
        3 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        7 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        17 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        14 as libc::c_int as libc::c_char,
        13 as libc::c_int as libc::c_char,
        8 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        10 as libc::c_int as libc::c_char,
        5 as libc::c_int as libc::c_char,
        19 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
        16 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        11 as libc::c_int as libc::c_char,
    ];
    let mut fuse: libc::c_uint = 0;
    let mut val: libc::c_uint = 0;
    fuse = rv_readl(0x3002228 as libc::c_int as intptr_t as *const libc::c_void)
        >> 8 as libc::c_int
        & 0x4 as libc::c_int as libc::c_uint;
    printf(
        b"ddr_efuse_type: 0x%x\n\0" as *const u8 as *const libc::c_char,
        fuse,
    );
    if (*para).dram_tpr13 >> 18 as libc::c_int & 0x3 as libc::c_int as libc::c_uint != 0 {
        memcpy_self(cfg0.as_mut_ptr(), cfg7.as_mut_ptr(), 22 as libc::c_int);
    } else {
        match fuse {
            8 => {
                memcpy_self(cfg0.as_mut_ptr(), cfg2.as_mut_ptr(), 22 as libc::c_int);
            }
            9 => {
                memcpy_self(cfg0.as_mut_ptr(), cfg3.as_mut_ptr(), 22 as libc::c_int);
            }
            10 => {
                memcpy_self(cfg0.as_mut_ptr(), cfg5.as_mut_ptr(), 22 as libc::c_int);
            }
            11 => {
                memcpy_self(cfg0.as_mut_ptr(), cfg4.as_mut_ptr(), 22 as libc::c_int);
            }
            13 | 14 => {}
            12 | _ => {
                memcpy_self(cfg0.as_mut_ptr(), cfg1.as_mut_ptr(), 22 as libc::c_int);
            }
        }
    }
    if (*para).dram_type == 2 as libc::c_int as libc::c_uint {
        if fuse == 15 as libc::c_int as libc::c_uint {
            return;
        }
        memcpy_self(cfg0.as_mut_ptr(), cfg6.as_mut_ptr(), 22 as libc::c_int);
    }
    if (*para).dram_type == 2 as libc::c_int as libc::c_uint
        || (*para).dram_type == 3 as libc::c_int as libc::c_uint
    {
        val = ((cfg0[4 as libc::c_int as usize] as libc::c_int) << 25 as libc::c_int
            | (cfg0[3 as libc::c_int as usize] as libc::c_int) << 20 as libc::c_int
            | (cfg0[2 as libc::c_int as usize] as libc::c_int) << 15 as libc::c_int
            | (cfg0[1 as libc::c_int as usize] as libc::c_int) << 10 as libc::c_int
            | (cfg0[0 as libc::c_int as usize] as libc::c_int) << 5 as libc::c_int)
            as libc::c_uint;
        rv_writel(
            val,
            0x3102500 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = ((cfg0[10 as libc::c_int as usize] as libc::c_int) << 25 as libc::c_int
            | (cfg0[9 as libc::c_int as usize] as libc::c_int) << 20 as libc::c_int
            | (cfg0[8 as libc::c_int as usize] as libc::c_int) << 15 as libc::c_int
            | (cfg0[7 as libc::c_int as usize] as libc::c_int) << 10 as libc::c_int
            | (cfg0[6 as libc::c_int as usize] as libc::c_int) << 5 as libc::c_int
            | cfg0[5 as libc::c_int as usize] as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x3102504 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = ((cfg0[15 as libc::c_int as usize] as libc::c_int) << 20 as libc::c_int
            | (cfg0[14 as libc::c_int as usize] as libc::c_int) << 15 as libc::c_int
            | (cfg0[13 as libc::c_int as usize] as libc::c_int) << 10 as libc::c_int
            | (cfg0[12 as libc::c_int as usize] as libc::c_int) << 5 as libc::c_int
            | cfg0[11 as libc::c_int as usize] as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x3102508 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = ((cfg0[21 as libc::c_int as usize] as libc::c_int) << 25 as libc::c_int
            | (cfg0[20 as libc::c_int as usize] as libc::c_int) << 20 as libc::c_int
            | (cfg0[19 as libc::c_int as usize] as libc::c_int) << 15 as libc::c_int
            | (cfg0[18 as libc::c_int as usize] as libc::c_int) << 10 as libc::c_int
            | (cfg0[17 as libc::c_int as usize] as libc::c_int) << 5 as libc::c_int
            | cfg0[16 as libc::c_int as usize] as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x310250c as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = ((cfg0[4 as libc::c_int as usize] as libc::c_int) << 25 as libc::c_int
            | (cfg0[3 as libc::c_int as usize] as libc::c_int) << 20 as libc::c_int
            | (cfg0[2 as libc::c_int as usize] as libc::c_int) << 15 as libc::c_int
            | (cfg0[1 as libc::c_int as usize] as libc::c_int) << 10 as libc::c_int
            | (cfg0[0 as libc::c_int as usize] as libc::c_int) << 5 as libc::c_int
            | 1 as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x3102500 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn mctl_channel_init(
    mut ch_index: libc::c_uint,
    mut para: *mut dram_para_t,
) -> libc::c_uint {
    let mut val: libc::c_uint = 0;
    let mut dqs_gating_mode: libc::c_uint = 0;
    dqs_gating_mode = (*para).dram_tpr13 >> 2 as libc::c_int & 0x3 as libc::c_int as libc::c_uint;
    val = rv_readl(0x310200c as libc::c_int as intptr_t as *const libc::c_void)
        & 0xfffff000 as libc::c_uint;
    val |= ((*para).dram_clk >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    rv_writel(
        val,
        0x310200c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103108 as libc::c_int as intptr_t as *const libc::c_void)
        & 0xfffff0ff as libc::c_uint;
    val |= 0x300 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103344 as libc::c_int as intptr_t as *const libc::c_void)
        & 0xffffffcf as libc::c_uint;
    val |= !(*para).dram_odt_en << 5 as libc::c_int & 0x20 as libc::c_int as libc::c_uint;
    if (*para).dram_clk > 672 as libc::c_int as libc::c_uint {
        val &= 0xffff09f1 as libc::c_uint;
        val |= 0x400 as libc::c_int as libc::c_uint;
    } else {
        val &= 0xffff0ff1 as libc::c_uint;
    }
    rv_writel(
        val,
        0x3103344 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x31033c4 as libc::c_int as intptr_t as *const libc::c_void)
        & 0xffffffcf as libc::c_uint;
    val |= !(*para).dram_odt_en << 5 as libc::c_int & 0x20 as libc::c_int as libc::c_uint;
    if (*para).dram_clk > 672 as libc::c_int as libc::c_uint {
        val &= 0xffff09f1 as libc::c_uint;
        val |= 0x400 as libc::c_int as libc::c_uint;
    } else {
        val &= 0xffff0ff1 as libc::c_uint;
    }
    rv_writel(
        val,
        0x31033c4 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val = rv_readl(0x3103208 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x2 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103208 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    eye_delay_compensation(para);
    val = rv_readl(0x3103108 as libc::c_int as intptr_t as *const libc::c_void);
    if dqs_gating_mode == 1 as libc::c_int as libc::c_uint {
        val &= !(0xc0 as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = rv_readl(0x31030bc as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xfffffef8 as libc::c_uint;
        rv_writel(
            val,
            0x31030bc as libc::c_int as intptr_t as *mut libc::c_void,
        );
    } else if dqs_gating_mode == 2 as libc::c_int as libc::c_uint {
        val &= !(0xc0 as libc::c_int) as libc::c_uint;
        val |= 0x80 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = rv_readl(0x31030bc as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xfffffef8 as libc::c_uint;
        val |= ((*para).dram_tpr13 >> 16 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            .wrapping_sub(2 as libc::c_int as libc::c_uint);
        val |= 0x100 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x31030bc as libc::c_int as intptr_t as *mut libc::c_void,
        );
        val = rv_readl(0x310311c as libc::c_int as intptr_t as *const libc::c_void)
            & 0x7fffffff as libc::c_int as libc::c_uint;
        val |= 0x8000000 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x310311c as libc::c_int as intptr_t as *mut libc::c_void,
        );
    } else {
        val &= !(0x40 as libc::c_int) as libc::c_uint;
        rv_writel(
            val,
            0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
        val = rv_readl(0x3103108 as libc::c_int as intptr_t as *const libc::c_void);
        val |= 0xc0 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    if (*para).dram_type == 6 as libc::c_int as libc::c_uint
        || (*para).dram_type == 7 as libc::c_int as libc::c_uint
    {
        val = rv_readl(0x310311c as libc::c_int as intptr_t as *const libc::c_void);
        if dqs_gating_mode == 1 as libc::c_int as libc::c_uint {
            val &= 0xf7ffff3f as libc::c_uint;
            val |= 0x80000000 as libc::c_uint;
        } else {
            val &= 0x88ffffff as libc::c_uint;
            val |= 0x22000000 as libc::c_int as libc::c_uint;
        }
        rv_writel(
            val,
            0x310311c as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    val = rv_readl(0x31030c0 as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xf0000000 as libc::c_uint;
    val |= (if (*para).dram_para2 & ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint != 0 {
        0x3000001 as libc::c_int
    } else {
        0x1000007 as libc::c_int
    }) as libc::c_uint;
    rv_writel(
        val,
        0x31030c0 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if rv_readl(0x70005d4 as libc::c_int as intptr_t as *const libc::c_void)
        & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
        != 0
    {
        val = rv_readl(0x7010250 as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xfffffffd as libc::c_uint;
        rv_writel(
            val,
            0x7010250 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
    }
    val = rv_readl(0x3103140 as libc::c_int as intptr_t as *const libc::c_void)
        & 0xfc000000 as libc::c_uint;
    val |= (*para).dram_zq & 0xffffff as libc::c_int as libc::c_uint;
    val |= 0x2000000 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103140 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if dqs_gating_mode == 1 as libc::c_int as libc::c_uint {
        rv_writel(
            0x52 as libc::c_int as u32_0,
            0x3103000 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            0x53 as libc::c_int as u32_0,
            0x3103000 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        while rv_readl(0x3103010 as libc::c_int as intptr_t as *const libc::c_void)
            & 0x1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {}
        sdelay(10 as libc::c_int as libc::c_ulong);
        val = (if (*para).dram_type == 3 as libc::c_int as libc::c_uint {
            0x5a0 as libc::c_int
        } else {
            0x520 as libc::c_int
        }) as libc::c_uint;
    } else if rv_readl(0x70005d4 as libc::c_int as intptr_t as *const libc::c_void)
        & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        val = (if (*para).dram_type == 3 as libc::c_int as libc::c_uint {
            0x1f2 as libc::c_int
        } else {
            0x172 as libc::c_int
        }) as libc::c_uint;
    } else {
        val = 0x62 as libc::c_int as libc::c_uint;
    }
    rv_writel(
        val,
        0x3103000 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    val |= 1 as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x3103000 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
    while rv_readl(0x3103010 as libc::c_int as intptr_t as *const libc::c_void)
        & 0x1 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {}
    if rv_readl(0x70005d4 as libc::c_int as intptr_t as *const libc::c_void)
        & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
        != 0
    {
        val = rv_readl(0x310310c as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xf9ffffff as libc::c_uint;
        val |= 0x4000000 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x310310c as libc::c_int as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
        val = rv_readl(0x3103004 as libc::c_int as intptr_t as *const libc::c_void);
        val |= 0x1 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x3103004 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        while rv_readl(0x3103018 as libc::c_int as intptr_t as *const libc::c_void)
            & 0x7 as libc::c_int as libc::c_uint
            != 0x3 as libc::c_int as libc::c_uint
        {}
        val = rv_readl(0x7010250 as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xfffffffe as libc::c_uint;
        rv_writel(
            val,
            0x7010250 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
        val = rv_readl(0x3103004 as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xfffffffe as libc::c_uint;
        rv_writel(
            val,
            0x3103004 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        while rv_readl(0x3103018 as libc::c_int as intptr_t as *const libc::c_void)
            & 0x7 as libc::c_int as libc::c_uint
            != 0x1 as libc::c_int as libc::c_uint
        {}
        sdelay(15 as libc::c_int as libc::c_ulong);
        if dqs_gating_mode == 1 as libc::c_int as libc::c_uint {
            val = rv_readl(0x3103108 as libc::c_int as intptr_t as *const libc::c_void);
            val &= 0xffffff3f as libc::c_uint;
            rv_writel(
                val,
                0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
            );
            val = rv_readl(0x310310c as libc::c_int as intptr_t as *const libc::c_void);
            val &= 0xf9ffffff as libc::c_uint;
            val |= 0x2000000 as libc::c_int as libc::c_uint;
            rv_writel(
                val,
                0x310310c as libc::c_int as intptr_t as *mut libc::c_void,
            );
            sdelay(1 as libc::c_int as libc::c_ulong);
            rv_writel(
                0x401 as libc::c_int as u32_0,
                0x3103000 as libc::c_int as intptr_t as *mut libc::c_void,
            );
            while rv_readl(0x3103010 as libc::c_int as intptr_t as *const libc::c_void)
                & 0x1 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {}
        }
    }
    val = rv_readl(0x3103010 as libc::c_int as intptr_t as *const libc::c_void);
    if val >> 20 as libc::c_int & 0xff as libc::c_int as libc::c_uint != 0
        && val & 0x100000 as libc::c_int as libc::c_uint != 0
    {
        printf(
            b"ZQ calibration error, check external 240 ohm resistor.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int as libc::c_uint;
    }
    while rv_readl(0x3103018 as libc::c_int as intptr_t as *const libc::c_void)
        & 0x1 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {}
    val = rv_readl(0x310308c as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x80000000 as libc::c_uint;
    rv_writel(
        val,
        0x310308c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
    val = rv_readl(0x310308c as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0x7fffffff as libc::c_int as libc::c_uint;
    rv_writel(
        val,
        0x310308c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
    val = rv_readl(0x3102014 as libc::c_int as intptr_t as *const libc::c_void);
    val |= 0x80000000 as libc::c_uint;
    rv_writel(
        val,
        0x3102014 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    sdelay(10 as libc::c_int as libc::c_ulong);
    val = rv_readl(0x310310c as libc::c_int as intptr_t as *const libc::c_void);
    val &= 0xf9ffffff as libc::c_uint;
    rv_writel(
        val,
        0x310310c as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if dqs_gating_mode == 1 as libc::c_int as libc::c_uint {
        val = rv_readl(0x310311c as libc::c_int as intptr_t as *const libc::c_void);
        val &= 0xffffff3f as libc::c_uint;
        val |= 0x40 as libc::c_int as libc::c_uint;
        rv_writel(
            val,
            0x310311c as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    return 1 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn DRAMC_get_dram_size() -> libc::c_int {
    let mut rval: libc::c_uint = 0;
    let mut temp: libc::c_uint = 0;
    let mut size0: libc::c_uint = 0;
    let mut size1: libc::c_uint = 0;
    rval =
        rv_readl(0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void);
    temp = rval >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
    temp = temp.wrapping_add(rval >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint);
    temp = temp.wrapping_add(rval >> 2 as libc::c_int & 0x3 as libc::c_int as libc::c_uint);
    temp = temp.wrapping_sub(14 as libc::c_int as libc::c_uint);
    size0 = ((1 as libc::c_int) << temp) as libc::c_uint;
    temp = rval & 0x3 as libc::c_int as libc::c_uint;
    if temp == 0 as libc::c_int as libc::c_uint {
        return size0 as libc::c_int;
    }
    rval = rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x4 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    );
    temp = rval & 0x3 as libc::c_int as libc::c_uint;
    if temp == 0 as libc::c_int as libc::c_uint {
        return (2 as libc::c_int as libc::c_uint).wrapping_mul(size0) as libc::c_int;
    }
    temp = rval >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
    temp = temp.wrapping_add(rval >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint);
    temp = temp.wrapping_add(rval >> 2 as libc::c_int & 0x3 as libc::c_int as libc::c_uint);
    temp = temp.wrapping_sub(14 as libc::c_int as libc::c_uint);
    size1 = ((1 as libc::c_int) << temp) as libc::c_uint;
    return size0.wrapping_add(size1) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dqs_gate_detect(mut para: *mut dram_para_t) -> libc::c_int {
    let mut rval: libc::c_uint = 0;
    let mut dx0: libc::c_uint = 0;
    let mut dx1: libc::c_uint = 0;
    if rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x1010 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) & ((1 as libc::c_int) << 22 as libc::c_int) as libc::c_uint
        != 0
    {
        dx0 = rv_readl(
            (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x1348 as libc::c_int as isize)
                as intptr_t as *const libc::c_void,
        ) >> 24 as libc::c_int
            & 0x3 as libc::c_int as libc::c_uint;
        dx1 = rv_readl(
            (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x13c8 as libc::c_int as isize)
                as intptr_t as *const libc::c_void,
        ) >> 24 as libc::c_int
            & 0x3 as libc::c_int as libc::c_uint;
        if dx0 == 2 as libc::c_int as libc::c_uint {
            rval = (*para).dram_para2;
            rval &= 0xffff0ff0 as libc::c_uint;
            if dx0 != dx1 {
                rval |= 0x1 as libc::c_int as libc::c_uint;
                (*para).dram_para2 = rval;
                printf(
                    b"[AUTO DEBUG] single rank and half DQ!\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*para).dram_para2 = rval;
            printf(
                b"[AUTO DEBUG] single rank and full DQ!\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        } else if dx0 == 0 as libc::c_int as libc::c_uint {
            rval = (*para).dram_para2;
            rval &= 0xfffffff0 as libc::c_uint;
            rval |= 0x1001 as libc::c_int as libc::c_uint;
            (*para).dram_para2 = rval;
            printf(b"[AUTO DEBUG] dual rank and half DQ!\n\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        } else {
            if (*para).dram_tpr13 & ((1 as libc::c_int) << 29 as libc::c_int) as libc::c_uint != 0 {
                printf(b"DX0 state:%d\n\0" as *const u8 as *const libc::c_char, dx0);
                printf(b"DX1 state:%d\n\0" as *const u8 as *const libc::c_char, dx1);
            }
            return 0 as libc::c_int;
        }
    } else {
        rval = (*para).dram_para2;
        rval &= 0xfffffff0 as libc::c_uint;
        rval |= 0x1000 as libc::c_int as libc::c_uint;
        (*para).dram_para2 = rval;
        printf(b"[AUTO DEBUG] two rank and full DQ!\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dramc_simple_wr_test(
    mut mem_mb: libc::c_uint,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut offs: libc::c_uint = (mem_mb >> 1 as libc::c_int) << 18 as libc::c_int;
    let mut patt1: libc::c_uint = 0x1234567 as libc::c_int as libc::c_uint;
    let mut patt2: libc::c_uint = 0xfedcba98 as libc::c_uint;
    let mut addr: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut v1: libc::c_uint = 0;
    let mut v2: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    addr = 0x40000000 as libc::c_int as *mut libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i != len as libc::c_uint {
        rv_writel(patt1.wrapping_add(i), addr as intptr_t as *mut libc::c_void);
        rv_writel(
            patt2.wrapping_add(i),
            addr.offset(offs as isize) as intptr_t as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        addr = addr.offset(1);
    }
    addr = 0x40000000 as libc::c_int as *mut libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i != len as libc::c_uint {
        v1 = rv_readl(addr.offset(i as isize) as intptr_t as *const libc::c_void);
        v2 = patt1.wrapping_add(i);
        if v1 != v2 {
            printf(b"DRAM simple test FAIL.\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"%x != %x at address %x\n\0" as *const u8 as *const libc::c_char,
                v1,
                v2,
                addr.offset(i as isize),
            );
            return 1 as libc::c_int;
        }
        v1 = rv_readl(
            addr.offset(offs as isize).offset(i as isize) as intptr_t as *const libc::c_void
        );
        v2 = patt2.wrapping_add(i);
        if v1 != v2 {
            printf(b"DRAM simple test FAIL.\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"%x != %x at address %x\n\0" as *const u8 as *const libc::c_char,
                v1,
                v2,
                addr.offset(offs as isize).offset(i as isize),
            );
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    printf(b"DRAM simple test OK.\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mctl_vrefzq_init(mut para: *mut dram_para_t) {
    let mut val: libc::c_uint = 0;
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        val = rv_readl(
            (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x1110 as libc::c_int as isize)
                as intptr_t as *const libc::c_void,
        ) & 0x80808080 as libc::c_uint;
        val |= (*para).dram_tpr5;
        rv_writel(
            val,
            (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x1110 as libc::c_int as isize)
                as intptr_t as *mut libc::c_void,
        );
        if (*para).dram_tpr13 & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            val = rv_readl(
                (0x3102000 as libc::c_int as *mut libc::c_char)
                    .offset(0x1114 as libc::c_int as isize) as intptr_t
                    as *const libc::c_void,
            ) & 0xffffff80 as libc::c_uint;
            val |= (*para).dram_tpr6 & 0x7f as libc::c_int as libc::c_uint;
            rv_writel(
                val,
                (0x3102000 as libc::c_int as *mut libc::c_char)
                    .offset(0x1114 as libc::c_int as isize) as intptr_t
                    as *mut libc::c_void,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mctl_core_init(mut para: *mut dram_para_t) -> libc::c_int {
    mctl_sys_init(para);
    mctl_vrefzq_init(para);
    mctl_com_init(para);
    mctl_phy_ac_remapping(para);
    auto_set_timing_para(para);
    return mctl_channel_init(0 as libc::c_int as libc::c_uint, para) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn auto_scan_dram_size(mut para: *mut dram_para_t) -> libc::c_int {
    let mut rval: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rank: libc::c_uint = 0;
    let mut maxrank: libc::c_uint = 0;
    let mut offs: libc::c_uint = 0;
    let mut mc_work_mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chk: libc::c_uint = 0;
    let mut ptr: libc::c_uint = 0;
    let mut shft: libc::c_uint = 0;
    let mut banks: libc::c_uint = 0;
    if mctl_core_init(para) == 0 as libc::c_int {
        printf(
            b"[ERROR DEBUG] DRAM initialisation error : 0!\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    maxrank = (if (*para).dram_para2 & 0xf000 as libc::c_int as libc::c_uint != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_uint;
    mc_work_mode = 0x3102000 as libc::c_int as *mut libc::c_char;
    offs = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    ptr = 0x40000000 as libc::c_int as libc::c_uint;
    while i < 64 as libc::c_int as libc::c_uint {
        rv_writel(
            if i & 1 as libc::c_int as libc::c_uint != 0 {
                ptr
            } else {
                !ptr
            },
            ptr as intptr_t as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    rank = 0 as libc::c_int as libc::c_uint;
    while rank < maxrank {
        rval = rv_readl(mc_work_mode as intptr_t as *const libc::c_void);
        rval &= 0xfffff0f3 as libc::c_uint;
        rval |= 0x6f0 as libc::c_int as libc::c_uint;
        rv_writel(rval, mc_work_mode as intptr_t as *mut libc::c_void);
        while rv_readl(mc_work_mode as intptr_t as *const libc::c_void) != rval {}
        i = 11 as libc::c_int as libc::c_uint;
        's_96: while i < 17 as libc::c_int as libc::c_uint {
            chk = (0x40000000 as libc::c_int
                + ((1 as libc::c_int) << i.wrapping_add(11 as libc::c_int as libc::c_uint)))
                as libc::c_uint;
            ptr = 0x40000000 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int as libc::c_uint;
            loop {
                if !(j < 64 as libc::c_int as libc::c_uint) {
                    break 's_96;
                }
                if rv_readl(chk as intptr_t as *const libc::c_void)
                    != (if j & 1 as libc::c_int as libc::c_uint != 0 {
                        ptr
                    } else {
                        !ptr
                    })
                {
                    break;
                }
                ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
                chk = chk.wrapping_add(4 as libc::c_int as libc::c_uint);
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if i > 16 as libc::c_int as libc::c_uint {
            i = 16 as libc::c_int as libc::c_uint;
        }
        printf(
            b"[AUTO DEBUG] rank %d row = %d\n\0" as *const u8 as *const libc::c_char,
            rank,
            i,
        );
        shft = (4 as libc::c_int as libc::c_uint).wrapping_add(offs);
        rval = (*para).dram_para1;
        rval &= !((0xff as libc::c_int) << shft) as libc::c_uint;
        rval |= i << shft;
        (*para).dram_para1 = rval;
        if rank == 1 as libc::c_int as libc::c_uint {
            rval = rv_readl(
                0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void,
            );
            rval &= 0xfffff003 as libc::c_uint;
            rval |= 0x6a4 as libc::c_int as libc::c_uint;
            rv_writel(
                rval,
                0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *mut libc::c_void,
            );
        }
        rval = rv_readl(mc_work_mode as intptr_t as *const libc::c_void);
        rval &= 0xfffff003 as libc::c_uint;
        rval |= 0x6a4 as libc::c_int as libc::c_uint;
        rv_writel(rval, mc_work_mode as intptr_t as *mut libc::c_void);
        while rv_readl(mc_work_mode as intptr_t as *const libc::c_void) != rval {}
        chk =
            (0x40000000 as libc::c_int + ((1 as libc::c_int) << 22 as libc::c_int)) as libc::c_uint;
        ptr = 0x40000000 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int as libc::c_uint;
        j = 0 as libc::c_int as libc::c_uint;
        while i < 64 as libc::c_int as libc::c_uint {
            if rv_readl(chk as intptr_t as *const libc::c_void)
                != (if i & 1 as libc::c_int as libc::c_uint != 0 {
                    ptr
                } else {
                    !ptr
                })
            {
                j = 1 as libc::c_int as libc::c_uint;
                break;
            } else {
                ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
                chk = chk.wrapping_add(4 as libc::c_int as libc::c_uint);
                i = i.wrapping_add(1);
            }
        }
        banks = j.wrapping_add(1 as libc::c_int as libc::c_uint) << 2 as libc::c_int;
        printf(
            b"[AUTO DEBUG] rank %d bank = %d\n\0" as *const u8 as *const libc::c_char,
            rank,
            banks,
        );
        shft = (12 as libc::c_int as libc::c_uint).wrapping_add(offs);
        rval = (*para).dram_para1;
        rval &= !((0xf as libc::c_int) << shft) as libc::c_uint;
        rval |= j << shft;
        (*para).dram_para1 = rval;
        if rank == 1 as libc::c_int as libc::c_uint {
            rval = rv_readl(
                0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *const libc::c_void,
            );
            rval &= 0xfffff003 as libc::c_uint;
            rval |= 0xaa0 as libc::c_int as libc::c_uint;
            rv_writel(
                rval,
                0x3102000 as libc::c_int as *mut libc::c_char as intptr_t as *mut libc::c_void,
            );
        }
        rval = rv_readl(mc_work_mode as intptr_t as *const libc::c_void);
        rval &= 0xfffff003 as libc::c_uint;
        rval |= 0xaa0 as libc::c_int as libc::c_uint;
        rv_writel(rval, mc_work_mode as intptr_t as *mut libc::c_void);
        while rv_readl(mc_work_mode as intptr_t as *const libc::c_void) != rval {}
        i = 9 as libc::c_int as libc::c_uint;
        's_364: while i < 14 as libc::c_int as libc::c_uint {
            chk = (0x40000000 as libc::c_int + ((1 as libc::c_int) << i)) as libc::c_uint;
            ptr = 0x40000000 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int as libc::c_uint;
            loop {
                if !(j < 64 as libc::c_int as libc::c_uint) {
                    break 's_364;
                }
                if rv_readl(chk as intptr_t as *const libc::c_void)
                    != (if j & 1 as libc::c_int as libc::c_uint != 0 {
                        ptr
                    } else {
                        !ptr
                    })
                {
                    break;
                }
                ptr = ptr.wrapping_add(4 as libc::c_int as libc::c_uint);
                chk = chk.wrapping_add(4 as libc::c_int as libc::c_uint);
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if i > 13 as libc::c_int as libc::c_uint {
            i = 13 as libc::c_int as libc::c_uint;
        }
        let mut pgsize: libc::c_int = if i == 9 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << i.wrapping_sub(10 as libc::c_int as libc::c_uint)
        };
        printf(
            b"[AUTO DEBUG] rank %d page size = %d KB\n\0" as *const u8 as *const libc::c_char,
            rank,
            pgsize,
        );
        shft = offs;
        rval = (*para).dram_para1;
        rval &= !((0xf as libc::c_int) << shft) as libc::c_uint;
        rval |= (pgsize << shft) as libc::c_uint;
        (*para).dram_para1 = rval;
        rank = rank.wrapping_add(1);
        if rank != maxrank {
            if rank == 1 as libc::c_int as libc::c_uint {
                rval = rv_readl(
                    (0x3102000 as libc::c_int as *mut libc::c_char)
                        .offset(0x100000 as libc::c_int as isize) as intptr_t
                        as *const libc::c_void,
                );
                rval &= 0xfffff003 as libc::c_uint;
                rval |= 0x6f0 as libc::c_int as libc::c_uint;
                rv_writel(
                    rval,
                    (0x3102000 as libc::c_int as *mut libc::c_char)
                        .offset(0x100000 as libc::c_int as isize) as intptr_t
                        as *mut libc::c_void,
                );
                rval = rv_readl(
                    (0x3102000 as libc::c_int as *mut libc::c_char)
                        .offset(0x100004 as libc::c_int as isize) as intptr_t
                        as *const libc::c_void,
                );
                rval &= 0xfffff003 as libc::c_uint;
                rval |= 0x6f0 as libc::c_int as libc::c_uint;
                rv_writel(
                    rval,
                    (0x3102000 as libc::c_int as *mut libc::c_char)
                        .offset(0x100004 as libc::c_int as isize) as intptr_t
                        as *mut libc::c_void,
                );
            }
            offs = offs.wrapping_add(16 as libc::c_int as libc::c_uint);
            mc_work_mode = mc_work_mode.offset(4 as libc::c_int as isize);
        }
    }
    if maxrank == 2 as libc::c_int as libc::c_uint {
        (*para).dram_para2 &= 0xfffff0ff as libc::c_uint;
        if rval & 0xffff as libc::c_int as libc::c_uint
            == rval >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint
        {
            printf(b"rank1 config same as rank0\n\0" as *const u8 as *const libc::c_char);
        } else {
            (*para).dram_para2 |= 0x100 as libc::c_int as libc::c_uint;
            printf(b"rank1 config different from rank0\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn auto_scan_dram_rank_width(mut para: *mut dram_para_t) -> libc::c_int {
    let mut s1: libc::c_uint = (*para).dram_tpr13;
    let mut s2: libc::c_uint = (*para).dram_para1;
    let mut v: libc::c_uint = 0;
    (*para).dram_para1 = 0xb000b0 as libc::c_int as libc::c_uint;
    v = (*para).dram_para2 & 0xfffffff0 as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint;
    (*para).dram_para2 = v;
    v = s1 & 0xfffffff7 as libc::c_uint | 0x5 as libc::c_int as libc::c_uint;
    (*para).dram_tpr13 = v;
    mctl_core_init(para);
    if rv_readl(
        (0x3102000 as libc::c_int as *mut libc::c_char).offset(0x1010 as libc::c_int as isize)
            as intptr_t as *const libc::c_void,
    ) & ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
        != 0
    {
        return 0 as libc::c_int;
    }
    if dqs_gate_detect(para) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*para).dram_tpr13 = s1;
    (*para).dram_para1 = s2;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn auto_scan_dram_config(mut para: *mut dram_para_t) -> libc::c_int {
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        && auto_scan_dram_rank_width(para) == 0 as libc::c_int
    {
        printf(
            b"[ERROR DEBUG] auto scan dram rank & width failed !\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        && auto_scan_dram_size(para) == 0 as libc::c_int
    {
        printf(
            b"[ERROR DEBUG] auto scan dram size failed !\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        (*para).dram_tpr13 |= 0x6003 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_DRAM(
    mut type_0: libc::c_int,
    mut para: *mut dram_para_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut mem_size: libc::c_int = 0;
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint != 0 {
        printf(b"DRAM only have internal ZQ!!\n\0" as *const u8 as *const libc::c_char);
        rv_writel(
            rv_readl(
                (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                    as intptr_t as *const libc::c_void,
            ) | 0x100 as libc::c_int as libc::c_uint,
            (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            0 as libc::c_int as u32_0,
            (0x3000000 as libc::c_int as *mut libc::c_char).offset(0x16e as libc::c_int as isize)
                as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
    } else {
        rv_writel(
            0 as libc::c_int as u32_0,
            (0x7010000 as libc::c_int as *mut libc::c_char).offset(0x254 as libc::c_int as isize)
                as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            rv_readl(
                (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                    as intptr_t as *const libc::c_void,
            ) & !(0x3 as libc::c_int) as libc::c_uint,
            (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
        rv_writel(
            rv_readl(
                (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                    as intptr_t as *const libc::c_void,
            ) & !(0x108 as libc::c_int) as libc::c_uint,
            (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                as intptr_t as *mut libc::c_void,
        );
        sdelay(10 as libc::c_int as libc::c_ulong);
        rv_writel(
            rv_readl(
                (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                    as intptr_t as *const libc::c_void,
            ) | 0x1 as libc::c_int as libc::c_uint,
            (0x7010000 as libc::c_uint).wrapping_add(0x310 as libc::c_int as libc::c_uint)
                as intptr_t as *mut libc::c_void,
        );
        sdelay(20 as libc::c_int as libc::c_ulong);
        printf(
            b"ZQ value = 0x%x***********\0" as *const u8 as *const libc::c_char,
            rv_readl(
                (0x3000000 as libc::c_int as *mut libc::c_char)
                    .offset(0x172 as libc::c_int as isize) as intptr_t
                    as *const libc::c_void,
            ),
        );
    }
    rc = get_pmu_exists();
    printf(
        b"get_pmu_exist() = %d\n\0" as *const u8 as *const libc::c_char,
        rc,
    );
    if rc < 0 as libc::c_int {
        dram_vol_set(para);
    } else if (*para).dram_type == 2 as libc::c_int as libc::c_uint {
        set_ddr_voltage(1800 as libc::c_int);
    } else if (*para).dram_type == 3 as libc::c_int as libc::c_uint {
        set_ddr_voltage(1500 as libc::c_int);
    }
    if (*para).dram_tpr13 & 0x1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if auto_scan_dram_config(para) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    printf(
        b"DRAM BOOT DRIVE INFO: %s\n\0" as *const u8 as *const libc::c_char,
        b"V0.24\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"DRAM CLK = %d MHz\n\0" as *const u8 as *const libc::c_char,
        (*para).dram_clk,
    );
    printf(
        b"DRAM Type = %d (2:DDR2,3:DDR3)\n\0" as *const u8 as *const libc::c_char,
        (*para).dram_type,
    );
    if (*para).dram_odt_en & 0x1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        printf(b"DRAMC read ODT  off.\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"DRAMC ZQ value: 0x%x\n\0" as *const u8 as *const libc::c_char,
            (*para).dram_zq,
        );
    }
    rc = (*para).dram_mr1 as libc::c_int;
    if rc & 0x44 as libc::c_int == 0 as libc::c_int {
        printf(b"DRAM ODT off.\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"DRAM ODT value: 0x%x.\n\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    if mctl_core_init(para) == 0 as libc::c_int {
        printf(b"DRAM initialisation error : 1 !\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    rc = (*para).dram_para2 as libc::c_int;
    if rc < 0 as libc::c_int {
        rc =
            ((rc as libc::c_uint & 0x7fff0000 as libc::c_uint) >> 16 as libc::c_int) as libc::c_int;
    } else {
        rc = DRAMC_get_dram_size();
        printf(
            b"DRAM SIZE =%d M\n\0" as *const u8 as *const libc::c_char,
            rc,
        );
        (*para).dram_para2 =
            (*para).dram_para2 & 0xffff as libc::c_uint | (rc << 16 as libc::c_int) as libc::c_uint;
    }
    mem_size = rc;
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_uint != 0 {
        rc = rv_readl(
            &mut (*para).dram_tpr8 as *mut libc::c_uint as intptr_t as *const libc::c_void,
        ) as libc::c_int;
        if rc == 0 as libc::c_int {
            rc = 0x10000200 as libc::c_int;
        }
        rv_writel(
            rc as u32_0,
            0x31030a0 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            0x40a as libc::c_int as u32_0,
            0x310309c as libc::c_int as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            rv_readl(0x3103004 as libc::c_int as intptr_t as *const libc::c_void)
                | 1 as libc::c_int as libc::c_uint,
            0x3103004 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        printf(b"Enable Auto SR\0" as *const u8 as *const libc::c_char);
    } else {
        rv_writel(
            rv_readl(0x31030a0 as libc::c_int as intptr_t as *const libc::c_void)
                & 0xffff0000 as libc::c_uint,
            0x31030a0 as libc::c_int as intptr_t as *mut libc::c_void,
        );
        rv_writel(
            rv_readl(0x3103004 as libc::c_int as intptr_t as *const libc::c_void)
                & !(0x1 as libc::c_int) as libc::c_uint,
            0x3103004 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    rc = (rv_readl(0x3103100 as libc::c_int as intptr_t as *const libc::c_void)
        & !(0xf000 as libc::c_int) as libc::c_uint) as libc::c_int;
    if (*para).dram_tpr13 & 0x200 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        if (*para).dram_type != 6 as libc::c_int as libc::c_uint {
            rv_writel(
                rc as u32_0,
                0x3103100 as libc::c_int as intptr_t as *mut libc::c_void,
            );
        }
    } else {
        rv_writel(
            (rc | 0x5000 as libc::c_int) as u32_0,
            0x3103100 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    rv_writel(
        rv_readl(0x3103140 as libc::c_int as intptr_t as *const libc::c_void)
            | ((1 as libc::c_int) << 31 as libc::c_int) as libc::c_uint,
        0x3103140 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0 {
        rv_writel(
            rv_readl(0x3103140 as libc::c_int as intptr_t as *const libc::c_void)
                | 0x300 as libc::c_int as libc::c_uint,
            0x31030b8 as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    rc = rv_readl(0x3103108 as libc::c_int as intptr_t as *const libc::c_void) as libc::c_int;
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint != 0 {
        rc = (rc as libc::c_uint & 0xffffdfff as libc::c_uint) as libc::c_int;
    } else {
        rc |= 0x2000 as libc::c_int;
    }
    rv_writel(
        rc as u32_0,
        0x3103108 as libc::c_int as intptr_t as *mut libc::c_void,
    );
    if (*para).dram_type == 7 as libc::c_int as libc::c_uint {
        rc = (rv_readl(0x310307c as libc::c_int as intptr_t as *const libc::c_void)
            & 0xfff0ffff as libc::c_uint) as libc::c_int;
        rc |= 0x1000 as libc::c_int;
        rv_writel(
            rc as u32_0,
            0x310307c as libc::c_int as intptr_t as *mut libc::c_void,
        );
    }
    dram_enable_all_master();
    if (*para).dram_tpr13 & ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint != 0 {
        rc = rv_readl(0x70005d4 as libc::c_int as intptr_t as *const libc::c_void) as libc::c_int;
        if rc & (1 as libc::c_int) << 16 as libc::c_int != 0
            || dramc_simple_wr_test(mem_size as libc::c_uint, 4096 as libc::c_int) != 0
        {
            return 0 as libc::c_int;
        }
    }
    handler_super_standby();
    return mem_size;
}
#[no_mangle]
pub unsafe extern "C" fn mctl_init() -> libc::c_uint {
    let mut ret_val: libc::c_int = 0 as libc::c_int;
    let mut dram_para: dram_para_t = dram_para_t {
        dram_clk: 0,
        dram_type: 0,
        dram_zq: 0,
        dram_odt_en: 0,
        dram_para1: 0,
        dram_para2: 0,
        dram_mr0: 0,
        dram_mr1: 0,
        dram_mr2: 0,
        dram_mr3: 0,
        dram_tpr0: 0,
        dram_tpr1: 0,
        dram_tpr2: 0,
        dram_tpr3: 0,
        dram_tpr4: 0,
        dram_tpr5: 0,
        dram_tpr6: 0,
        dram_tpr7: 0,
        dram_tpr8: 0,
        dram_tpr9: 0,
        dram_tpr10: 0,
        dram_tpr11: 0,
        dram_tpr12: 0,
        dram_tpr13: 0,
    };
    dram_para.dram_clk = 528 as libc::c_int as libc::c_uint;
    dram_para.dram_type = 2 as libc::c_int as libc::c_uint;
    dram_para.dram_zq = 0x7b7bf9 as libc::c_int as libc::c_uint;
    dram_para.dram_odt_en = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_para1 = 210 as libc::c_int as libc::c_uint;
    dram_para.dram_para2 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_mr0 = 0xe73 as libc::c_int as libc::c_uint;
    dram_para.dram_mr1 = 0x2 as libc::c_int as libc::c_uint;
    dram_para.dram_mr2 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_mr3 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr0 = 0x471992 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr1 = 0x131a10c as libc::c_int as libc::c_uint;
    dram_para.dram_tpr2 = 0x80000000 as libc::c_uint;
    dram_para.dram_tpr3 = 0xffffffff as libc::c_uint;
    dram_para.dram_tpr4 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr5 = 0x48484848 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr6 = 0x48 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr7 = 0x1621121e as libc::c_int as libc::c_uint;
    dram_para.dram_tpr8 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr9 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr10 = 0 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr11 = 0x40000000 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr12 = 0x45 as libc::c_int as libc::c_uint;
    dram_para.dram_tpr13 = 0x34000000 as libc::c_int as libc::c_uint;
    ret_val = init_DRAM(0 as libc::c_int, &mut dram_para);
    return ret_val as libc::c_uint;
}
