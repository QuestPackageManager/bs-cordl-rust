#[cfg(feature = "cordl_class_BurstSliderGameNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderGameNoteController {
    __cordl_parent: crate::GlobalNamespace::NoteController,
    pub _bigCuttableBySaberList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BoxCuttableBySaber>,
        >,
    >,
    pub _smallCuttableBySaberList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BoxCuttableBySaber>,
        >,
    >,
    pub _wrapperGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _variableMovementDataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVariableMovementDataProvider,
    >,
    pub cubeNoteControllerDidInitEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BurstSliderGameNoteController,
            >,
        >,
    >,
    pub _noteVisualModifierType: crate::GlobalNamespace::NoteVisualModifierType,
    pub _gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub _bigCuttableBySaber: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoxCuttableBySaber,
    >,
    pub _originalColliderSize: crate::UnityEngine::Vector3,
    pub _originalColliderCenter: crate::UnityEngine::Vector3,
    pub _moveEndOffsetZ: f32,
    pub _jumpEndOffsetZ: f32,
}
#[cfg(feature = "cordl_class_BurstSliderGameNoteController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BurstSliderGameNoteController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BurstSliderGameNoteController";
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
#[cfg(feature = "BurstSliderGameNoteController")]
impl std::ops::Deref for crate::GlobalNamespace::BurstSliderGameNoteController {
    type Target = crate::GlobalNamespace::NoteController;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl std::ops::DerefMut for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl crate::GlobalNamespace::BurstSliderGameNoteController {
    #[cfg(feature = "BurstSliderGameNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::BurstSliderGameNoteController_Pool;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleBigWasCutBySaber(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("HandleBigWasCutBySaber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleBigWasCutBySaber", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (saber, cutPoint, orientation, cutDirVec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleCut(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
        allowBadCut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Vector3,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("HandleCut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleCut", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (saber, cutPoint, orientation, cutDirVec, allowBadCut),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSmallWasCutBySaber(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("HandleSmallWasCutBySaber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleSmallWasCutBySaber", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (saber, cutPoint, orientation, cutDirVec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HiddenStateDidChange(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HiddenStateDidChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HiddenStateDidChange", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (hide))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        noteVisualModifierType: crate::GlobalNamespace::NoteVisualModifierType,
        uniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::NoteSpawnData,
                            >,
                            crate::GlobalNamespace::NoteVisualModifierType,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (noteData, noteSpawnData, noteVisualModifierType, uniformScale),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ManualUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ManualUpdate", 0usize
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
    pub fn NoteDidPassMissedMarker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("NoteDidPassMissedMarker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NoteDidPassMissedMarker", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NoteDidStartDissolving(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("NoteDidStartDissolving")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NoteDidStartDissolving", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NoteDidStartJump(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("NoteDidStartJump")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NoteDidStartJump", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
    pub fn Pause(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Pause")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Pause",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (pause))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBigCuttableColliderSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SetBigCuttableColliderSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBigCuttableColliderSize", 0usize
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
    pub fn add_cubeNoteControllerDidInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BurstSliderGameNoteController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BurstSliderGameNoteController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_cubeNoteControllerDidInitEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_cubeNoteControllerDidInitEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::NoteData_GameplayType,
                        0usize,
                    >("get_gameplayType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_gameplayType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteMovement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteMovement>,
                        0usize,
                    >("get_noteMovement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_noteMovement", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMovement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteVisualModifierType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::NoteVisualModifierType,
                        0usize,
                    >("get_noteVisualModifierType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_noteVisualModifierType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteVisualModifierType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_cubeNoteControllerDidInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BurstSliderGameNoteController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BurstSliderGameNoteController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_cubeNoteControllerDidInitEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_cubeNoteControllerDidInitEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BurstSliderGameNoteController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsRef<
    crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >,
> for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsMut<
    crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >,
> for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsRef<crate::GlobalNamespace::IGameNoteMirrorable>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IGameNoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsMut<crate::GlobalNamespace::IGameNoteMirrorable>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IGameNoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsRef<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsMut<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsRef<crate::GlobalNamespace::INoteMovementProvider>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteMovementProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsMut<crate::GlobalNamespace::INoteMovementProvider>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteMovementProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsRef<crate::GlobalNamespace::INoteVisualModifierTypeProvider>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteVisualModifierTypeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl AsMut<crate::GlobalNamespace::INoteVisualModifierTypeProvider>
for crate::GlobalNamespace::BurstSliderGameNoteController {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteVisualModifierTypeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_BurstSliderGameNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderGameNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >,
}
#[cfg(feature = "cordl_class_BurstSliderGameNoteController+Pool")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BurstSliderGameNoteController/Pool";
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
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
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
#[cfg(feature = "cordl_class_BurstSliderGameNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
