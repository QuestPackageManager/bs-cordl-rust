#[cfg(feature = "MockBeatmapDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapDataAssetFileModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelDataAssetDownloadUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::LevelDataAssetDownloadUpdate>,
    >,
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockBeatmapDataAssetFileModel";
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
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    pub const kAssetsDir: &'static str = "BeatmapDataAssets";
    pub fn GetAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
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
                        crate::GlobalNamespace::GetAssetBundleFileResult,
                    >,
                >,
                3usize,
            >("GetAssetBundleFileForBeatmapLevelAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAssetBundleFileForBeatmapLevelAsync", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryDeleteAssetBundleFileForBeatmapLevelAsync(
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
            >("TryDeleteAssetBundleFileForBeatmapLevelAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryDeleteAssetBundleFileForBeatmapLevelAsync", 3usize
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::LevelDataAssetDownloadUpdate>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_levelDataAssetDownloadUpdateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_levelDataAssetDownloadUpdateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::LevelDataAssetDownloadUpdate>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_levelDataAssetDownloadUpdateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_levelDataAssetDownloadUpdateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl AsRef<crate::GlobalNamespace::IBeatmapDataAssetFileModel>
for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatmapDataAssetFileModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapDataAssetFileModel")]
impl AsMut<crate::GlobalNamespace::IBeatmapDataAssetFileModel>
for crate::GlobalNamespace::MockBeatmapDataAssetFileModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatmapDataAssetFileModel {
        unsafe { std::mem::transmute(self) }
    }
}
