#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
#[repr(C)]
#[derive(Debug)]
pub struct RumbleHapticFeedbackPlayer_RumbleData {
    __cordl_parent: crate::System::Object,
    pub active: bool,
    pub continuous: bool,
    pub strength: f32,
    pub endTime: f32,
    pub frequency: f32,
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData => ""
    ."RumbleHapticFeedbackPlayer/RumbleData"
);
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl std::ops::Deref for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct RumbleHapticFeedbackPlayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub _rumblesByNode: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::XR::XRNode,
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Object,
            *mut crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
        >,
    >,
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RumbleHapticFeedbackPlayer =>
    ""."RumbleHapticFeedbackPlayer"
);
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    pub const kContinuousRumbleFrameDuration: f32 = 0.016666668f32;
    #[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
    pub type RumbleData = crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData;
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanPlayHapticPreset", (hapticPreset, node))?;
        Ok(__cordl_ret)
    }
    pub fn GetRumble(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        preset: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData = __cordl_object
            .invoke("GetRumble", (node, preset))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticFeedback", (node, hapticPreset))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateRumbles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRumbles", ())?;
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
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
