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
    pub keyboardModelShader: *mut crate::UnityEngine::Shader,
    pub keyboardModelAlphaBlendShader: *mut crate::UnityEngine::Shader,
    pub currentKeyboardPresentationStyles: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles,
    pub projectedPassthroughOpaque_: *mut crate::GlobalNamespace::OVROverlay,
    pub activeKeyboardRenderers_: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::MeshRenderer,
    >,
    pub activeKeyboardMesh_: *mut crate::UnityEngine::GameObject,
    pub keyboardMeshNodes_: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub activeKeyboardMeshRenderer_: *mut crate::UnityEngine::MeshRenderer,
    pub passthroughQuad_: *mut crate::UnityEngine::GameObject,
    pub opaqueShader_: *mut crate::UnityEngine::Shader,
    pub dynamicQualityTexture_: *mut crate::UnityEngine::Texture2D,
    pub untrackedPosition_: crate::UnityEngine::Vector3,
    pub KeyLabelModeShader: *mut crate::UnityEngine::Shader,
    pub PassthroughShader: *mut crate::UnityEngine::Shader,
    pub projectedPassthroughRoot: *mut crate::UnityEngine::Transform,
    pub projectedPassthroughMesh: *mut crate::UnityEngine::MeshFilter,
    pub ProjectedPassthroughKeyLabel: *mut crate::GlobalNamespace::OVRPassthroughLayer,
    pub TrackedKeyboardActiveChanged: *mut crate::System::Action_1<
        crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent,
    >,
    pub TrackedKeyboardVisibilityChanged: *mut crate::System::Action_1<
        crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent,
    >,
    pub ActiveKeyboardTransform: *mut crate::UnityEngine::Transform,
    pub HandsOverKeyboard: bool,
    pub cameraRig_: *mut crate::GlobalNamespace::OVRCameraRig,
    pub updateKeyboardRoutine_: *mut crate::UnityEngine::Coroutine,
    pub keyboardBoundingBox_: *mut crate::UnityEngine::BoxCollider,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTrackedKeyboard => ""
    ."OVRTrackedKeyboard"
);
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
    #[cfg(feature = "OVRTrackedKeyboard+_InitializeHandPresenceData_d__89")]
    pub type _InitializeHandPresenceData_d__89 = crate::GlobalNamespace::OVRTrackedKeyboard__InitializeHandPresenceData_d__89;
    #[cfg(feature = "OVRTrackedKeyboard+_StartKeyboardTrackingCoroutine_d__96")]
    pub type _StartKeyboardTrackingCoroutine_d__96 = crate::GlobalNamespace::OVRTrackedKeyboard__StartKeyboardTrackingCoroutine_d__96;
    #[cfg(feature = "OVRTrackedKeyboard+_Start_d__88")]
    pub type _Start_d__88 = crate::GlobalNamespace::OVRTrackedKeyboard__Start_d__88;
    #[cfg(feature = "OVRTrackedKeyboard+_UpdateKeyboardPose_d__98")]
    pub type _UpdateKeyboardPose_d__98 = crate::GlobalNamespace::OVRTrackedKeyboard__UpdateKeyboardPose_d__98;
    #[cfg(feature = "OVRTrackedKeyboard+_UpdateTrackingStateCoroutine_d__95")]
    pub type _UpdateTrackingStateCoroutine_d__95 = crate::GlobalNamespace::OVRTrackedKeyboard__UpdateTrackingStateCoroutine_d__95;
    #[cfg(feature = "OVRTrackedKeyboard+__c")]
    pub type __c = crate::GlobalNamespace::OVRTrackedKeyboard___c;
    pub fn DispatchVisibilityEvent(
        &mut self,
        timeOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DispatchVisibilityEvent", (timeOut))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDistanceToKeyboard(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDistanceToKeyboard", (point))?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyboardVisibility(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetKeyboardVisibility", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeHandPresenceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("InitializeHandPresenceData", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeKeyboardInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeKeyboardInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn KeyboardTrackerIsRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("KeyboardTrackerIsRunning", ())?;
        Ok(__cordl_ret)
    }
    pub fn LaunchLocalKeyboardSelectionDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchLocalKeyboardSelectionDialog", ())?;
        Ok(__cordl_ret)
    }
    pub fn LaunchOverlayIntent(
        &mut self,
        dataUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchOverlayIntent", (dataUri))?;
        Ok(__cordl_ret)
    }
    pub fn LaunchRemoteKeyboardSelectionDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchRemoteKeyboardSelectionDialog", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadKeyboardMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadRuntimeKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("LoadRuntimeKeyboardMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterPassthroughMeshToSDK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterPassthroughMeshToSDK", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartKeyboardTrackingCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("StartKeyboardTrackingCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopKeyboardTrackingInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopKeyboardTrackingInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateKeyboardPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateKeyboardPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateKeyboardVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateKeyboardVisibility", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn UpdateSkippedPoseTimer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSkippedPoseTimer", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTextureQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTextureQuality", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTrackingStateCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateTrackingStateCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_ConnectionRequired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ConnectionRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentKeyboardAngleFromUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_CurrentKeyboardAngleFromUp", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_PassthroughOverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::OVROverlay> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVROverlay = __cordl_object
            .invoke("get_PassthroughOverlay", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_RemoteKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RemoteKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShowUntracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShowUntracked", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_TrackingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TrackingEnabled", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_PassthroughOverlay(
        &mut self,
        value: *mut crate::GlobalNamespace::OVROverlay,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PassthroughOverlay", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRTrackedKeyboard_KeyboardPresentation {
    PreferKeyLabels = 1i32,
    PreferOpaque = 0i32,
}
#[cfg(feature = "OVRTrackedKeyboard+KeyboardPresentation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboard_KeyboardPresentation => ""
    ."OVRTrackedKeyboard/KeyboardPresentation"
);
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent {
    pub IsEnabled: bool,
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardSetActiveEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent => ""
    ."OVRTrackedKeyboard/TrackedKeyboardSetActiveEvent"
);
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRTrackedKeyboard_TrackedKeyboardState {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState => ""
    ."OVRTrackedKeyboard/TrackedKeyboardState"
);
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent {
    pub ActiveKeyboardName: *mut crate::System::String,
    pub State: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
    pub TrackingTimeout: bool,
}
#[cfg(feature = "OVRTrackedKeyboard+TrackedKeyboardVisibilityChangedEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent => ""
    ."OVRTrackedKeyboard/TrackedKeyboardVisibilityChangedEvent"
);
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
        keyboardModel: *mut crate::System::String,
        state: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardState,
        timeout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyboardModel, state, timeout),
        )?;
        Ok(__cordl_ret)
    }
}
