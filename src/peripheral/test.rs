#[test]
fn cpuid() {
    let cpuid = ::peripheral::cpuid();

    assert_eq!(::address(&cpuid.base), 0xE000_ED00);
    assert_eq!(::address(&cpuid.pfr), 0xE000_ED40);
    assert_eq!(::address(&cpuid.dfr), 0xE000_ED48);
    assert_eq!(::address(&cpuid.afr), 0xE000_ED4C);
    assert_eq!(::address(&cpuid.mmfr), 0xE000_ED50);
    assert_eq!(::address(&cpuid.isar), 0xE000_ED60);
    assert_eq!(::address(&cpuid.clidr), 0xE000_ED78);
    assert_eq!(::address(&cpuid.ctr), 0xE000_ED7C);
    assert_eq!(::address(&cpuid.ccsidr), 0xE000_ED80);
    assert_eq!(::address(&cpuid.csselr), 0xE000_ED84);
}

#[test]
fn dcb() {
    for dcb in &[::peripheral::dcb(), unsafe { ::peripheral::dcb_mut() }] {
        assert_eq!(::address(&dcb.dhcsr), 0xE000_EDF0);
        assert_eq!(::address(&dcb.dcrsr), 0xE000_EDF4);
        assert_eq!(::address(&dcb.dcrdr), 0xE000_EDF8);
        assert_eq!(::address(&dcb.demcr), 0xE000_EDFC);
    }
}

#[test]
fn dwt() {
    for dwt in &[::peripheral::dwt(), unsafe { ::peripheral::dwt_mut() }] {
        assert_eq!(::address(&dwt.ctrl), 0xE000_1000);
        assert_eq!(::address(&dwt.cyccnt), 0xE000_1004);
        assert_eq!(::address(&dwt.cpicnt), 0xE000_1008);
        assert_eq!(::address(&dwt.exccnt), 0xE000_100C);
        assert_eq!(::address(&dwt.sleepcnt), 0xE000_1010);
        assert_eq!(::address(&dwt.lsucnt), 0xE000_1014);
        assert_eq!(::address(&dwt.foldcnt), 0xE000_1018);
        assert_eq!(::address(&dwt.pcsr), 0xE000_101C);
        assert_eq!(::address(&dwt.c[0].comp), 0xE000_1020);
        assert_eq!(::address(&dwt.c[0].mask), 0xE000_1024);
        assert_eq!(::address(&dwt.c[0].function), 0xE000_1028);
        assert_eq!(::address(&dwt.c[1].comp), 0xE000_1030);
        assert_eq!(::address(&dwt.c[1].mask), 0xE000_1034);
        assert_eq!(::address(&dwt.c[1].function), 0xE000_1038);
        assert_eq!(::address(&dwt.lar), 0xE000_1FB0);
        assert_eq!(::address(&dwt.lsr), 0xE000_1FB4);
    }
}

#[test]
fn fpb() {
    for fpb in &[::peripheral::fpb(), unsafe { ::peripheral::fpb_mut() }] {
        assert_eq!(::address(&fpb.ctrl), 0xE000_2000);
        assert_eq!(::address(&fpb.remap), 0xE000_2004);
        assert_eq!(::address(&fpb.comp), 0xE000_2008);
        assert_eq!(::address(&fpb.comp[1]), 0xE000_200C);
        assert_eq!(::address(&fpb.lar), 0xE000_2FB0);
        assert_eq!(::address(&fpb.lsr), 0xE000_2FB4);
    }
}

#[test]
fn fpu() {
    for fpu in &[::peripheral::fpu(), unsafe { ::peripheral::fpu_mut() }] {
        assert_eq!(::address(&fpu.fpccr), 0xE000_EF34);
        assert_eq!(::address(&fpu.fpcar), 0xE000_EF38);
        assert_eq!(::address(&fpu.fpdscr), 0xE000_EF3C);
        assert_eq!(::address(&fpu.mvfr), 0xE000_EF40);
        assert_eq!(::address(&fpu.mvfr[1]), 0xE000_EF44);
        assert_eq!(::address(&fpu.mvfr[2]), 0xE000_EF48);
    }
}

#[test]
fn itm() {
    for itm in &[::peripheral::itm(), unsafe { ::peripheral::itm_mut() }] {
        assert_eq!(::address(&itm.stim), 0xE000_0000);
        assert_eq!(::address(&itm.ter), 0xE000_0E00);
        assert_eq!(::address(&itm.tpr), 0xE000_0E40);
        assert_eq!(::address(&itm.tcr), 0xE000_0E80);
        assert_eq!(::address(&itm.lar), 0xE000_0FB0);
        assert_eq!(::address(&itm.lsr), 0xE000_0FB4);
    }
}

