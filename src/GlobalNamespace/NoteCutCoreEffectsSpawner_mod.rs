#[cfg(feature = "NoteCutCoreEffectsSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutCoreEffectsSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _shockWaveYPos: f32,
    pub _noteCutParticlesEffect: *mut NoteCutParticlesEffect,
    pub _noteDebrisSpawner: *mut NoteDebrisSpawner,
    pub _noteCutHapticEffect: *mut NoteCutHapticEffect,
    pub _shockwaveEffect: *mut ShockwaveEffect,
    pub _bombExplosionEffect: *mut BombExplosionEffect,
    pub _colorManager: *mut ColorManager,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _sliderInteractionManagers: *mut crate::System::Collections::Generic::List_1<
        *mut SliderInteractionManager,
    >,
    pub _colorTypeToSliderInteractionManagerDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        ColorType,
        *mut SliderInteractionManager,
    >,
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteCutCoreEffectsSpawner => ""
    ."NoteCutCoreEffectsSpawner"
);
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::Deref for NoteCutCoreEffectsSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::DerefMut for NoteCutCoreEffectsSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl NoteCutCoreEffectsSpawner {
    pub const kBurstSliderElementParticlesCount: i32 = 20i32;
    pub const kBurstSliderElementSparkleParticlesCount: i32 = 50i32;
    pub const kNormalNoteExplosionParticlesCount: i32 = 50i32;
    pub const kNormalNoteSparkleParticlesCount: i32 = 150i32;
    pub fn PlayHitChainNoteHapticEffect(
        &mut self,
        noteCutInfo: NoteCutInfo,
        isChainHead: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHitChainNoteHapticEffect", (noteCutInfo, isChainHead))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn SpawnBombCutEffect(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnBombCutEffect", (noteCutInfo, noteController))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SpawnNoteCutEffect(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
        noteController: *mut NoteController,
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
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn PlayHitNoteHapticEffect(
        &mut self,
        noteCutInfo: NoteCutInfo,
        noteData: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHitNoteHapticEffect", (noteCutInfo, noteData))?;
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
    pub fn IsArcHapticsCurrentlyActive(
        &mut self,
        colorType: ColorType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsArcHapticsCurrentlyActive", (colorType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl quest_hook::libil2cpp::ObjectType for NoteCutCoreEffectsSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
