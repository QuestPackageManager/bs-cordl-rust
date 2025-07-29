#[cfg(
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
    #[default]
    All = 0i32,
    FullHeightOnly = 1i32,
    None = 2i32,
}
#[cfg(
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/GameplayModifiers/EnabledObstacleType";
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
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
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
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
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
    feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
    #[default]
    Bar = 0i32,
    Battery = 1i32,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/GameplayModifiers/EnergyType";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
    #[default]
    Faster = 1i32,
    Normal = 0i32,
    Slower = 2i32,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/GameplayModifiers/SongSpeed";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub localPlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer,
            >,
        >,
    >,
    pub guestPlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer,
            >,
        >,
    >,
    pub lastSelectedBeatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataV1_0_1 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1 {
    pub const kCurrentVersion: &'static str = "";
    #[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
    pub type AchievementsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
    pub type GameplayModifiers = crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
    pub type GuestPlayer = crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer;
    #[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
    pub type LocalPlayer = crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
    pub type PlayerAllOverallStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
    pub type PlayerLevelStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
    pub type PlayerMissionStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
    pub type PlayerOverallStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
    pub type PlayerSpecificSettings = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings;
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerSaveDataV1_0_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+AchievementsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_AchievementsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub unlockedAchievements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub unlockedAchievementsToUpload: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+AchievementsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/AchievementsData";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+AchievementsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+AchievementsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+AchievementsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_GameplayModifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub energyType: crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType,
    pub noFail: bool,
    pub instaFail: bool,
    pub failOnSaberClash: bool,
    pub enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType,
    pub fastNotes: bool,
    pub strictAngles: bool,
    pub disappearingArrows: bool,
    pub noBombs: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/GameplayModifiers";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType")]
    pub type EnabledObstacleType = crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnabledObstacleType;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
    pub type EnergyType = crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_EnergyType;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiers_PlayerSaveDataV1_0_1_SongSpeed;
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GameplayModifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GuestPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_GuestPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GuestPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/GuestPlayer";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GuestPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GuestPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+GuestPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+LocalPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_LocalPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shouldShowTutorialPrompt: bool,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers,
    >,
    pub playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings,
    >,
    pub playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData,
    >,
    pub levelsStatsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData,
            >,
        >,
    >,
    pub missionsStatsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData,
            >,
        >,
    >,
    pub showedMissionHelpIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub achievementsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+LocalPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/LocalPlayer";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+LocalPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+LocalPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+LocalPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub campaignOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
    >,
    pub soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
    >,
    pub partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
    >,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/PlayerAllOverallStatsData";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData1(
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
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
    pub fn _ctor_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData1(
        &mut self,
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
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
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub highScore: i32,
    pub maxCombo: i32,
    pub fullCombo: bool,
    pub maxRank: crate::GlobalNamespace::RankModel_Rank,
    pub validScore: bool,
    pub playCount: i32,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/PlayerLevelStatsData";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cleared: bool,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/PlayerMissionStatsData";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerOverallStatsData {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/PlayerOverallStatsData";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
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
        cleardLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cummulativeCutScoreWithoutMultiplier: i64,
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
                    cleardLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    cummulativeCutScoreWithoutMultiplier,
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
        cleardLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cummulativeCutScoreWithoutMultiplier: i64,
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
                        cleardLevelsCount,
                        failedLevelsCount,
                        fullComboCount,
                        timePlayed,
                        handDistanceTravelled,
                        cummulativeCutScoreWithoutMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub staticLights: bool,
    pub leftHanded: bool,
    pub swapColors: bool,
    pub playerHeight: f32,
    pub disableSFX: bool,
    pub reduceDebris: bool,
    pub advancedHud: bool,
    pub noTextsAndHuds: bool,
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataV1_0_1/PlayerSpecificSettings";
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
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
#[cfg(feature = "cordl_class_PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
