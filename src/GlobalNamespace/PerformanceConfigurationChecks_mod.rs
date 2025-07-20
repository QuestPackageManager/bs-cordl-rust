#[cfg(feature = "PerformanceConfigurationChecks")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationChecks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub appConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig,
    pub xrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig,
    pub ovrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig,
    pub oculusXrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig,
    pub settingsConfig: crate::BeatSaber::Settings::Settings,
    pub playerConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig,
    pub levelConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig,
    pub invalid: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch,
        >,
    >,
}
#[cfg(feature = "PerformanceConfigurationChecks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks";
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
#[cfg(feature = "PerformanceConfigurationChecks")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceConfigurationChecks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceConfigurationChecks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks {
    #[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
    pub type AppConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
    pub type LevelConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
    pub type Mismatch = crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch;
    #[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
    pub type OVRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
    pub type OculusXRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
    pub type PlayerConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
    pub type XRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig;
    pub fn CreateErrorLog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("CreateErrorLog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), "CreateErrorLog", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), "IsValid", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetExpected(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    crate::GlobalNamespace::GameplayModifierMask,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RecPlayBehaviour_State,
                    >,
                ),
                bool,
                4usize,
            >("SetExpected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), "SetExpected", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (settings, playerSettings, modifiers, recPlayState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyEntry<T>(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expected: T,
        actual: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, T, T),
                quest_hook::libil2cpp::Void,
                3usize,
            >("VerifyEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), "VerifyEntry", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, expected, actual))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyExpected(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    crate::GlobalNamespace::GameplayModifierMask,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RecPlayBehaviour_State,
                    >,
                ),
                bool,
                4usize,
            >("VerifyExpected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), "VerifyExpected", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (settings, playerSettings, modifiers, recPlayState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PerformanceConfigurationChecks as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PerformanceConfigurationChecks as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceConfigurationChecks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_AppConfig {
    pub targetFrameRate: i32,
    pub systemLanguage: crate::UnityEngine::SystemLanguage,
    pub runInBackground: bool,
    pub backgroundLoadingPriority: crate::UnityEngine::ThreadPriority,
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/AppConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_LevelConfig {
    pub modifiers: crate::GlobalNamespace::GameplayModifierMask,
}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/LevelConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_Mismatch {
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub frames: i32,
}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/Mismatch";
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
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
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
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
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
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
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
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_OVRConfig {
    pub spaceWarpEnabled: bool,
    pub chromatic: bool,
    pub monoscopic: bool,
    pub colorGamut: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub nativeColorGamut: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub compositionMethod: crate::GlobalNamespace::OVRManager_CompositionMethod,
    pub enableMixedReality: bool,
    pub trackingOriginType: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    pub usePositionTracking: bool,
    pub useRotationTracking: bool,
    pub sharpenType: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    pub enableDynamicResolution: bool,
    pub minDynamicResolutionScale: f32,
    pub maxDynamicResolutionScale: f32,
    pub simultaneousHandsAndControllersSupport: bool,
    pub suggestedCpuPerfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    pub suggestedGpuPerfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    pub systemDisplayFrequency: f32,
    pub eyeTrackedFoveatedRenderingEnabled: bool,
    pub foveatedRenderingLevel: crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
    pub useDynamicFoveatedRendering: bool,
    pub gpuUtilSupported: bool,
    pub gpuUtilLevel: f32,
    pub eyeFovPremultipliedAlphaModeEnabled: bool,
    pub asymmetricFovEnabled: bool,
    pub eyeTextureArrayEnabled: bool,
    pub localDimmingSupported: bool,
    pub localDimming: bool,
}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/OVRConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_OculusXRConfig {
    pub boundaryDimension: crate::UnityEngine::Vector3,
}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/OculusXRConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_PlayerConfig {
    pub leftHanded: bool,
    pub automaticPlayerHeight: bool,
    pub playerHeight: f32,
    pub noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    pub noteJumpFixedDuration: f32,
    pub noteJumpStartBeatOffset: f32,
    pub autoRestart: bool,
    pub headsetHapticIntensity: f32,
    pub arcsHapticFeedback: bool,
    pub reduceDebris: bool,
    pub noFailEffects: bool,
    pub hideNoteSpawnEffect: bool,
    pub arcVisibility: crate::GlobalNamespace::ArcVisibilityType,
    pub saberTrailIntensity: f32,
    pub noTextsAndHuds: bool,
    pub advancedHud: bool,
    pub sfxVolume: f32,
    pub adaptiveSfx: bool,
    pub environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/PlayerConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceConfigurationChecks_XRConfig {
    pub enabled: bool,
    pub isDeviceActive: bool,
    pub loadedDeviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub stereoRenderingMode: crate::UnityEngine::XR::XRSettings_StereoRenderingMode,
    pub eyeTextureWidth: i32,
    pub eyeTextureHeight: i32,
    pub eyeTextureResolutionScale: f32,
    pub deviceEyeTextureDimension: crate::UnityEngine::Rendering::TextureDimension,
    pub renderViewportScale: f32,
    pub occlusionMaskScale: f32,
    pub useOcclusionMesh: bool,
}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceConfigurationChecks/XRConfig";
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
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
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
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {}
