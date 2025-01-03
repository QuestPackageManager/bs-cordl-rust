#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PreUpdate =>
    "UnityEngine.PlayerLoop"."PreUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
    pub type AIUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
    pub type CheckTexFieldInput = crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
    pub type IMGUISendQueuedEvents = crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
    pub type NewInputUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
    pub type Physics2DUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
    pub type PhysicsClothUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
    pub type PhysicsUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
    pub type SendMouseEvents = crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
    pub type UpdateVideo = crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo;
    #[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
    pub type WindUpdate = crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate;
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_AIUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PreUpdate_AIUpdate =>
    "UnityEngine.PlayerLoop"."PreUpdate/AIUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+AIUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_AIUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_CheckTexFieldInput {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput => "UnityEngine.PlayerLoop"
    ."PreUpdate/CheckTexFieldInput"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+CheckTexFieldInput")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_CheckTexFieldInput {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_IMGUISendQueuedEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents =>
    "UnityEngine.PlayerLoop"."PreUpdate/IMGUISendQueuedEvents"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+IMGUISendQueuedEvents")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_IMGUISendQueuedEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_NewInputUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate => "UnityEngine.PlayerLoop"
    ."PreUpdate/NewInputUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+NewInputUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_NewInputUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_Physics2DUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate => "UnityEngine.PlayerLoop"
    ."PreUpdate/Physics2DUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+Physics2DUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_Physics2DUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_PhysicsClothUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate => "UnityEngine.PlayerLoop"
    ."PreUpdate/PhysicsClothUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsClothUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsClothUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_PhysicsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate
    => "UnityEngine.PlayerLoop"."PreUpdate/PhysicsUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+PhysicsUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_PhysicsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_SendMouseEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents => "UnityEngine.PlayerLoop"
    ."PreUpdate/SendMouseEvents"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+SendMouseEvents")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_SendMouseEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_UpdateVideo {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo
    => "UnityEngine.PlayerLoop"."PreUpdate/UpdateVideo"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+UpdateVideo")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_UpdateVideo {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PreUpdate_WindUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PreUpdate_WindUpdate =>
    "UnityEngine.PlayerLoop"."PreUpdate/WindUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PreUpdate+WindUpdate")]
impl crate::UnityEngine::PlayerLoop::PreUpdate_WindUpdate {}