#[test]
fn mpu() {
    for mpu in &[::peripheral::mpu(), unsafe { ::peripheral::mpu_mut() }] {
        assert_eq!(::address(&mpu._type), 0xE000ED90);
        assert_eq!(::address(&mpu.ctrl), 0xE000ED94);
        assert_eq!(::address(&mpu.rnr), 0xE000ED98);
        assert_eq!(::address(&mpu.rbar), 0xE000ED9C);
        assert_eq!(::address(&mpu.rasr), 0xE000EDA0);
        assert_eq!(::address(&mpu.rbar_a1), 0xE000EDA4);
        assert_eq!(::address(&mpu.rsar_a1), 0xE000EDA8);
        assert_eq!(::address(&mpu.rbar_a2), 0xE000EDAC);
        assert_eq!(::address(&mpu.rsar_a2), 0xE000EDB0);
        assert_eq!(::address(&mpu.rbar_a3), 0xE000EDB4);
        assert_eq!(::address(&mpu.rsar_a3), 0xE000EDB8);
    }
}

#[test]
fn nvic() {
    for nvic in &[::peripheral::nvic(), unsafe { ::peripheral::nvic_mut() }] {
        assert_eq!(::address(&nvic.iser), 0xE000E100);
        assert_eq!(::address(&nvic.iser[15]), 0xE000E13C);
        assert_eq!(::address(&nvic.icer), 0xE000E180);
        assert_eq!(::address(&nvic.icer[7]), 0xE000E19C);
        assert_eq!(::address(&nvic.icer[15]), 0xE000E1BC);
        assert_eq!(::address(&nvic.ispr), 0xE000E200);
        assert_eq!(::address(&nvic.ispr[15]), 0xE000E23C);
        assert_eq!(::address(&nvic.icpr), 0xE000E280);
        assert_eq!(::address(&nvic.icpr[15]), 0xE000E2BC);
        assert_eq!(::address(&nvic.iabr), 0xE000E300);
        assert_eq!(::address(&nvic.iabr[15]), 0xE000E33C);
        assert_eq!(::address(&nvic.ipr), 0xE000E400);
        assert_eq!(::address(&nvic.ipr[59]), 0xE000E4EC);
    }
}

#[test]
fn scb() {
    for scb in &[::peripheral::scb(), unsafe { ::peripheral::scb_mut() }] {
        assert_eq!(::address(&scb.icsr), 0xE000_ED04);
        assert_eq!(::address(&scb.vtor), 0xE000_ED08);
        assert_eq!(::address(&scb.aircr), 0xE000_ED0C);
        assert_eq!(::address(&scb.scr), 0xE000_ED10);
        assert_eq!(::address(&scb.ccr), 0xE000_ED14);
        assert_eq!(::address(&scb.shpr), 0xE000_ED18);
        assert_eq!(::address(&scb.shpcrs), 0xE000_ED24);
        assert_eq!(::address(&scb.cfsr), 0xE000_ED28);
        assert_eq!(::address(&scb.hfsr), 0xE000_ED2C);
        assert_eq!(::address(&scb.dfsr), 0xE000_ED30);
        assert_eq!(::address(&scb.mmar), 0xE000_ED34);
        assert_eq!(::address(&scb.bfar), 0xE000_ED38);
        assert_eq!(::address(&scb.afsr), 0xE000_ED3C);
        assert_eq!(::address(&scb.cpacr), 0xE000_ED88);
    }
}

#[test]
fn syst() {
    for syst in &[::peripheral::syst(), unsafe { ::peripheral::syst_mut() }] {
        assert_eq!(::address(&syst.csr), 0xE000_E010);
        assert_eq!(::address(&syst.rvr), 0xE000_E014);
        assert_eq!(::address(&syst.cvr), 0xE000_E018);
        assert_eq!(::address(&syst.calib), 0xE000_E01C);
    }
}

#[test]
fn tpiu() {
    for tpiu in &[::peripheral::tpiu(), unsafe { ::peripheral::tpiu_mut() }] {
        assert_eq!(::address(&tpiu.sspsr), 0xE004_0000);
        assert_eq!(::address(&tpiu.cspsr), 0xE004_0004);
        assert_eq!(::address(&tpiu.acpr), 0xE004_0010);
        assert_eq!(::address(&tpiu.sppr), 0xE004_00F0);
        assert_eq!(::address(&tpiu.lar), 0xE004_0FB0);
        assert_eq!(::address(&tpiu.lsr), 0xE004_0FB4);
        assert_eq!(::address(&tpiu._type), 0xE004_0FC8);
    }
}
