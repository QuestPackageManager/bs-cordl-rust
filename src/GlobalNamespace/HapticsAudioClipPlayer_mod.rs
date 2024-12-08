#[cfg(feature = "HapticsAudioClipPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct HapticsAudioClipPlayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _baseVolume: f32,
    pub _triggeredThisFrame: bool,
    pub _lastTriggerTime: f32,
}
#[cfg(feature = "HapticsAudioClipPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HapticsAudioClipPlayer => ""."HapticsAudioClipPlayer"
);
#[cfg(feature = "HapticsAudioClipPlayer")]
impl std::ops::Deref for HapticsAudioClipPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer")]
impl std::ops::DerefMut for HapticsAudioClipPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer")]
impl HapticsAudioClipPlayer {
    pub const kContinuousRumbleFadeDuration: f32 = 0.016666668f32;
    #[cfg(feature = "HapticsAudioClipPlayer+_HandleContinuousAudioCoroutine_d__9")]
    pub type _HandleContinuousAudioCoroutine_d__9 = crate::GlobalNamespace::HapticsAudioClipPlayer__HandleContinuousAudioCoroutine_d__9;
    #[cfg(feature = "HapticsAudioClipPlayer+_HandleOneShotPlayEndCoroutine_d__11")]
    pub type _HandleOneShotPlayEndCoroutine_d__11 = crate::GlobalNamespace::HapticsAudioClipPlayer__HandleOneShotPlayEndCoroutine_d__11;
    #[cfg(feature = "HapticsAudioClipPlayer+Pool")]
    pub type Pool = crate::GlobalNamespace::HapticsAudioClipPlayer_Pool;
    pub fn ForceStopPlaying(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceStopPlaying", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPanForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPanForNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleContinuousAudioCoroutine(
        &mut self,
        onComplete: *mut crate::System::Action_1<*mut HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("HandleContinuousAudioCoroutine", (onComplete))?;
        Ok(__cordl_ret)
    }
    pub fn HandleOneShotPlayEndCoroutine(
        &mut self,
        onComplete: *mut crate::System::Action_1<*mut HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("HandleOneShotPlayEndCoroutine", (onComplete))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PlayHapticsPreset(
        &mut self,
        onNode: crate::UnityEngine::XR::XRNode,
        preset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        onComplete: *mut crate::System::Action_1<*mut HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticsPreset", (onNode, preset, onComplete))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn RestartHaptic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartHaptic", ())?;
        Ok(__cordl_ret)
    }
    pub fn TriggerContinuousHaptic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerContinuousHaptic", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFadeVolume(
        &mut self,
        timeSinceEnd: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFadeVolume", (timeSinceEnd))?;
        Ok(__cordl_ret)
    }
    pub fn _HandleOneShotPlayEndCoroutine_b__11_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<HandleOneShotPlayEndCoroutine>b__11_0", ())?;
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
#[cfg(feature = "HapticsAudioClipPlayer")]
impl quest_hook::libil2cpp::ObjectType for HapticsAudioClipPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct HapticsAudioClipPlayer_Pool {
    __cordl_parent: crate::Zenject::MemoryPool_1<*mut HapticsAudioClipPlayer>,
}
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HapticsAudioClipPlayer_Pool =>
    ""."HapticsAudioClipPlayer/Pool"
);
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::HapticsAudioClipPlayer_Pool {
    type Target = crate::Zenject::MemoryPool_1<*mut HapticsAudioClipPlayer>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::HapticsAudioClipPlayer_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
impl crate::GlobalNamespace::HapticsAudioClipPlayer_Pool {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnCreated(
        &mut self,
        item: *mut HapticsAudioClipPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCreated", (item))?;
        Ok(__cordl_ret)
    }
    pub fn OnDespawned(
        &mut self,
        item: *mut HapticsAudioClipPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDespawned", (item))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroyed(
        &mut self,
        item: *mut HapticsAudioClipPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroyed", (item))?;
        Ok(__cordl_ret)
    }
    pub fn Reinitialize(
        &mut self,
        clipPlayer: *mut HapticsAudioClipPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reinitialize", (clipPlayer))?;
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
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HapticsAudioClipPlayer_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
