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
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HapticsAudioClipPlayer => ""
    ."HapticsAudioClipPlayer"
);
#[cfg(feature = "HapticsAudioClipPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::HapticsAudioClipPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::HapticsAudioClipPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HapticsAudioClipPlayer")]
impl crate::GlobalNamespace::HapticsAudioClipPlayer {
    pub const kContinuousRumbleFadeDuration: f32 = 0.016666668f32;
    #[cfg(feature = "HapticsAudioClipPlayer+Pool")]
    pub type Pool = crate::GlobalNamespace::HapticsAudioClipPlayer_Pool;
    #[cfg(feature = "HapticsAudioClipPlayer+_HandleContinuousAudioCoroutine_d__9")]
    pub type _HandleContinuousAudioCoroutine_d__9 = crate::GlobalNamespace::HapticsAudioClipPlayer__HandleContinuousAudioCoroutine_d__9;
    #[cfg(feature = "HapticsAudioClipPlayer+_HandleOneShotPlayEndCoroutine_d__11")]
    pub type _HandleOneShotPlayEndCoroutine_d__11 = crate::GlobalNamespace::HapticsAudioClipPlayer__HandleOneShotPlayEndCoroutine_d__11;
    pub fn ForceStopPlaying(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceStopPlaying", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPanForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPanForNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleContinuousAudioCoroutine(
        &mut self,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::HapticsAudioClipPlayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("HandleContinuousAudioCoroutine", (onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleOneShotPlayEndCoroutine(
        &mut self,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::HapticsAudioClipPlayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("HandleOneShotPlayEndCoroutine", (onComplete))?;
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
    pub fn PlayHapticsPreset(
        &mut self,
        onNode: crate::UnityEngine::XR::XRNode,
        preset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::HapticsAudioClipPlayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticsPreset", (onNode, preset, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RestartHaptic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartHaptic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerContinuousHaptic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerContinuousHaptic", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _HandleOneShotPlayEndCoroutine_b__11_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<HandleOneShotPlayEndCoroutine>b__11_0", ())?;
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
#[cfg(feature = "HapticsAudioClipPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HapticsAudioClipPlayer {
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
    __cordl_parent: crate::Zenject::MemoryPool_1<
        *mut crate::GlobalNamespace::HapticsAudioClipPlayer,
    >,
}
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HapticsAudioClipPlayer_Pool =>
    ""."HapticsAudioClipPlayer/Pool"
);
#[cfg(feature = "HapticsAudioClipPlayer+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::HapticsAudioClipPlayer_Pool {
    type Target = crate::Zenject::MemoryPool_1<
        *mut crate::GlobalNamespace::HapticsAudioClipPlayer,
    >;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCreated(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCreated", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDespawned(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDespawned", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroyed(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroyed", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize(
        &mut self,
        clipPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HapticsAudioClipPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reinitialize", (clipPlayer))?;
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
