#[cfg(feature = "EffectPoolsManualInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct EffectPoolsManualInstaller {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flyingTextEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingTextEffect,
    >,
    pub _flyingScoreEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingScoreEffect,
    >,
    pub _beatEffectPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatEffect>,
    pub _shortBeatEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatEffect,
    >,
    pub _noteCutSoundEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect,
    >,
    pub _bombCutSoundEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BombCutSoundEffect,
    >,
    pub _flyingSpriteEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingSpriteEffect,
    >,
}
#[cfg(feature = "EffectPoolsManualInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EffectPoolsManualInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EffectPoolsManualInstaller";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "EffectPoolsManualInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::EffectPoolsManualInstaller {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EffectPoolsManualInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::EffectPoolsManualInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EffectPoolsManualInstaller")]
impl crate::GlobalNamespace::EffectPoolsManualInstaller {
    pub fn ManualInstallBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        shortBeatEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualInstallBindings", (container, shortBeatEffect))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "EffectPoolsManualInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EffectPoolsManualInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
