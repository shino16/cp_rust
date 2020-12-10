use crate::float::*;

/// return (argmin f, min f)
pub fn convex_min<F: FnMut(Float) -> Float>(
	mut l: Float,
	mut r: Float,
	e: Float,
	mut f: F,
) -> (Float, Float) {
	const PHI: Float = 1.6180339887498948482;
	let k = ((r - l) / e).log(PHI) as u32 + 2;

	let mut ml = (PHI * l + r) / (1.0 + PHI);
	let mut mr = (l + PHI * r) / (1.0 + PHI);
	let mut yml = f(ml);
	let mut ymr = f(mr);

	for _ in 0..k {
		if yml < ymr {
			l = ml;
			ml = mr;
			yml = ymr;
			mr = (ml + PHI * r) / (1.0 + PHI);
			ymr = f(mr);
		} else {
			r = mr;
			mr = ml;
			ymr = yml;
			mr = (PHI * l + mr) / (1.0 + PHI);
			yml = f(ml);
		}
	}
	(ml, yml)
}
