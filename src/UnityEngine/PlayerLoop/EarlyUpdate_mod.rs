#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "EarlyUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
    pub type ARCoreUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
    pub type AnalyticsCoreStatsUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
    pub type ClearIntermediateRenderers = crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
    pub type ClearLines = crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
    pub type DeliverIosPlatformEvents = crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
    pub type DispatchEventQueueEvents = crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
    pub type ExecuteMainThreadJobs = crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
    pub type GpuTimestamp = crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
    pub type PerformanceAnalyticsUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
    pub type Physics2DEarlyUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
    )]
    pub type PhysicsResetInterpolatedTransformPosition = crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
    pub type PlayerCleanupCachedData = crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
    pub type PollHtcsPlayerConnection = crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
    pub type PollPlayerConnection = crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
    pub type PresentBeforeUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
    pub type ProcessMouseInWindow = crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
    pub type ProcessRemoteInput = crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
    pub type RendererNotifyInvisible = crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
    pub type ResetFrameStatsAfterPresent = crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
    pub type ScriptRunDelayedStartupFrame = crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
    pub type SpriteAtlasManagerUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
    pub type TangoUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
    pub type UnityWebRequestUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
    pub type UpdateAsyncInstantiate = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
    pub type UpdateAsyncReadbackManager = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
    pub type UpdateCanvasRectTransform = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
    pub type UpdateContentLoading = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
    pub type UpdateInputManager = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
    pub type UpdateKinect = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
    pub type UpdateMainGameViewRect = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
    pub type UpdatePreloading = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
    pub type UpdateStreamingManager = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
    pub type UpdateTextureStreamingManager = crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager;
    #[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
    pub type XRUpdate = crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate;
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ARCoreUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ARCoreUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ARCoreUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ARCoreUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_AnalyticsCoreStatsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "AnalyticsCoreStatsUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+AnalyticsCoreStatsUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_AnalyticsCoreStatsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ClearIntermediateRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ClearIntermediateRenderers";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearIntermediateRenderers")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearIntermediateRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ClearLines {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ClearLines";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ClearLines")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ClearLines {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_DeliverIosPlatformEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "DeliverIosPlatformEvents";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DeliverIosPlatformEvents")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_DeliverIosPlatformEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_DispatchEventQueueEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "DispatchEventQueueEvents";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+DispatchEventQueueEvents")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_DispatchEventQueueEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ExecuteMainThreadJobs {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ExecuteMainThreadJobs";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ExecuteMainThreadJobs")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ExecuteMainThreadJobs {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_GpuTimestamp {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "GpuTimestamp";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+GpuTimestamp")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_GpuTimestamp {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PerformanceAnalyticsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PerformanceAnalyticsUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PerformanceAnalyticsUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PerformanceAnalyticsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_Physics2DEarlyUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "Physics2DEarlyUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+Physics2DEarlyUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_Physics2DEarlyUpdate {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PhysicsResetInterpolatedTransformPosition {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PhysicsResetInterpolatedTransformPosition";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+EarlyUpdate+PhysicsResetInterpolatedTransformPosition"
)]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PhysicsResetInterpolatedTransformPosition {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PlayerCleanupCachedData {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerCleanupCachedData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PlayerCleanupCachedData")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PlayerCleanupCachedData {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PollHtcsPlayerConnection {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PollHtcsPlayerConnection";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollHtcsPlayerConnection")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PollHtcsPlayerConnection {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PollPlayerConnection {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PollPlayerConnection";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PollPlayerConnection")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PollPlayerConnection {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_PresentBeforeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PresentBeforeUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+PresentBeforeUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_PresentBeforeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ProcessMouseInWindow {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ProcessMouseInWindow";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessMouseInWindow")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessMouseInWindow {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ProcessRemoteInput {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ProcessRemoteInput";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ProcessRemoteInput")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ProcessRemoteInput {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_RendererNotifyInvisible {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "RendererNotifyInvisible";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+RendererNotifyInvisible")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_RendererNotifyInvisible {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ResetFrameStatsAfterPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ResetFrameStatsAfterPresent";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ResetFrameStatsAfterPresent")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ResetFrameStatsAfterPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_ScriptRunDelayedStartupFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ScriptRunDelayedStartupFrame";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+ScriptRunDelayedStartupFrame")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_ScriptRunDelayedStartupFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_SpriteAtlasManagerUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "SpriteAtlasManagerUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+SpriteAtlasManagerUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_SpriteAtlasManagerUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_TangoUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "TangoUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+TangoUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_TangoUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UnityWebRequestUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UnityWebRequestUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UnityWebRequestUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UnityWebRequestUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateAsyncInstantiate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateAsyncInstantiate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncInstantiate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncInstantiate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateAsyncReadbackManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateAsyncReadbackManager";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateAsyncReadbackManager")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateAsyncReadbackManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateCanvasRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateCanvasRectTransform";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateCanvasRectTransform")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateCanvasRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateContentLoading {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateContentLoading";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateContentLoading")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateContentLoading {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateInputManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateInputManager";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateInputManager")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateInputManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateKinect {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateKinect";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateKinect")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateKinect {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateMainGameViewRect {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateMainGameViewRect";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateMainGameViewRect")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateMainGameViewRect {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdatePreloading {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdatePreloading";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdatePreloading")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdatePreloading {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateStreamingManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateStreamingManager";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateStreamingManager")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateStreamingManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_UpdateTextureStreamingManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateTextureStreamingManager";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+UpdateTextureStreamingManager")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_UpdateTextureStreamingManager {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EarlyUpdate_XRUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "XRUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+EarlyUpdate+XRUpdate")]
impl crate::UnityEngine::PlayerLoop::EarlyUpdate_XRUpdate {}
