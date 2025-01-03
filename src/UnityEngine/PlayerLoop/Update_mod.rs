#[cfg(feature = "UnityEngine+PlayerLoop+Update")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Update {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::Update =>
    "UnityEngine.PlayerLoop"."Update"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Update")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Update {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update")]
impl crate::UnityEngine::PlayerLoop::Update {
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
    pub type DirectorUpdate = crate::UnityEngine::PlayerLoop::Update_DirectorUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
    pub type ScriptRunBehaviourUpdate = crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
    pub type ScriptRunDelayedDynamicFrameRate = crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
    pub type ScriptRunDelayedTasks = crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks;
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Update_DirectorUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::Update_DirectorUpdate
    => "UnityEngine.PlayerLoop"."Update/DirectorUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Update_DirectorUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+DirectorUpdate")]
impl crate::UnityEngine::PlayerLoop::Update_DirectorUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Update_ScriptRunBehaviourUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate =>
    "UnityEngine.PlayerLoop"."Update/ScriptRunBehaviourUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunBehaviourUpdate")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunBehaviourUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Update_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate =>
    "UnityEngine.PlayerLoop"."Update/ScriptRunDelayedDynamicFrameRate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedDynamicFrameRate")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Update_ScriptRunDelayedTasks {}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks => "UnityEngine.PlayerLoop"
    ."Update/ScriptRunDelayedTasks"
);
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+Update+ScriptRunDelayedTasks")]
impl crate::UnityEngine::PlayerLoop::Update_ScriptRunDelayedTasks {}
