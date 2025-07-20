#[cfg(feature = "NoteCutSoundEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _audioSamplesBeatAlignOffset: f32,
    pub _longCutEffectsAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _shortCutEffectsAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _testAudioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffectManager_InitData,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _noteCutSoundEffectPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect_Pool,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _handleWrongSaberTypeAsGood_k__BackingField: bool,
    pub _randomLongCutSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _randomShortCutSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _prevNoteATime: f32,
    pub _prevNoteBTime: f32,
    pub _prevNoteASoundEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect,
    >,
    pub _prevNoteBSoundEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect,
    >,
    pub _beatAlignOffset: f32,
    pub _useTestAudioClips: bool,
    pub _noteCutSoundEffectPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutSoundEffect>,
        >,
    >,
}
#[cfg(feature = "NoteCutSoundEffectManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutSoundEffectManager";
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
#[cfg(feature = "NoteCutSoundEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl crate::GlobalNamespace::NoteCutSoundEffectManager {
    pub const kDenseNotesVolumeMul: f32 = 0.9f32;
    pub const kMaxNumberOfEffects: i32 = 64i32;
    pub const kTwoNotesAtTheSameTimeVolumeMul: f32 = 0.9f32;
    #[cfg(feature = "NoteCutSoundEffectManager+InitData")]
    pub type InitData = crate::GlobalNamespace::NoteCutSoundEffectManager_InitData;
    pub fn HandleNoteCutSoundEffectDidFinish(
        &mut self,
        noteCutSoundEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutSoundEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::NoteCutSoundEffect,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleNoteCutSoundEffectDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteCutSoundEffectDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteCutSoundEffect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteDidStartDissolving(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::NoteControllerBase,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleNoteDidStartDissolving")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteDidStartDissolving", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteController))?
        };
        Ok(__cordl_ret.into())
    }
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
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::NoteController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleNoteWasSpawned")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteWasSpawned", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSupportedNote(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>),
                        bool,
                        1usize,
                    >("IsSupportedNote")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSupportedNote", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LateUpdate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    pub fn get_handleWrongSaberTypeAsGood(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_handleWrongSaberTypeAsGood")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_handleWrongSaberTypeAsGood", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_handleWrongSaberTypeAsGood(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_handleWrongSaberTypeAsGood")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_handleWrongSaberTypeAsGood", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl AsRef<crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent>
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl AsMut<crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent>
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub useTestAudioClips: bool,
    pub ignoreBadCuts: bool,
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutSoundEffectManager/InitData";
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
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    pub fn New(
        useTestAudioClips: bool,
        ignoreBadCuts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useTestAudioClips, ignoreBadCuts))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        useTestAudioClips: bool,
        ignoreBadCuts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (useTestAudioClips, ignoreBadCuts))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
