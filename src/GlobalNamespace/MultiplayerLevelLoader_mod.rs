#[cfg(feature = "cordl_class_MultiplayerLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    >,
    pub stillDownloadingSongEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub countdownFinishedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelGameplaySetupData>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
        >,
    >,
    pub _loaderState: crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState,
    pub _getBeatmapCancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _getBeatmapLevelResultTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::LoadBeatmapLevelDataResult,
        >,
    >,
    pub _gameplaySetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILevelGameplaySetupData,
    >,
    pub _beatmapLevelData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLevelData,
    >,
    pub _startTime: i64,
    pub _stillDownloadingCalled: bool,
}
#[cfg(feature = "cordl_class_MultiplayerLevelLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLevelLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelLoader";
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
#[cfg(feature = "MultiplayerLevelLoader")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLevelLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl crate::GlobalNamespace::MultiplayerLevelLoader {
    #[cfg(feature = "MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
    pub type MultiplayerBeatmapLoaderState = crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState;
    pub fn ClearLoading(
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
                    >("ClearLoading")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearLoading", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        gameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ILevelGameplaySetupData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
                            >,
                        >,
                        1usize,
                    >("LoadBeatmapLevelDataAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadBeatmapLevelDataAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (gameplaySetupData))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadLevel(
        &mut self,
        gameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
        initialStartTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILevelGameplaySetupData,
                            >,
                            i64,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LoadLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadLevel", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (gameplaySetupData, initialStartTime))?
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
    pub fn SetNewStartTime(
        &mut self,
        newStartTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i64),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetNewStartTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetNewStartTime", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (newStartTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Tick",
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
    pub fn add_countdownFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ILevelGameplaySetupData,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::ILevelGameplaySetupData,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::IBeatmapLevelData,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_countdownFinishedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_countdownFinishedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_stillDownloadingSongEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_stillDownloadingSongEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_stillDownloadingSongEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_countdownFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ILevelGameplaySetupData,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::ILevelGameplaySetupData,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::IBeatmapLevelData,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_countdownFinishedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_countdownFinishedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_stillDownloadingSongEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_stillDownloadingSongEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_stillDownloadingSongEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MultiplayerLevelLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
    #[default]
    LoadingBeatmap = 1i32,
    NotLoading = 0i32,
    WaitingForCountdown = 2i32,
}
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelLoader/MultiplayerBeatmapLoaderState";
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
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
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
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
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
#[cfg(feature = "cordl_class_MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
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
