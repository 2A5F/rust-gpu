#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr, lang_items),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std::Sampler;

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
pub fn main_fs(#[spirv(descriptor_set = 2, binding = 1)] _noise_sampler: &Sampler) {}
