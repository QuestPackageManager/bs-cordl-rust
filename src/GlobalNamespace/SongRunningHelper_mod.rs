#[cfg(feature = "cordl_class_SongRunningHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SongRunningHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapCharacteristics: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicCollection,
    >,
    pub _menuTransitionsHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuTransitionsHelper,
    >,
    pub _beatmapLevels: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _environmentsListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _playQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
        >,
    >,
    pub _current: crate::System::Nullable_1<
        crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    >,
}
#[cfg(feature = "cordl_class_SongRunningHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SongRunningHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongRunningHelper";
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
#[cfg(feature = "SongRunningHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SongRunningHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongRunningHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl crate::GlobalNamespace::SongRunningHelper {
    #[cfg(feature = "SongRunningHelper+MessageStrings")]
    pub type MessageStrings = crate::GlobalNamespace::SongRunningHelper_MessageStrings;
    #[cfg(feature = "SongRunningHelper+QueuedSongParams")]
    pub type QueuedSongParams = crate::GlobalNamespace::SongRunningHelper_QueuedSongParams;
    pub fn EnqueueLevel(
        &mut self,
        queuedSongParams: crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::SongRunningHelper_QueuedSongParams),
                        bool,
                        1usize,
                    >("EnqueueLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnqueueLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (queuedSongParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindBeatmapLevelPackBeatmaps(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >,
                        2usize,
                    >("FindBeatmapLevelPackBeatmaps")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindBeatmapLevelPackBeatmaps", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (packId, ignoreCase))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerCanPause(
        &mut self,
        canPause: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandlePauseControllerCanPause")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandlePauseControllerCanPause", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (canPause))?
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
    pub fn StartLevel(
        &mut self,
        queuedSongParams: crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::SongRunningHelper_QueuedSongParams),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        1usize,
                    >("StartLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (queuedSongParams))? };
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
    pub fn get_SongsRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_SongsRunning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SongsRunning", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongRunningHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+MessageStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct SongRunningHelper_MessageStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_SongRunningHelper+MessageStrings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SongRunningHelper_MessageStrings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongRunningHelper/MessageStrings";
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
#[cfg(feature = "SongRunningHelper+MessageStrings")]
impl std::ops::Deref for crate::GlobalNamespace::SongRunningHelper_MessageStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper+MessageStrings")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongRunningHelper_MessageStrings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper+MessageStrings")]
impl crate::GlobalNamespace::SongRunningHelper_MessageStrings {
    pub const kFmtMessageSongStarted: &'static str = "RunSong: Starting song {0}";
    pub const kFmtMessageSongsRemaining: &'static str = "RunSong: -- There are {0} songs in queue...";
    pub const kLogPrefix: &'static str = "RunSong: ";
    pub const kMessageQueueFinished: &'static str = "RunSong: ---Finished the runsong level queue---";
    pub const kMessageQueueStarted: &'static str = "RunSong: ---Starting the runsong level queue---";
    pub const kMessageSongFinished: &'static str = "RunSong: -- Finished song";
    pub const kMessageSongStarted: &'static str = "RunSong: -- Started song";
}
#[cfg(feature = "cordl_class_SongRunningHelper+MessageStrings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongRunningHelper_MessageStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SongRunningHelper_QueuedSongParams {
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub advancedHud: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    pub recordingToolData: crate::GlobalNamespace::RecordingToolManager_SetupData,
    pub songFinishedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        >,
    >,
    pub zenMode: bool,
    pub disablePause: bool,
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongRunningHelper/QueuedSongParams";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
impl crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    pub fn _ctor(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        advancedHud: bool,
        songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
        recordingToolData: crate::GlobalNamespace::RecordingToolManager_SetupData,
        zenMode: bool,
        disablePause: bool,
        songFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            bool,
                            crate::GlobalNamespace::GameplayModifiers_SongSpeed,
                            crate::GlobalNamespace::RecordingToolManager_SetupData,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LevelCompletionResults,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        beatmapLevel,
                        characteristic,
                        difficulty,
                        advancedHud,
                        songSpeed,
                        recordingToolData,
                        zenMode,
                        disablePause,
                        songFinishedCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
