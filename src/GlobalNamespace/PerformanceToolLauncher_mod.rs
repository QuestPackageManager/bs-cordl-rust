#[cfg(feature = "cordl_class_PerformanceToolLauncher")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceToolLauncher {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assets: crate::GlobalNamespace::PerformanceToolLauncher_Assets,
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceToolLauncher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceToolLauncher";
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
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceToolLauncher {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceToolLauncher {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl crate::GlobalNamespace::PerformanceToolLauncher {
    #[cfg(feature = "PerformanceToolLauncher+Assets")]
    pub type Assets = crate::GlobalNamespace::PerformanceToolLauncher_Assets;
    #[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
    pub type OverrideConfig = crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig;
    pub fn Initialize(
        &mut self,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        mainCamera: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainCamera>,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
        songController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongController,
        >,
        gamePause: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGamePause>,
        sceneSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayCoreSceneSetupData,
        >,
        overrideConfig: crate::System::Nullable_1<
            crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SettingsManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerDataModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MainCamera,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RecPlayBehaviour_State,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SongController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IGamePause,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayCoreSceneSetupData,
                            >,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        settingsManager,
                        playerDataModel,
                        mainCamera,
                        recPlayState,
                        songController,
                        gamePause,
                        sceneSetupData,
                        overrideConfig,
                    ),
                )?
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
    pub fn Run(
        recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerformanceRecorder>,
        songController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PerformanceRecorder,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SongController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Run")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Run",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (recorder, songController))?
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceToolLauncher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceToolLauncher_Assets {
    pub visualizer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PerformanceVisualizer,
    >,
    pub recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerformanceRecorder>,
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceToolLauncher/Assets";
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceToolLauncher+Assets")]
impl crate::GlobalNamespace::PerformanceToolLauncher_Assets {}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PerformanceToolLauncher_OverrideConfig {
    pub enableAutoplay: bool,
    pub enableRecording: bool,
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerformanceToolLauncher/OverrideConfig";
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
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
#[cfg(feature = "cordl_class_PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
impl crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {}
