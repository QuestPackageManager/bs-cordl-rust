#[cfg(feature = "IBeatmapLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatmapLevelLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBeatmapLevelLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IBeatmapLevelLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBeatmapLevelLoader";
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
#[cfg(feature = "IBeatmapLevelLoader")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatmapLevelLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatmapLevelLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl crate::GlobalNamespace::IBeatmapLevelLoader {
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
                3usize,
            >("CheckBeatmapLevelDataExistsAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckBeatmapLevelDataExistsAsync", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearCache", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
                    >,
                >,
                3usize,
            >("LoadBeatmapLevelDataAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadBeatmapLevelDataAsync", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDownloadingUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_levelDownloadingUpdateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_levelDownloadingUpdateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn remove_levelDownloadingUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_levelDownloadingUpdateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_levelDownloadingUpdateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBeatmapLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::IBeatmapLevelLoader {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::IBeatmapLevelLoader {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
