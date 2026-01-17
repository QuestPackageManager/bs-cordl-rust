#[cfg(feature = "cordl_class_NoteDebrisSpawner")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct NoteDebrisSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rotation: f32,
    pub _cutDirMultiplier: f32,
    pub _fromCenterSpeed: f32,
    pub _moveSpeedMultiplier: f32,
    pub _normalNotesDebrisPool: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris_Pool>,
    pub _burstSliderHeadNotesDebrisPool:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris_Pool>,
    pub _burstSliderElementNotesDebrisPool:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris_Pool>,
    pub _random: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRandom>,
    pub _determinismConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DeterminismConfig>,
    pub _poolForNoteGameplayType: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::NoteData_GameplayType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris_Pool>,
        >,
    >,
    pub _poolForNoteDebris: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris_Pool>,
        >,
    >,
}
#[cfg(feature = "cordl_class_NoteDebrisSpawner")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteDebrisSpawner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteDebrisSpawner";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "NoteDebrisSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebrisSpawner {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl crate::GlobalNamespace::NoteDebrisSpawner {
    pub const kLifeTimeOffset: f32 = 0.05f32;
    pub const kMaxLifeTime: f32 = 2f32;
    pub const kMinLifeTime: f32 = 0.2f32;
    pub fn DespawnNoteDebris(
        &mut self,
        noteDebris: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DespawnNoteDebris")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DespawnNoteDebris", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (noteDebris))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteDebrisDidFinish(
        &mut self,
        noteDebris: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleNoteDebrisDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteDebrisDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (noteDebris))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SpawnDebris(
        &mut self,
        noteGameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        saberSpeed: f32,
        saberDir: crate::UnityEngine::Vector3,
        notePos: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        colorType: crate::GlobalNamespace::ColorType,
        timeToNextColorNote: f32,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::NoteData_GameplayType,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                        crate::GlobalNamespace::ColorType,
                        f32,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 11usize>("SpawnDebris")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SpawnDebris",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    noteGameplayType,
                    cutPoint,
                    cutNormal,
                    saberSpeed,
                    saberDir,
                    notePos,
                    noteRotation,
                    noteScale,
                    colorType,
                    timeToNextColorNote,
                    moveVec,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpawnNoteDebris(
        &mut self,
        noteGameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        debris0: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
        >,
        debris1: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::NoteData_GameplayType,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("SpawnNoteDebris")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SpawnNoteDebris",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (noteGameplayType, debris0, debris1))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_NoteDebrisSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteDebrisSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl AsRef<crate::GlobalNamespace::INoteDebrisDidFinishEvent>
    for crate::GlobalNamespace::NoteDebrisSpawner
{
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteDebrisDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl AsMut<crate::GlobalNamespace::INoteDebrisDidFinishEvent>
    for crate::GlobalNamespace::NoteDebrisSpawner
{
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteDebrisDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
