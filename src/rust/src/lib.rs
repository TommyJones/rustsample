mod urn;

use extendr_api::prelude::*;

#[extendr]
fn rsample(probs: Vec<f64>, n: u64) -> Vec<u64> {

    let samples = urn::Urn::new(&probs);

    let s = samples.sample(n);

    return s;

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustsample;
    fn rsample;
}
