#[cfg(feature = "RecordingToolManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recordingToolEnabled: bool,
    pub performanceRecordingEnabled: bool,
    pub configJsonData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub recordingToolSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingToolSettings,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _configurationProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingToolConfigurationProcessor,
    >,
    pub _menuTransitionsHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuTransitionsHelper,
    >,
    pub _environmentsListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _currentRecordingIndex: i32,
}
#[cfg(feature = "RecordingToolManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolManager";
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
#[cfg(feature = "RecordingToolManager")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingToolManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingToolManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolManager")]
impl crate::GlobalNamespace::RecordingToolManager {
    pub const kRecordingToolId: &'static str = "RecordingTool";
    #[cfg(feature = "RecordingToolManager+SetupData")]
    pub type SetupData = crate::GlobalNamespace::RecordingToolManager_SetupData;
    pub fn New(
        processor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
        diContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        menuTransitionsHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuTransitionsHelper,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    processor,
                    beatmapCharacteristicCollection,
                    diContainer,
                    menuTransitionsHelper,
                    environmentsListModel,
                    playerDataModel,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Run_b__15_0(
        &mut self,
        StandardLevelScenesTransitionSetupDataSO: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
        LevelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<Run>b__15_0",
                (StandardLevelScenesTransitionSetupDataSO, LevelCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        processor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
        diContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        menuTransitionsHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuTransitionsHelper,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    processor,
                    beatmapCharacteristicCollection,
                    diContainer,
                    menuTransitionsHelper,
                    environmentsListModel,
                    playerDataModel,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showRecordingToolScene(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRecordingToolScene", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingToolManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RecordingToolManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolManager+SetupData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecordingToolManager_SetupData {
    pub profileSong: bool,
    pub runAutopilot: bool,
}
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolManager_SetupData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolManager/SetupData";
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
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::RecordingToolManager_SetupData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::RecordingToolManager_SetupData {
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
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::RecordingToolManager_SetupData {
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
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::RecordingToolManager_SetupData {
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
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::RecordingToolManager_SetupData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "RecordingToolManager+SetupData")]
impl crate::GlobalNamespace::RecordingToolManager_SetupData {}
