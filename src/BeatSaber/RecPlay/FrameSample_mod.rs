#[cfg(feature = "BeatSaber+RecPlay+FrameSample")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameSample {
    pub previous: i32,
    pub next: i32,
    pub alpha: f32,
}
#[cfg(feature = "BeatSaber+RecPlay+FrameSample")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::FrameSample =>
    "BeatSaber.RecPlay"."FrameSample"
);
#[cfg(feature = "BeatSaber+RecPlay+FrameSample")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::FrameSample {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+FrameSample")]
impl crate::BeatSaber::RecPlay::FrameSample {}
