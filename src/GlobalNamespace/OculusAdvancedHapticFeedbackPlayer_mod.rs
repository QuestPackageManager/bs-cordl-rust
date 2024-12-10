#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusAdvancedHapticFeedbackPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub _oculusVRHelper: *mut crate::GlobalNamespace::OculusVRHelper,
    pub _hapticPlayerStatesDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<
            *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
            crate::UnityEngine::XR::XRNode,
        >,
        *mut crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState,
    >,
    pub _hasAtLeastOneLoopingClipPlaying: bool,
    pub _isLeftHandSupported: crate::System::Nullable_1<bool>,
    pub _isRightHandSupported: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer => ""
    ."OculusAdvancedHapticFeedbackPlayer"
);
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    #[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
    pub type HapticPlayerState = crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState;
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanPlayHapticPreset", (hapticPreset, node))?;
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
    pub fn GetHapticClip(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClip> = __cordl_object
            .invoke("GetHapticClip", (node, hapticPreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationQuitting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplicationQuitting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticFeedback", (node, hapticPreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub player: *mut crate::Oculus::Haptics::HapticClipPlayer,
    pub lastFrameTriggered: i32,
    pub isPlayingLoopingClip: bool,
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState => ""
    ."OculusAdvancedHapticFeedbackPlayer/HapticPlayerState"
);
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
