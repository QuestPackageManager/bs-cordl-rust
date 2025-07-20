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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutCoreEffectsSpawner";
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
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutCoreEffectsSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutCoreEffectsSpawner {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::NoteController,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::NoteCutInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleNoteWasCut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteWasCut", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteController, noteCutInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsArcHapticsCurrentlyActive(
        &mut self,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::ColorType),
                        bool,
                        1usize,
                    >("IsArcHapticsCurrentlyActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsArcHapticsCurrentlyActive", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (colorType))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDestroy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlayHitChainNoteHapticEffect(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
        isChainHead: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::NoteCutInfo, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PlayHitChainNoteHapticEffect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PlayHitChainNoteHapticEffect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteCutInfo, isChainHead))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlayHitNoteHapticEffect(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::NoteCutInfo,
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PlayHitNoteHapticEffect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PlayHitNoteHapticEffect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteCutInfo, noteData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpawnBombCutEffect(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::NoteCutInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::NoteController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SpawnBombCutEffect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SpawnBombCutEffect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteCutInfo, noteController))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::NoteCutInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::NoteController,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SpawnNoteCutEffect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SpawnNoteCutEffect", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        noteCutInfo,
                        noteController,
                        sparkleParticlesCount,
                        explosionParticlesCount,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
