#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostLateUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::PostLateUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/BatchModeUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ClearImmediateRenderers"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/DirectorLateUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/DirectorRenderImage"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/EndGraphicsJobsAfterScriptLateUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/EnlightenRuntimeUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ExecuteGameCenterCallbacks"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/FinishFrameRendering"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/GUIClearEvents"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/GraphicsWarmupPreloadedShaders"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/InputEndFrame"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/MemoryFrameMaintenance"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ObjectDispatcherPostLateUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ParticleSystemEndUpdateAll"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PhysicsSkinnedClothBeginUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PhysicsSkinnedClothFinishUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerEmitCanvasGeometry"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerRenderUIEBatchModeOffscreen =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerRenderUIEBatchModeOffscreen"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerSendFrameComplete"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerSendFramePostPresent"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerSendFrameStarted"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PlayerUpdateCanvases"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/PresentAfterDraw"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ProcessWebSendMessages"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ProfilerEndFrame"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ProfilerSynchronizeStats"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/ResetInputAxis"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ScriptRunDelayedDynamicFrameRate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ShaderHandleErrors"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/SortingGroupsUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/ThreadedLoadingDebug"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/TriggerEndOfFrameCallbacks"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateAllRenderers"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateAllSkinnedMeshes"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/UpdateAudio"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateCanvasRectTransform"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateCaptureScreenshot"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateCustomRenderTextures"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateLightProbeProxyVolumes"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateRectTransform"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateResolution"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/UpdateSubstance"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/UpdateVideo"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/UpdateVideoTextures"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/VFXUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate =>
    "UnityEngine.PlayerLoop"."PostLateUpdate/XRPostLateUpdate"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/XRPostPresent"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame => "UnityEngine.PlayerLoop"
    ."PostLateUpdate/XRPreEndFrame"
);
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
