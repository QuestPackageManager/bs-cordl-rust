#[cfg(feature = "LufsMetering+FilterCoefficients")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FilterCoefficients {
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
}
#[cfg(feature = "LufsMetering+FilterCoefficients")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::FilterCoefficients =>
    "LufsMetering"."FilterCoefficients"
);
#[cfg(feature = "LufsMetering+FilterCoefficients")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LufsMetering::FilterCoefficients {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+FilterCoefficients")]
impl crate::LufsMetering::FilterCoefficients {}
