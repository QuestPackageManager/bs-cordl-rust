#[cfg(feature = "OVRTrackedKeyboardHands")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTrackedKeyboardHands {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub LeftHandPresence: *mut crate::UnityEngine::GameObject,
    pub RightHandPresence: *mut crate::UnityEngine::GameObject,
    pub handPresenceInitialized_: bool,
    pub leftHandRoot_: *mut crate::UnityEngine::Transform,
    pub rightHandRoot_: *mut crate::UnityEngine::Transform,
    pub KeyboardTracker: *mut crate::GlobalNamespace::OVRTrackedKeyboard,
    pub cameraRig_: *mut crate::GlobalNamespace::OVRCameraRig,
    pub leftHand_: *mut crate::GlobalNamespace::OVRHand,
    pub leftHandSkeleton_: *mut crate::GlobalNamespace::OVRSkeleton,
    pub leftHandSkeletonRenderer_: *mut crate::GlobalNamespace::OVRSkeletonRenderer,
    pub leftHandSkeletonRendererGO_: *mut crate::UnityEngine::GameObject,
    pub leftHandSkinnedMeshRenderer_: *mut crate::UnityEngine::SkinnedMeshRenderer,
    pub leftHandMeshRenderer_: *mut crate::GlobalNamespace::OVRMeshRenderer,
    pub rightHand_: *mut crate::GlobalNamespace::OVRHand,
    pub rightHandSkeleton_: *mut crate::GlobalNamespace::OVRSkeleton,
    pub rightHandSkeletonRenderer_: *mut crate::GlobalNamespace::OVRSkeletonRenderer,
    pub rightHandSkeletonRendererGO_: *mut crate::UnityEngine::GameObject,
    pub rightHandMeshRenderer_: *mut crate::GlobalNamespace::OVRMeshRenderer,
    pub rightHandSkinnedMeshRenderer_: *mut crate::UnityEngine::SkinnedMeshRenderer,
    pub _RightHandOverKeyboard_k__BackingField: bool,
    pub _LeftHandOverKeyboard_k__BackingField: bool,
    pub lastVisibilityEvent_: crate::System::Nullable_1<
        crate::GlobalNamespace::OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent,
    >,
    pub boneMappings_: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRTrackedKeyboardHands_HandBoneMapping,
    >,
    pub HandsMaterial: *mut crate::UnityEngine::Material,
    pub keyboardPositionID_: i32,
    pub keyboardRotationID_: i32,
    pub keyboardScaleID_: i32,
}
#[cfg(feature = "OVRTrackedKeyboardHands")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTrackedKeyboardHands => ""
    ."OVRTrackedKeyboardHands"
);
#[cfg(feature = "OVRTrackedKeyboardHands")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTrackedKeyboardHands {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTrackedKeyboardHands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands")]
impl crate::GlobalNamespace::OVRTrackedKeyboardHands {
    pub const FORWARD_OFFSET: f32 = -0.02f32;
    pub const XSCALE: f32 = 0.73f32;
    pub const YSCALE: f32 = 0.8f32;
    #[cfg(feature = "OVRTrackedKeyboardHands+HandBoneMapping")]
    pub type HandBoneMapping = crate::GlobalNamespace::OVRTrackedKeyboardHands_HandBoneMapping;
    #[cfg(
        feature = "OVRTrackedKeyboardHands+TrackedKeyboardHandsVisibilityChangedEvent"
    )]
    pub type TrackedKeyboardHandsVisibilityChangedEvent = crate::GlobalNamespace::OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOpacity(
        &mut self,
        distance: f32,
        innerThreshold: f32,
        outerThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeOpacity", (distance, innerThreshold, outerThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableHandObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableHandObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandDistanceToKeyboard(
        &mut self,
        handSkeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetHandDistanceToKeyboard", (handSkeleton))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RetargetHandTrackingToHandPresence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RetargetHandTrackingToHandPresence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHandModelsEnabled(
        &mut self,
        enableLeftModel: bool,
        enableRightModel: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHandModelsEnabled", (enableLeftModel, enableRightModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldEnableModel(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldEnableModel", (distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldEnablePassthrough(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldEnablePassthrough", (distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopHandPresence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopHandPresence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackedKeyboardActiveUpdated(
        &mut self,
        e: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardSetActiveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackedKeyboardActiveUpdated", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackedKeyboardVisibilityChanged(
        &mut self,
        e: crate::GlobalNamespace::OVRTrackedKeyboard_TrackedKeyboardVisibilityChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackedKeyboardVisibilityChanged", (e))?;
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
    pub fn get_AreControllersActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AreControllersActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LeftHandOverKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_LeftHandOverKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RightHandOverKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RightHandOverKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LeftHandOverKeyboard(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LeftHandOverKeyboard", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RightHandOverKeyboard(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RightHandOverKeyboard", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTrackedKeyboardHands {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands+HandBoneMapping")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTrackedKeyboardHands_HandBoneMapping {
    pub LeftHandTransform: *mut crate::UnityEngine::Transform,
    pub LeftPresenceTransform: *mut crate::UnityEngine::Transform,
    pub RightHandTransform: *mut crate::UnityEngine::Transform,
    pub RightPresenceTransform: *mut crate::UnityEngine::Transform,
    pub BoneName: crate::GlobalNamespace::OVRSkeleton_BoneId,
    pub HandPresenceLeftBoneName: *mut quest_hook::libil2cpp::Il2CppString,
    pub HandPresenceRightBoneName: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "OVRTrackedKeyboardHands+HandBoneMapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboardHands_HandBoneMapping => ""
    ."OVRTrackedKeyboardHands/HandBoneMapping"
);
#[cfg(feature = "OVRTrackedKeyboardHands+HandBoneMapping")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTrackedKeyboardHands_HandBoneMapping {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands+HandBoneMapping")]
impl crate::GlobalNamespace::OVRTrackedKeyboardHands_HandBoneMapping {}
#[cfg(feature = "OVRTrackedKeyboardHands+TrackedKeyboardHandsVisibilityChangedEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent {
    pub leftVisible: bool,
    pub rightVisible: bool,
}
#[cfg(feature = "OVRTrackedKeyboardHands+TrackedKeyboardHandsVisibilityChangedEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent
    => ""."OVRTrackedKeyboardHands/TrackedKeyboardHandsVisibilityChangedEvent"
);
#[cfg(feature = "OVRTrackedKeyboardHands+TrackedKeyboardHandsVisibilityChangedEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTrackedKeyboardHands+TrackedKeyboardHandsVisibilityChangedEvent")]
impl crate::GlobalNamespace::OVRTrackedKeyboardHands_TrackedKeyboardHandsVisibilityChangedEvent {}
