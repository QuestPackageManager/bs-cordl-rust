#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicSettingsHandler {
    __cordl_parent: crate::BGLib::SaveDataCore::SaveDataHandler_1<
        *mut crate::BeatSaber::GameSettings::GraphicSettings,
    >,
    pub _currentPreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
    pub _currentPresetKey: *mut crate::System::String,
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::GraphicSettingsHandler
    => "BeatSaber.GameSettings"."GraphicSettingsHandler"
);
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::GraphicSettingsHandler {
    type Target = crate::BGLib::SaveDataCore::SaveDataHandler_1<
        *mut crate::BeatSaber::GameSettings::GraphicSettings,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::GraphicSettingsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
impl crate::BeatSaber::GameSettings::GraphicSettingsHandler {
    #[cfg(
        feature = "BeatSaber+GameSettings+GraphicSettingsHandler+_PerformPostLoadAsync_d__15"
    )]
    pub type _PerformPostLoadAsync_d__15 = crate::BeatSaber::GameSettings::GraphicSettingsHandler__PerformPostLoadAsync_d__15;
    #[cfg(
        feature = "BeatSaber+GameSettings+GraphicSettingsHandler+_TryUpdateCurrentPerformancePresetAsync_d__14"
    )]
    pub type _TryUpdateCurrentPerformancePresetAsync_d__14 = crate::BeatSaber::GameSettings::GraphicSettingsHandler__TryUpdateCurrentPerformancePresetAsync_d__14;
    #[cfg(
        feature = "BeatSaber+GameSettings+GraphicSettingsHandler+_InternalLoadAsync_d__16"
    )]
    pub type _InternalLoadAsync_d__16 = crate::BeatSaber::GameSettings::GraphicSettingsHandler__InternalLoadAsync_d__16;
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_firstVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_firstVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fileNameWithExtension(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_fileNameWithExtension", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredStorageLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<StoragePreference> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: StoragePreference = __cordl_object
            .invoke("get_preferredStorageLocation", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetCurrentPerformancePreset_ByRefMut0(
        &mut self,
        currentPreset: quest_hook::libil2cpp::ByRefMut<
            *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetCurrentPerformancePreset", (currentPreset))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetCurrentPerformancePreset_String_ByRefMut1(
        &mut self,
        serializedEnvironmentName: *mut crate::System::String,
        currentPreset: quest_hook::libil2cpp::ByRefMut<
            *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetCurrentPerformancePreset",
                (serializedEnvironmentName, currentPreset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::SaveDataResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::SaveDataResult = __cordl_object
            .invoke("Load", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IFileStorage0(
        &mut self,
        fileStorage: *mut IFileStorage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_GraphicSettings1(
        &mut self,
        fileStorage: *mut IFileStorage,
        instance: *mut crate::BeatSaber::GameSettings::GraphicSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage, instance))?;
        Ok(__cordl_ret)
    }
    pub fn InternalLoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("InternalLoadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn PerformPostLoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("PerformPostLoadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryUpdateCurrentPerformancePresetAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("TryUpdateCurrentPerformancePresetAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn __n__0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("<>n__0", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IFileStorage0(
        fileStorage: *mut IFileStorage,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage))?;
        Ok(__cordl_object)
    }
    pub fn New_GraphicSettings1(
        fileStorage: *mut IFileStorage,
        instance: *mut crate::BeatSaber::GameSettings::GraphicSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage, instance))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettingsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::GraphicSettingsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
