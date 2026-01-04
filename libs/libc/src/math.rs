use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqrt(x: f64) -> f64 {
    let mut res: f64;
    asm!("sqrtsd {0}, {0}", inout(xmm_reg) x => res);
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & !(1 << 63))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ceil(x: f64) -> f64 {
    let mut res: f64;
    asm!("roundsd {0}, {1}, 2", out(xmm_reg) res, in(xmm_reg) x);
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floor(x: f64) -> f64 {
    let mut res: f64;
    asm!("roundsd {0}, {1}, 1", out(xmm_reg) res, in(xmm_reg) x);
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sin(x: f64) -> f64 {
    let mut res: f64 = 0.0;
    asm!(
        "fld QWORD PTR [{0}]",
        "fsin",
        "fstp QWORD PTR [{1}]",
        in(reg) &x,
        in(reg) &mut res,
    );
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cos(x: f64) -> f64 {
    let mut res: f64 = 0.0;
    asm!(
        "fld QWORD PTR [{0}]",
        "fcos",
        "fstp QWORD PTR [{1}]",
        in(reg) &x,
        in(reg) &mut res,
    );
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tan(x: f64) -> f64 {
    let mut res: f64 = 0.0;
    asm!(
        "fld QWORD PTR [{0}]",
        "fptan",
        "fstp st(0)", // pop 1.0
        "fstp QWORD PTR [{1}]",
        in(reg) &x,
        in(reg) &mut res,
    );
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn atan(x: f64) -> f64 {
    let mut res: f64 = 0.0;
    asm!(
        "fld QWORD PTR [{0}]",
        "fld1",
        "fpatan",
        "fstp QWORD PTR [{1}]",
        in(reg) &x,
        in(reg) &mut res,
    );
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pow(base: f64, exp: f64) -> f64 {
    if base == 0.0 { return 0.0; }
    if exp == 0.0 { return 1.0; }
    
    let mut res: f64 = 0.0;
    asm!(
        "fld QWORD PTR [{1}]",   // exp
        "fld QWORD PTR [{0}]",   // base
        "fyl2x",      // st(0) = exp * log2(base)
        "fld st(0)",
        "frndint",    // st(0) = round(st(0))
        "fsubr st(0), st(1)", // st(1) = fraction
        "fxch st(1)",
        "f2xm1",      // st(0) = 2^fraction - 1
        "fld1",
        "faddp",      // st(0) = 2^fraction
        "fscale",     // st(0) = 2^fraction * 2^integer
        "fstp st(1)",
        "fstp QWORD PTR [{2}]",
        in(reg) &base,
        in(reg) &exp,
        in(reg) &mut res,
    );
    res
}
