#[cfg(feature = "SongRunningHelper")]
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
    pub _playQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
        >,
    >,
    pub _current: crate::System::Nullable_1<
        crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    >,
}
#[cfg(feature = "SongRunningHelper")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongRunningHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl crate::GlobalNamespace::SongRunningHelper {
    #[cfg(feature = "SongRunningHelper+QueuedSongParams")]
    pub type QueuedSongParams = crate::GlobalNamespace::SongRunningHelper_QueuedSongParams;
    pub fn EnqueueLevel(
        &mut self,
        queuedSongParams: crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EnqueueLevel", (queuedSongParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBeatmapLevelPackBeatmaps(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        > = __cordl_object.invoke("FindBeatmapLevelPackBeatmaps", (packId))?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLevel", (queuedSongParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopAllLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAllLevels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopCurrentLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCurrentLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _StartLevel_g__HandleLevelDidFinishCallback_9_0(
        &mut self,
        standardLevelScenesTransition: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<StartLevel>g__HandleLevelDidFinishCallback|9_0",
                (standardLevelScenesTransition, results),
            )?;
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
#[cfg(feature = "SongRunningHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongRunningHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
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
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
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
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
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
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
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
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
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
        songFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                beatmapLevel,
                characteristic,
                difficulty,
                advancedHud,
                songSpeed,
                recordingToolData,
                zenMode,
                songFinishedCallback,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
