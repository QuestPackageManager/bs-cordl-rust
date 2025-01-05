#[cfg(feature = "NoteCutCoreEffectsSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutCoreEffectsSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _shockWaveYPos: f32,
    pub _noteCutParticlesEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutParticlesEffect,
    >,
    pub _noteDebrisSpawner: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebrisSpawner,
    >,
    pub _noteCutHapticEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutHapticEffect,
    >,
    pub _shockwaveEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ShockwaveEffect,
    >,
    pub _bombExplosionEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BombExplosionEffect,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _sliderInteractionManagers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderInteractionManager>,
        >,
    >,
    pub _colorTypeToSliderInteractionManagerDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::ColorType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderInteractionManager>,
        >,
    >,
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutCoreEffectsSpawner => ""
    ."NoteCutCoreEffectsSpawner"
);
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    pub const kBurstSliderElementParticlesCount: i32 = 20i32;
    pub const kBurstSliderElementSparkleParticlesCount: i32 = 50i32;
    pub const kNormalNoteExplosionParticlesCount: i32 = 50i32;
    pub const kNormalNoteSparkleParticlesCount: i32 = 150i32;
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
    pub fn IsArcHapticsCurrentlyActive(
        &mut self,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsArcHapticsCurrentlyActive", (colorType))?;
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
    pub fn PlayHitChainNoteHapticEffect(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
        isChainHead: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHitChainNoteHapticEffect", (noteCutInfo, isChainHead))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayHitNoteHapticEffect(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHitNoteHapticEffect", (noteCutInfo, noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnBombCutEffect(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnBombCutEffect", (noteCutInfo, noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnNoteCutEffect(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        sparkleParticlesCount: i32,
        explosionParticlesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SpawnNoteCutEffect",
                (
                    noteCutInfo,
                    noteController,
                    sparkleParticlesCount,
                    explosionParticlesCount,
                ),
            )?;
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
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
