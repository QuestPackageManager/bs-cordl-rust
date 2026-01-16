#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::PostLateUpdate {
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PlayerLoop::PostLateUpdate {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PlayerLoop::PostLateUpdate {
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
    pub type ClearImmediateRenderers =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
    pub type DirectorLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
    pub type DirectorRenderImage =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate")]
    pub type EndGraphicsJobsAfterScriptLateUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
    pub type EnlightenRuntimeUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
    pub type ExecuteGameCenterCallbacks =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
    pub type FinishFrameRendering =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
    pub type GUIClearEvents = crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
    pub type GraphicsWarmupPreloadedShaders =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
    pub type InputEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
    pub type MemoryFrameMaintenance =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
    pub type ObjectDispatcherPostLateUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
    pub type ParticleSystemEndUpdateAll =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
    pub type PhysicsSkinnedClothBeginUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate")]
    pub type PhysicsSkinnedClothFinishUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
    pub type PlayerEmitCanvasGeometry =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
    pub type PlayerSendFrameComplete =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
    pub type PlayerSendFramePostPresent =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
    pub type PlayerSendFrameStarted =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
    pub type PlayerUpdateCanvases =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
    pub type PresentAfterDraw = crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
    pub type ProcessWebSendMessages =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
    pub type ProfilerEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
    pub type ProfilerSynchronizeStats =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
    pub type ResetInputAxis = crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate")]
    pub type ScriptRunDelayedDynamicFrameRate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
    pub type ShaderHandleErrors = crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
    pub type SortingGroupsUpdate =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
    pub type ThreadedLoadingDebug =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
    pub type TriggerEndOfFrameCallbacks =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen")]
    pub type UIElementsRenderBatchModeOffscreen =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
    pub type UIElementsRepaintPanels =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
    pub type UpdateAllRenderers = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
    pub type UpdateAllSkinnedMeshes =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
    pub type UpdateAudio = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
    pub type UpdateCanvasRectTransform =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
    pub type UpdateCaptureScreenshot =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
    pub type UpdateCustomRenderTextures =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
    pub type UpdateLightProbeProxyVolumes =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
    pub type UpdateRectTransform =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
    pub type UpdateResolution = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
    pub type UpdateSubstance = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
    pub type UpdateVideo = crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
    pub type UpdateVideoTextures =
        crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
    pub type VFXUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
    pub type XRPostLateUpdate = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
    pub type XRPostPresent = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent;
    #[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
    pub type XRPreEndFrame = crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame;
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_BatchModeUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/BatchModeUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+BatchModeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_BatchModeUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ClearImmediateRenderers {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ClearImmediateRenderers";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ClearImmediateRenderers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ClearImmediateRenderers
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_DirectorLateUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/DirectorLateUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorLateUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_DirectorRenderImage {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/DirectorRenderImage";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+DirectorRenderImage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_DirectorRenderImage
{
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
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/EndGraphicsJobsAfterScriptLateUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+EndGraphicsJobsAfterScriptLateUpdate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_EndGraphicsJobsAfterScriptLateUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_EnlightenRuntimeUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/EnlightenRuntimeUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+EnlightenRuntimeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_EnlightenRuntimeUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ExecuteGameCenterCallbacks {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ExecuteGameCenterCallbacks";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ExecuteGameCenterCallbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ExecuteGameCenterCallbacks
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_FinishFrameRendering {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/FinishFrameRendering";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+FinishFrameRendering")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_FinishFrameRendering
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_GUIClearEvents {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/GUIClearEvents";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GUIClearEvents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GUIClearEvents
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_GraphicsWarmupPreloadedShaders {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/GraphicsWarmupPreloadedShaders";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+GraphicsWarmupPreloadedShaders")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_GraphicsWarmupPreloadedShaders
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_InputEndFrame {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/InputEndFrame";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+InputEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_InputEndFrame
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_MemoryFrameMaintenance {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/MemoryFrameMaintenance";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+MemoryFrameMaintenance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_MemoryFrameMaintenance
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ObjectDispatcherPostLateUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ObjectDispatcherPostLateUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ObjectDispatcherPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ObjectDispatcherPostLateUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ParticleSystemEndUpdateAll {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ParticleSystemEndUpdateAll";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ParticleSystemEndUpdateAll")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ParticleSystemEndUpdateAll
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PhysicsSkinnedClothBeginUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PhysicsSkinnedClothBeginUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothBeginUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothBeginUpdate
{
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PhysicsSkinnedClothFinishUpdate {}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PhysicsSkinnedClothFinishUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PhysicsSkinnedClothFinishUpdate"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PhysicsSkinnedClothFinishUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PlayerEmitCanvasGeometry {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PlayerEmitCanvasGeometry";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerEmitCanvasGeometry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerEmitCanvasGeometry
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PlayerSendFrameComplete {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PlayerSendFrameComplete";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameComplete")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameComplete
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PlayerSendFramePostPresent {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PlayerSendFramePostPresent";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFramePostPresent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFramePostPresent
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PlayerSendFrameStarted {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PlayerSendFrameStarted";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerSendFrameStarted")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerSendFrameStarted
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PlayerUpdateCanvases {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PlayerUpdateCanvases";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PlayerUpdateCanvases")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PlayerUpdateCanvases
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_PresentAfterDraw {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/PresentAfterDraw";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+PresentAfterDraw")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_PresentAfterDraw
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ProcessWebSendMessages {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ProcessWebSendMessages";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProcessWebSendMessages")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProcessWebSendMessages
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ProfilerEndFrame {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ProfilerEndFrame";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerEndFrame
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ProfilerSynchronizeStats {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ProfilerSynchronizeStats";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ProfilerSynchronizeStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ProfilerSynchronizeStats
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ResetInputAxis {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ResetInputAxis";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ResetInputAxis")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ResetInputAxis
{
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
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ScriptRunDelayedDynamicFrameRate {}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ScriptRunDelayedDynamicFrameRate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+ScriptRunDelayedDynamicFrameRate")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_ScriptRunDelayedDynamicFrameRate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ShaderHandleErrors {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ShaderHandleErrors";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ShaderHandleErrors")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ShaderHandleErrors
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_SortingGroupsUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/SortingGroupsUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+SortingGroupsUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_SortingGroupsUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_ThreadedLoadingDebug {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/ThreadedLoadingDebug";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+ThreadedLoadingDebug")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_ThreadedLoadingDebug
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_TriggerEndOfFrameCallbacks {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/TriggerEndOfFrameCallbacks";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+TriggerEndOfFrameCallbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_TriggerEndOfFrameCallbacks
{
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UIElementsRenderBatchModeOffscreen {}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UIElementsRenderBatchModeOffscreen";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRenderBatchModeOffscreen")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRenderBatchModeOffscreen {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UIElementsRepaintPanels {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UIElementsRepaintPanels";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+PostLateUpdate+UIElementsRepaintPanels")]
impl crate::UnityEngine::PlayerLoop::PostLateUpdate_UIElementsRepaintPanels {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateAllRenderers {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateAllRenderers";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllRenderers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllRenderers
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateAllSkinnedMeshes {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateAllSkinnedMeshes";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAllSkinnedMeshes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAllSkinnedMeshes
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateAudio {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateAudio";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateAudio")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateAudio
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateCanvasRectTransform {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateCanvasRectTransform";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCanvasRectTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCanvasRectTransform
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateCaptureScreenshot {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateCaptureScreenshot";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCaptureScreenshot")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCaptureScreenshot
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateCustomRenderTextures {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateCustomRenderTextures";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateCustomRenderTextures")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateCustomRenderTextures
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateLightProbeProxyVolumes {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateLightProbeProxyVolumes";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateLightProbeProxyVolumes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateLightProbeProxyVolumes
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateRectTransform {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateRectTransform";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateRectTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateRectTransform
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateResolution {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateResolution";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateResolution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateResolution
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateSubstance {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateSubstance";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateSubstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateSubstance
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateVideo {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateVideo";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideo
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_UpdateVideoTextures {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/UpdateVideoTextures";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+UpdateVideoTextures")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_UpdateVideoTextures
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_VFXUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/VFXUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+VFXUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_VFXUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_XRPostLateUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/XRPostLateUpdate";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostLateUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostLateUpdate
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_XRPostPresent {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/XRPostPresent";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPostPresent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPostPresent
{
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostLateUpdate_XRPreEndFrame {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "PostLateUpdate/XRPreEndFrame";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+PostLateUpdate+XRPreEndFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::PlayerLoop::PostLateUpdate_XRPreEndFrame
{
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
