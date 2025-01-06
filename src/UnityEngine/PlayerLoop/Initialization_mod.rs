#[cfg(feature = "UnityEngine+PlayerLoop+Initialization")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::Initialization =>
    "UnityEngine.PlayerLoop"."Initialization"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization")]
impl crate::UnityEngine::PlayerLoop::Initialization {
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+AsyncUploadTimeSlicedUpdate")]
    pub type AsyncUploadTimeSlicedUpdate = crate::UnityEngine::PlayerLoop::Initialization_AsyncUploadTimeSlicedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+DirectorSampleTime")]
    pub type DirectorSampleTime = crate::UnityEngine::PlayerLoop::Initialization_DirectorSampleTime;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+ProfilerStartFrame")]
    pub type ProfilerStartFrame = crate::UnityEngine::PlayerLoop::Initialization_ProfilerStartFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeInputs")]
    pub type SynchronizeInputs = crate::UnityEngine::PlayerLoop::Initialization_SynchronizeInputs;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeState")]
    pub type SynchronizeState = crate::UnityEngine::PlayerLoop::Initialization_SynchronizeState;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+UpdateCameraMotionVectors")]
    pub type UpdateCameraMotionVectors = crate::UnityEngine::PlayerLoop::Initialization_UpdateCameraMotionVectors;
    #[cfg(feature = "UnityEngine+PlayerLoop+Initialization+XREarlyUpdate")]
    pub type XREarlyUpdate = crate::UnityEngine::PlayerLoop::Initialization_XREarlyUpdate;
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+AsyncUploadTimeSlicedUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_AsyncUploadTimeSlicedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+AsyncUploadTimeSlicedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_AsyncUploadTimeSlicedUpdate =>
    "UnityEngine.PlayerLoop"."Initialization/AsyncUploadTimeSlicedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+AsyncUploadTimeSlicedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_AsyncUploadTimeSlicedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+AsyncUploadTimeSlicedUpdate")]
impl crate::UnityEngine::PlayerLoop::Initialization_AsyncUploadTimeSlicedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+DirectorSampleTime")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_DirectorSampleTime {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+DirectorSampleTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_DirectorSampleTime =>
    "UnityEngine.PlayerLoop"."Initialization/DirectorSampleTime"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+DirectorSampleTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_DirectorSampleTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+DirectorSampleTime")]
impl crate::UnityEngine::PlayerLoop::Initialization_DirectorSampleTime {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+ProfilerStartFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_ProfilerStartFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+ProfilerStartFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_ProfilerStartFrame =>
    "UnityEngine.PlayerLoop"."Initialization/ProfilerStartFrame"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+ProfilerStartFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_ProfilerStartFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+ProfilerStartFrame")]
impl crate::UnityEngine::PlayerLoop::Initialization_ProfilerStartFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeInputs")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_SynchronizeInputs {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeInputs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_SynchronizeInputs =>
    "UnityEngine.PlayerLoop"."Initialization/SynchronizeInputs"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeInputs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_SynchronizeInputs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeInputs")]
impl crate::UnityEngine::PlayerLoop::Initialization_SynchronizeInputs {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_SynchronizeState {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_SynchronizeState =>
    "UnityEngine.PlayerLoop"."Initialization/SynchronizeState"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_SynchronizeState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+SynchronizeState")]
impl crate::UnityEngine::PlayerLoop::Initialization_SynchronizeState {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+UpdateCameraMotionVectors")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_UpdateCameraMotionVectors {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+UpdateCameraMotionVectors")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_UpdateCameraMotionVectors =>
    "UnityEngine.PlayerLoop"."Initialization/UpdateCameraMotionVectors"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+UpdateCameraMotionVectors")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_UpdateCameraMotionVectors {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+UpdateCameraMotionVectors")]
impl crate::UnityEngine::PlayerLoop::Initialization_UpdateCameraMotionVectors {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+XREarlyUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Initialization_XREarlyUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+XREarlyUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Initialization_XREarlyUpdate => "UnityEngine.PlayerLoop"
    ."Initialization/XREarlyUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+XREarlyUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Initialization_XREarlyUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Initialization+XREarlyUpdate")]
impl crate::UnityEngine::PlayerLoop::Initialization_XREarlyUpdate {}
