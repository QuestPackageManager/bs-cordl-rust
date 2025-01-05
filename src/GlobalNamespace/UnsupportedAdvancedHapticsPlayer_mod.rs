#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsupportedAdvancedHapticsPlayer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnsupportedAdvancedHapticsPlayer => ""
    ."UnsupportedAdvancedHapticsPlayer"
);
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
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
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHapticFeedbackPlayer>>
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHapticFeedbackPlayer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHapticFeedbackPlayer>>
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHapticFeedbackPlayer> {
        unsafe { std::mem::transmute(self) }
    }
}
