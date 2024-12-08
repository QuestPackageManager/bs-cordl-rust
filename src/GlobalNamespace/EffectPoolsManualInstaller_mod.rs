#[cfg(feature = "EffectPoolsManualInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct EffectPoolsManualInstaller {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flyingTextEffectPrefab: *mut FlyingTextEffect,
    pub _flyingScoreEffectPrefab: *mut FlyingScoreEffect,
    pub _beatEffectPrefab: *mut BeatEffect,
    pub _shortBeatEffectPrefab: *mut BeatEffect,
    pub _noteCutSoundEffectPrefab: *mut NoteCutSoundEffect,
    pub _bombCutSoundEffectPrefab: *mut BombCutSoundEffect,
    pub _flyingSpriteEffectPrefab: *mut FlyingSpriteEffect,
}
#[cfg(feature = "EffectPoolsManualInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EffectPoolsManualInstaller => ""
    ."EffectPoolsManualInstaller"
);
#[cfg(feature = "EffectPoolsManualInstaller")]
impl std::ops::Deref for EffectPoolsManualInstaller {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EffectPoolsManualInstaller")]
impl std::ops::DerefMut for EffectPoolsManualInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EffectPoolsManualInstaller")]
impl EffectPoolsManualInstaller {
    pub fn ManualInstallBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        shortBeatEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualInstallBindings", (container, shortBeatEffect))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "EffectPoolsManualInstaller")]
impl quest_hook::libil2cpp::ObjectType for EffectPoolsManualInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
