#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5AdvancedHapticsPlayerController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _hapticsPlayerPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticsAudioClipPlayer_Pool,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _activePlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                crate::UnityEngine::XR::XRNode,
                *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
            >,
            *mut crate::GlobalNamespace::HapticsAudioClipPlayer,
        >,
    >,
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PS5AdvancedHapticsPlayerController => ""
    ."PS5AdvancedHapticsPlayerController"
);
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl std::ops::Deref for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
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
    pub fn OnHapticPlayFinishedCallback(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHapticPlayFinishedCallback", (player))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayContinuousHapticPreset(
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
            .invoke("PlayContinuousHapticPreset", (node, hapticPreset))?;
        Ok(__cordl_ret.into())
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
    pub fn PlayOneShotHapticPreset(
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
            .invoke("PlayOneShotHapticPreset", (node, hapticPreset))?;
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
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsRef<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsMut<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
