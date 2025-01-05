#[cfg(feature = "BombCutSoundEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BombCutSoundEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _volume: f32,
    pub _bombExplosionAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::AudioClip>,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _bombCutSoundEffectPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BombCutSoundEffect_Pool,
    >,
    pub _randomSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
}
#[cfg(feature = "BombCutSoundEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombCutSoundEffectManager => ""
    ."BombCutSoundEffectManager"
);
#[cfg(feature = "BombCutSoundEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::BombCutSoundEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BombCutSoundEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::BombCutSoundEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BombCutSoundEffectManager")]
impl crate::GlobalNamespace::BombCutSoundEffectManager {
    pub fn HandleBombCutSoundEffectDidFinish(
        &mut self,
        bombCutSoundEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BombCutSoundEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBombCutSoundEffectDidFinish", (bombCutSoundEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
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
#[cfg(feature = "BombCutSoundEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BombCutSoundEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
