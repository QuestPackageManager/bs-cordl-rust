#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveData_EnabledObstacleType {
    #[default]
    All = 0i32,
    FullHeightOnly = 1i32,
    None = 2i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/GameplayModifiers/EnabledObstacleType";
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveData_EnergyType {
    #[default]
    Bar = 0i32,
    Battery = 1i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/GameplayModifiers/EnergyType";
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveData_SongSpeed {
    #[default]
    Faster = 1i32,
    Normal = 0i32,
    Slower = 2i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/GameplayModifiers/SongSpeed";
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData {
    __cordl_parent: crate::GlobalNamespace::VersionSaveData,
    pub localPlayers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData_LocalPlayer>,
    >,
    pub guestPlayers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData_GuestPlayer>,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PlayerSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData";
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
#[cfg(feature = "PlayerSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData {
    type Target = crate::GlobalNamespace::VersionSaveData;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData")]
impl crate::GlobalNamespace::PlayerSaveData {
    pub const kCurrentVersion: &'static str = "2.0.28";
    #[cfg(feature = "PlayerSaveData+ColorOverrideType")]
    pub type ColorOverrideType = crate::GlobalNamespace::PlayerSaveData_ColorOverrideType;
    #[cfg(feature = "PlayerSaveData+ColorScheme")]
    pub type ColorScheme = crate::GlobalNamespace::PlayerSaveData_ColorScheme;
    #[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
    pub type ColorSchemesSettings = crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers")]
    pub type GameplayModifiers = crate::GlobalNamespace::PlayerSaveData_GameplayModifiers;
    #[cfg(feature = "PlayerSaveData+GuestPlayer")]
    pub type GuestPlayer = crate::GlobalNamespace::PlayerSaveData_GuestPlayer;
    #[cfg(feature = "PlayerSaveData+LocalPlayer")]
    pub type LocalPlayer = crate::GlobalNamespace::PlayerSaveData_LocalPlayer;
    #[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
    pub type MultiplayerModeSettings = crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings;
    #[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
    pub type OverrideEnvironmentSettings = crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings;
    #[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
    pub type PlayerAgreementsData = crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData;
    #[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
    pub type PlayerAllOverallStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData;
    #[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
    pub type PlayerLevelStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData;
    #[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
    pub type PlayerMissionStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData;
    #[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
    pub type PlayerOverallStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData;
    #[cfg(feature = "PlayerSaveData+PlayerSensitivityFlagSaveData")]
    pub type PlayerSensitivityFlagSaveData = crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData;
    #[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
    pub type PlayerSpecificSettings = crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings;
    #[cfg(feature = "PlayerSaveData+PracticeSettings")]
    pub type PracticeSettings = crate::GlobalNamespace::PlayerSaveData_PracticeSettings;
    #[cfg(feature = "PlayerSaveData+UserAgeCategorySaveData")]
    pub type UserAgeCategorySaveData = crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData;
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
#[cfg(feature = "cordl_class_PlayerSaveData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSaveData_ColorOverrideType {
    #[default]
    All = 0i32,
    NotesOnly = 1i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_ColorOverrideType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/ColorOverrideType";
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
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSaveData_ColorOverrideType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSaveData_ColorOverrideType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSaveData_ColorOverrideType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+ColorOverrideType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSaveData_ColorOverrideType {
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
#[cfg(feature = "cordl_class_PlayerSaveData+ColorScheme")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_ColorScheme {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub colorSchemeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub saberAColor: crate::UnityEngine::Color,
    pub saberBColor: crate::UnityEngine::Color,
    pub environmentColor0: crate::UnityEngine::Color,
    pub environmentColor1: crate::UnityEngine::Color,
    pub obstaclesColor: crate::UnityEngine::Color,
    pub environmentColor0Boost: crate::UnityEngine::Color,
    pub environmentColor1Boost: crate::UnityEngine::Color,
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorScheme")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/ColorScheme";
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
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    pub fn New(
        colorSchemeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorSchemeId,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    obstaclesColor,
                    environmentColor0Boost,
                    environmentColor1Boost,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        colorSchemeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        colorSchemeId,
                        saberAColor,
                        saberBColor,
                        environmentColor0,
                        environmentColor1,
                        obstaclesColor,
                        environmentColor0Boost,
                        environmentColor1Boost,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorScheme")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorSchemesSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_ColorSchemesSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub overrideDefaultColors: bool,
    pub selectedColorSchemeId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub colorOverrideType: crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
    pub colorSchemes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData_ColorScheme>,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorSchemesSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/ColorSchemesSettings";
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
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    pub fn New(
        overrideDefaultColors: bool,
        selectedColorSchemeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        colorOverrideType: crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
        colorSchemes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData_ColorScheme>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    overrideDefaultColors,
                    selectedColorSchemeId,
                    colorOverrideType,
                    colorSchemes,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        overrideDefaultColors: bool,
        selectedColorSchemeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        colorOverrideType: crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
        colorSchemes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData_ColorScheme>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::PlayerSaveData_ColorScheme,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        overrideDefaultColors,
                        selectedColorSchemeId,
                        colorOverrideType,
                        colorSchemes,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+ColorSchemesSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_GameplayModifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub energyType: crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType,
    pub noFailOn0Energy: bool,
    pub instaFail: bool,
    pub failOnSaberClash: bool,
    pub enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType,
    pub fastNotes: bool,
    pub strictAngles: bool,
    pub disappearingArrows: bool,
    pub ghostNotes: bool,
    pub noBombs: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed,
    pub noArrows: bool,
    pub proMode: bool,
    pub zenMode: bool,
    pub smallCubes: bool,
}
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/GameplayModifiers";
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
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
    pub type EnabledObstacleType = crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnabledObstacleType;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+EnergyType")]
    pub type EnergyType = crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_EnergyType;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiers_PlayerSaveData_SongSpeed;
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
#[cfg(feature = "cordl_class_PlayerSaveData+GameplayModifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+GuestPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_GuestPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_PlayerSaveData+GuestPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/GuestPlayer";
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
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
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
#[cfg(feature = "cordl_class_PlayerSaveData+GuestPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+LocalPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_LocalPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shouldShowTutorialPrompt: bool,
    pub shouldShow360Warning: bool,
    pub agreedToEula: bool,
    pub didSelectLanguage: bool,
    pub agreedToMultiplayerDisclaimer: bool,
    pub didSelectRegionVersion: i32,
    pub selectedAvatarSystemTypeId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub playerAgreements: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData,
    >,
    pub lastSelectedBeatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub lastSelectedBeatmapCharacteristicName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_GameplayModifiers,
    >,
    pub playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings,
    >,
    pub practiceSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PracticeSettings,
    >,
    pub playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
    >,
    pub levelsStatsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData,
        >,
    >,
    pub missionsStatsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData,
        >,
    >,
    pub showedMissionHelpIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub colorSchemesSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings,
    >,
    pub overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings,
    >,
    pub favoritesLevelIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub multiplayerModeSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings,
    >,
    pub currentDlcPromoDisplayCount: i32,
    pub currentDlcPromoId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub userAgeCategory: crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData,
    pub desiredSensitivityFlag: crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData,
}
#[cfg(feature = "cordl_class_PlayerSaveData+LocalPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/LocalPlayer";
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
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
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
#[cfg(feature = "cordl_class_PlayerSaveData+LocalPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+MultiplayerModeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_MultiplayerModeSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub createServerNumberOfPlayers: i32,
    pub quickPlayDifficulty: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub quickPlaySongPackMask: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub quickPlaySongPackMaskSerializedName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub quickPlayEnableLevelSelection: bool,
}
#[cfg(feature = "cordl_class_PlayerSaveData+MultiplayerModeSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/MultiplayerModeSettings";
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
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
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
#[cfg(feature = "cordl_class_PlayerSaveData+MultiplayerModeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+OverrideEnvironmentSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_OverrideEnvironmentSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub overrideEnvironments: bool,
    pub overrideNormalEnvironmentName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub override360EnvironmentName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveData+OverrideEnvironmentSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/OverrideEnvironmentSettings";
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
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
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
#[cfg(feature = "cordl_class_PlayerSaveData+OverrideEnvironmentSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAgreementsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerAgreementsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub eulaVersion: i32,
    pub privacyPolicyVersion: i32,
    pub healthAndSafetyVersion: i32,
    pub playerSensitivityFlagVersion: i32,
    pub endOfLifeVersion: i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAgreementsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerAgreementsData";
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
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAgreementsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAllOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerAllOverallStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub campaignOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    >,
    pub soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    >,
    pub partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    >,
    pub onlinePlayOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAllOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerAllOverallStatsData";
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
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData1(
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        onlinePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    campaignOverallStatsData,
                    soloFreePlayOverallStatsData,
                    partyFreePlayOverallStatsData,
                    onlinePlayOverallStatsData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
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
    pub fn _ctor_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData1(
        &mut self,
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
        onlinePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        campaignOverallStatsData,
                        soloFreePlayOverallStatsData,
                        partyFreePlayOverallStatsData,
                        onlinePlayOverallStatsData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerAllOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerLevelStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerLevelStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub beatmapCharacteristicName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub highScore: i32,
    pub maxCombo: i32,
    pub fullCombo: bool,
    pub maxRank: crate::GlobalNamespace::RankModel_Rank,
    pub validScore: bool,
    pub playCount: i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerLevelStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerLevelStatsData";
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
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerLevelStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerMissionStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cleared: bool,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerMissionStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerMissionStatsData";
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
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerOverallStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub goodCutsCount: i32,
    pub badCutsCount: i32,
    pub missedCutsCount: i32,
    pub totalScore: i64,
    pub playedLevelsCount: i32,
    pub cleardLevelsCount: i32,
    pub failedLevelsCount: i32,
    pub fullComboCount: i32,
    pub timePlayed: f32,
    pub handDistanceTravelled: i32,
    pub cummulativeCutScoreWithoutMultiplier: i64,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerOverallStatsData";
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
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        clearedLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cumulativeCutScoreWithoutMultiplier: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    goodCutsCount,
                    badCutsCount,
                    missedCutsCount,
                    totalScore,
                    playedLevelsCount,
                    clearedLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    cumulativeCutScoreWithoutMultiplier,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
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
    pub fn _ctor_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        &mut self,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        clearedLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cumulativeCutScoreWithoutMultiplier: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, i64, i32, i32, i32, i32, f32, i32, i64),
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        goodCutsCount,
                        badCutsCount,
                        missedCutsCount,
                        totalScore,
                        playedLevelsCount,
                        clearedLevelsCount,
                        failedLevelsCount,
                        fullComboCount,
                        timePlayed,
                        handDistanceTravelled,
                        cumulativeCutScoreWithoutMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSaveData_PlayerSensitivityFlagSaveData {
    #[default]
    Explicit = 3i32,
    Safe = 1i32,
    Themes = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerSensitivityFlagSaveData";
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSensitivityFlagSaveData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerSpecificSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub staticLights: bool,
    pub leftHanded: bool,
    pub playerHeight: f32,
    pub automaticPlayerHeight: bool,
    pub sfxVolume: f32,
    pub reduceDebris: bool,
    pub noTextsAndHuds: bool,
    pub advancedHud: bool,
    pub saberTrailIntensity: f32,
    pub _noteJumpDurationTypeSettingsSaveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
    pub noteJumpFixedDuration: f32,
    pub autoRestart: bool,
    pub noFailEffects: bool,
    pub noteJumpBeatOffset: f32,
    pub hideNoteSpawnEffect: bool,
    pub adaptiveSfx: bool,
    pub arcsHapticFeedback: bool,
    pub arcsVisibleSaveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
    pub environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    pub environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    pub headsetHapticIntensity: f32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerSpecificSettings";
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
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    #[cfg(feature = "PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData")]
    pub type ArcVisibilityTypeSaveData = crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData;
    #[cfg(
        feature = "PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
    )]
    pub type EnvironmentEffectsFilterPresetSaveData = crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData;
    #[cfg(
        feature = "PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
    )]
    pub type NoteJumpDurationTypeSettingsSaveData = crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData;
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
#[cfg(feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+PracticeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PracticeSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub startSongTime: f32,
    pub songSpeedMul: f32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+PracticeSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PracticeSettings";
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
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
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
#[cfg(feature = "cordl_class_PlayerSaveData+PracticeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSaveData_UserAgeCategorySaveData {
    #[default]
    Adult = 3i32,
    Child = 1i32,
    Teen = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/UserAgeCategorySaveData";
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
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData {
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
#[cfg(feature = "cordl_class_PlayerSaveData+UserAgeCategorySaveData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
    #[default]
    High = 3i32,
    Low = 1i32,
    None = 0i32,
    Standard = 2i32,
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerSpecificSettings/ArcVisibilityTypeSaveData";
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
    #[default]
    AllEffects = 0i32,
    NoEffects = 10i32,
    StrobeFilter = 1i32,
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerSpecificSettings/EnvironmentEffectsFilterPresetSaveData";
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
    #[default]
    Dynamic = 0i32,
    Static = 1i32,
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveData/PlayerSpecificSettings/NoteJumpDurationTypeSettingsSaveData";
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
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
#[cfg(
    feature = "cordl_class_PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData {
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
