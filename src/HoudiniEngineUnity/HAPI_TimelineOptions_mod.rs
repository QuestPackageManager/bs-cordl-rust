#[cfg(feature = "HoudiniEngineUnity+HAPI_TimelineOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_TimelineOptions {
    pub fps: f32,
    pub startTime: f32,
    pub endTime: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TimelineOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_TimelineOptions =>
    "HoudiniEngineUnity"."HAPI_TimelineOptions"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_TimelineOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_TimelineOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TimelineOptions")]
impl crate::HoudiniEngineUnity::HAPI_TimelineOptions {}
