#[cfg(feature = "UnityEngine+FrameTiming")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameTiming {
    pub cpuFrameTime: f64,
    pub cpuMainThreadFrameTime: f64,
    pub cpuMainThreadPresentWaitTime: f64,
    pub cpuRenderThreadFrameTime: f64,
    pub gpuFrameTime: f64,
    pub frameStartTimestamp: u64,
    pub firstSubmitTimestamp: u64,
    pub cpuTimePresentCalled: u64,
    pub cpuTimeFrameComplete: u64,
    pub heightScale: f32,
    pub widthScale: f32,
    pub syncInterval: u32,
}
#[cfg(feature = "UnityEngine+FrameTiming")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FrameTiming => "UnityEngine"
    ."FrameTiming"
);
#[cfg(feature = "UnityEngine+FrameTiming")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::FrameTiming {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+FrameTiming")]
impl crate::UnityEngine::FrameTiming {}
