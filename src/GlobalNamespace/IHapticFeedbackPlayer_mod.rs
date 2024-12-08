#[cfg(feature = "IHapticFeedbackPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct IHapticFeedbackPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IHapticFeedbackPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IHapticFeedbackPlayer => ""."IHapticFeedbackPlayer"
);
#[cfg(feature = "IHapticFeedbackPlayer")]
impl std::ops::Deref for IHapticFeedbackPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IHapticFeedbackPlayer")]
impl std::ops::DerefMut for IHapticFeedbackPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IHapticFeedbackPlayer")]
impl IHapticFeedbackPlayer {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IHapticFeedbackPlayer")]
impl quest_hook::libil2cpp::ObjectType for IHapticFeedbackPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}