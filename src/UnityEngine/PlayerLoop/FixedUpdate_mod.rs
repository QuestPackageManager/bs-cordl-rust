#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_AudioFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate => "UnityEngine.PlayerLoop"
    ."FixedUpdate/AudioFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_ClearLines {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::FixedUpdate_ClearLines
    => "UnityEngine.PlayerLoop"."FixedUpdate/ClearLines"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_DirectorFixedSampleTime {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime =>
    "UnityEngine.PlayerLoop"."FixedUpdate/DirectorFixedSampleTime"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_DirectorFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/DirectorFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_DirectorFixedUpdatePostPhysics {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics =>
    "UnityEngine.PlayerLoop"."FixedUpdate/DirectorFixedUpdatePostPhysics"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::FixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
    pub type XRFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+AudioFixedUpdate")]
    pub type AudioFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_AudioFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdatePostPhysics")]
    pub type DirectorFixedUpdatePostPhysics = crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdatePostPhysics;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedSampleTime")]
    pub type DirectorFixedSampleTime = crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedSampleTime;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+DirectorFixedUpdate")]
    pub type DirectorFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_DirectorFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ClearLines")]
    pub type ClearLines = crate::UnityEngine::PlayerLoop::FixedUpdate_ClearLines;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
    pub type PhysicsFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
    pub type ScriptRunDelayedFixedFrameRate = crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
    pub type ScriptRunBehaviourFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
    pub type NewInputFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
    pub type LegacyFixedAnimationUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
    pub type Physics2DFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
    pub type PhysicsClothFixedUpdate = crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate;
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_LegacyFixedAnimationUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/LegacyFixedAnimationUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+LegacyFixedAnimationUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_LegacyFixedAnimationUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_NewInputFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/NewInputFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+NewInputFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_NewInputFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_Physics2DFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/Physics2DFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+Physics2DFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_Physics2DFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_PhysicsClothFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/PhysicsClothFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsClothFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsClothFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_PhysicsFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate => "UnityEngine.PlayerLoop"
    ."FixedUpdate/PhysicsFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+PhysicsFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_PhysicsFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_ScriptRunBehaviourFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/ScriptRunBehaviourFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunBehaviourFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunBehaviourFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_ScriptRunDelayedFixedFrameRate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate =>
    "UnityEngine.PlayerLoop"."FixedUpdate/ScriptRunDelayedFixedFrameRate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+ScriptRunDelayedFixedFrameRate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_ScriptRunDelayedFixedFrameRate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedUpdate_XRFixedUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate => "UnityEngine.PlayerLoop"
    ."FixedUpdate/XRFixedUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+FixedUpdate+XRFixedUpdate")]
impl crate::UnityEngine::PlayerLoop::FixedUpdate_XRFixedUpdate {}
