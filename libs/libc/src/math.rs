#[unsafe(no_mangle)] pub unsafe extern "C" fn sqrt(x: f64) -> f64 { let mut r: f64; core::arch::asm!("sqrtsd {}, {}", out(xmm_reg) r, in(xmm_reg) x); r }
#[unsafe(no_mangle)] pub unsafe extern "C" fn fabs(x: f64) -> f64 { if x < 0.0 { -x } else { x } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn sin(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn cos(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn tan(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn atan(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn ceil(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn floor(_x: f64) -> f64 { 0.0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn pow(_b: f64, _e: f64) -> f64 { 0.0 }
