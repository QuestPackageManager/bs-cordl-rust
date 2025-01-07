#[cfg(feature = "OVRTrackedKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTrackedKeyboard {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _CurrentKeyboardAngleFromUp_k__BackingField: f32,
    pub _TrackingState_k__BackingField: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    pub _ActiveKeyboardInfo_k__BackingField: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    pub _SystemKeyboardInfo_k__BackingField: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    pub trackingEnabled: bool,
    pub connectionRequired: bool,
    pub showUntracked: bool,
    pub keyboardQueryFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
    pub presentation: crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation,
    pub textureFiltering: crate::GlobalNamespace::OVRTextureQualityFiltering,
    pub mipmapBias: f32,
    pub PassthroughBorderMultiplier: f32,
    pub keyboardModelShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub keyboardModelAlphaBlendShader: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Shader,
    >,
    pub currentKeyboardPresentationStyles: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles,
    pub projectedPassthroughOpaque_: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVROverlay,
    >,
    pub activeKeyboardRenderers_: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
        >,
    >,
    pub activeKeyboardMesh_: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub keyboardMeshNodes_: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub activeKeyboardMeshRenderer_: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MeshRenderer,
    >,
    pub passthroughQuad_: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub opaqueShader_: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub dynamicQualityTexture_: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub untrackedPosition_: crate::UnityEngine::Vector3,
    pub KeyLabelModeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub PassthroughShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub projectedPassthroughRoot: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub projectedPassthroughMesh: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MeshFilter,
    >,
    pub ProjectedPassthroughKeyLabel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRPassthroughLayer,
    >,
    pub TrackedKeyboardActiveChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent,
        >,
    >,
    pub TrackedKeyboardVisibilityChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent,
        >,
    >,
    pub ActiveKeyboardTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub HandsOverKeyboard: bool,
    pub cameraRig_: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    pub updateKeyboardRoutine_: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    pub keyboardBoundingBox_: quest_hook::libil2cpp::Gc<crate::UnityEngine::BoxCollider>,
    pub staleTimeoutCounter_: f32,
    pub reacquisitionTimer_: f32,
    pub sendFilteredPoseEventTimer_: f32,
    pub skippedPoseCount_: i32,
    pub EWAPosition: crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    pub EWARotation: crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    pub HAND_HEIGHT_TUNING: f32,
    pub UseHeuristicRollback: bool,
}
#[cfg(feature = "OVRTrackedKeyboard")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTrackedKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTrackedKeyboard";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRTrackedKeyboard")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTrackedKeyboard {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboard")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTrackedKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboard")]
impl crate::GlobalNamespace::OVRTrackedKeyboard {
    pub const FILTERED_POSE_TIMEOUT: f32 = 15f32;
    pub const STALE_TIMEOUT: f32 = 10f32;
    #[cfg(feature = "OVRTrackedKeyboard+KeyboardPresentation")]
    pub type KeyboardPresentation = crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation;
    #[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
    pub type TrackedKeyboardSetActiveEvent = crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent;
    #[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardState")]
    pub type TrackedKeyboardState = crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState;
    #[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
    pub type TrackedKeyboardVisibilityChangedEvent = crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent;
    pub fn DispatchVisibilityEvent(
        &mut self,
        timeOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DispatchVisibilityEvent", (timeOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDistanceToKeyboard(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDistanceToKeyboard", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyboardVisibility(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetKeyboardVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeHandPresenceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("InitializeHandPresenceData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeKeyboardInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeKeyboardInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KeyboardTrackerIsRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("KeyboardTrackerIsRunning", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchLocalKeyboardSelectionDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchLocalKeyboardSelectionDialog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchOverlayIntent(
        &mut self,
        dataUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchOverlayIntent", (dataUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchRemoteKeyboardSelectionDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchRemoteKeyboardSelectionDialog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadKeyboardMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRuntimeKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("LoadRuntimeKeyboardMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterPassthroughMeshToSDK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterPassthroughMeshToSDK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardState(
        &mut self,
        state: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyboardState", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartKeyboardTrackingCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("StartKeyboardTrackingCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopKeyboardTrackingInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopKeyboardTrackingInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateKeyboardPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("UpdateKeyboardPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateKeyboardVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateKeyboardVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePresentation(
        &mut self,
        isVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePresentation", (isVisible))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSkippedPoseTimer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSkippedPoseTimer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextureQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTextureQuality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTrackingStateCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("UpdateTrackingStateCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ActiveKeyboardInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo = __cordl_object
            .invoke("get_ActiveKeyboardInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectionRequired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ConnectionRequired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentKeyboardAngleFromUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_CurrentKeyboardAngleFromUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyboardQueryFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags = __cordl_object
            .invoke("get_KeyboardQueryFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PassthroughOverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay> = __cordl_object
            .invoke("get_PassthroughOverlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Presentation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation = __cordl_object
            .invoke("get_Presentation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RemoteKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShowUntracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShowUntracked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SystemKeyboardInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo = __cordl_object
            .invoke("get_SystemKeyboardInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TrackingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState = __cordl_object
            .invoke("get_TrackingState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ActiveKeyboardInfo(
        &mut self,
        value: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ActiveKeyboardInfo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ConnectionRequired(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConnectionRequired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CurrentKeyboardAngleFromUp(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurrentKeyboardAngleFromUp", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_KeyboardQueryFlags(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyboardQueryFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PassthroughOverlay(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PassthroughOverlay", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Presentation(
        &mut self,
        value: crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Presentation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RemoteKeyboard(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RemoteKeyboard", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ShowUntracked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ShowUntracked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SystemKeyboardInfo(
        &mut self,
        value: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SystemKeyboardInfo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TrackingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TrackingEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TrackingState(
        &mut self,
        value: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TrackingState", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTrackedKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTrackedKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTrackedKeyboard+KeyboardPresentation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRTrackedKeyboard_KeyboardPresentation {
    #[default]
    PreferKeyLabels = 1i32,
    PreferOpaque = 0i32,
}
#[cfg(feature = "OVRTrackedKeyboard+KeyboardPresentation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KeyboardPresentation";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation {
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
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    pub IsEnabled: bool,
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TrackedKeyboardSetActiveEvent";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
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
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
impl crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    pub fn _ctor(
        &mut self,
        isEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isEnabled),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRTrackedKeyboard_TrackedKeyboardState {
    #[default]
    Error = 6i32,
    ErrorExtensionFailed = 7i32,
    NoTrackableKeyboard = 1i32,
    Offline = 2i32,
    Stale = 4i32,
    StartedNotTracked = 3i32,
    Uninitialized = 0i32,
    Valid = 5i32,
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TrackedKeyboardState";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState {
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
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    pub ActiveKeyboardName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub State: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    pub TrackingTimeout: bool,
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TrackedKeyboardVisibilityChangedEvent";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
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
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
impl crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    pub fn _ctor(
        &mut self,
        keyboardModel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
        timeout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyboardModel, state, timeout),
        )?;
        Ok(__cordl_ret.into())
    }
}
