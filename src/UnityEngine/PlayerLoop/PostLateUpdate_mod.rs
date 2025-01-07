#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate {
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
    pub type BatchModeUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
    pub type ClearImmediateRenderers = crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
    pub type DirectorLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
    pub type DirectorRenderImage = crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
    )]
    pub type EndGraphicsJobsAfterScriptLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
    pub type EnlightenRuntimeUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
    pub type ExecuteGameCenterCallbacks = crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
    pub type FinishFrameRendering = crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
    pub type GUIClearEvents = crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders"
    )]
    pub type GraphicsWarmupPreloadedShaders = crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
    pub type InputEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
    pub type MemoryFrameMaintenance = crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate"
    )]
    pub type ObjectDispatcherPostLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
    pub type ParticleSystemEndUpdateAll = crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate"
    )]
    pub type PhysicsSkinnedClothBeginUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
    )]
    pub type PhysicsSkinnedClothFinishUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
    pub type PlayerEmitCanvasGeometry = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
    )]
    pub type PlayerRenderUIEBatchModeOffscreen = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
    pub type PlayerSendFrameComplete = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
    pub type PlayerSendFramePostPresent = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
    pub type PlayerSendFrameStarted = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
    pub type PlayerUpdateCanvases = crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
    pub type PresentAfterDraw = crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
    pub type ProcessWebSendMessages = crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
    pub type ProfilerEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
    pub type ProfilerSynchronizeStats = crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
    pub type ResetInputAxis = crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
    )]
    pub type ScriptRunDelayedDynamicFrameRate = crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
    pub type ShaderHandleErrors = crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
    pub type SortingGroupsUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
    pub type ThreadedLoadingDebug = crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
    pub type TriggerEndOfFrameCallbacks = crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
    pub type UpdateAllRenderers = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
    pub type UpdateAllSkinnedMeshes = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
    pub type UpdateAudio = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
    pub type UpdateCanvasRectTransform = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
    pub type UpdateCaptureScreenshot = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
    pub type UpdateCustomRenderTextures = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures;
    #[cfg(
        feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes"
    )]
    pub type UpdateLightProbeProxyVolumes = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
    pub type UpdateRectTransform = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
    pub type UpdateResolution = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
    pub type UpdateSubstance = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
    pub type UpdateVideo = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
    pub type UpdateVideoTextures = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
    pub type VFXUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
    pub type XRPostLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
    pub type XRPostPresent = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
    pub type XRPreEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame;
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_BatchModeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "BatchModeUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ClearImmediateRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ClearImmediateRenderers";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_DirectorLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "DirectorLateUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_DirectorRenderImage {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "DirectorRenderImage";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "EndGraphicsJobsAfterScriptLateUpdate";
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_EnlightenRuntimeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "EnlightenRuntimeUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ExecuteGameCenterCallbacks {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ExecuteGameCenterCallbacks";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_FinishFrameRendering {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "FinishFrameRendering";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_GUIClearEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "GUIClearEvents";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_GraphicsWarmupPreloadedShaders {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "GraphicsWarmupPreloadedShaders";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_InputEndFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "InputEndFrame";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_MemoryFrameMaintenance {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "MemoryFrameMaintenance";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ObjectDispatcherPostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ObjectDispatcherPostLateUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ParticleSystemEndUpdateAll {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ParticleSystemEndUpdateAll";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PhysicsSkinnedClothBeginUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PhysicsSkinnedClothBeginUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PhysicsSkinnedClothFinishUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PhysicsSkinnedClothFinishUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerEmitCanvasGeometry {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerEmitCanvasGeometry";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerRenderUIEBatchModeOffscreen";
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerRenderUIEBatchModeOffscreen"
)]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerSendFrameComplete {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerSendFrameComplete";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerSendFramePostPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerSendFramePostPresent";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerSendFrameStarted {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerSendFrameStarted";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PlayerUpdateCanvases {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PlayerUpdateCanvases";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_PresentAfterDraw {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PresentAfterDraw";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ProcessWebSendMessages {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ProcessWebSendMessages";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ProfilerEndFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ProfilerEndFrame";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ProfilerSynchronizeStats {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ProfilerSynchronizeStats";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ResetInputAxis {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ResetInputAxis";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ScriptRunDelayedDynamicFrameRate {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ScriptRunDelayedDynamicFrameRate";
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
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
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ShaderHandleErrors {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ShaderHandleErrors";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_SortingGroupsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "SortingGroupsUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_ThreadedLoadingDebug {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "ThreadedLoadingDebug";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_TriggerEndOfFrameCallbacks {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "TriggerEndOfFrameCallbacks";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateAllRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateAllRenderers";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateAllSkinnedMeshes {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateAllSkinnedMeshes";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateAudio {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateAudio";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateCanvasRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateCaptureScreenshot {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateCaptureScreenshot";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateCustomRenderTextures {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateCustomRenderTextures";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateLightProbeProxyVolumes {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateLightProbeProxyVolumes";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateRectTransform";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateResolution {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateResolution";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateSubstance {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateSubstance";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateVideo {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateVideo";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_UpdateVideoTextures {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "UpdateVideoTextures";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_VFXUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "VFXUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_XRPostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "XRPostLateUpdate";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_XRPostPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "XRPostPresent";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate_XRPreEndFrame {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "XRPreEndFrame";
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
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
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame {}
