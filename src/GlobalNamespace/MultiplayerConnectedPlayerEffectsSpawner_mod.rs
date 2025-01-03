#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerEffectsSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _noteDebrisSpawner: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebrisSpawner,
    >,
    pub _bombExplosionEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BombExplosionEffect,
    >,
    pub _beatmapObjectEventManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerEffectsSpawner => ""
    ."MultiplayerConnectedPlayerEffectsSpawner"
);
#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerEffectsSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerEffectsSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerEffectsSpawner {
    pub fn HandleBeatmapObjectEventManagerConnectedPlayerBeatmapObjectWasCut(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapObjectEventManagerConnectedPlayerBeatmapObjectWasCut",
                (noteCutInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
#[cfg(feature = "MultiplayerConnectedPlayerEffectsSpawner")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerEffectsSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
