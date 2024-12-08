#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettingsHandler {
    __cordl_parent: crate::BGLib::SaveDataCore::SaveDataHandler_1<
        *mut crate::BeatSaber::GameSettings::MainSettings,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::MainSettingsHandler =>
    "BeatSaber.GameSettings"."MainSettingsHandler"
);
#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::MainSettingsHandler {
    type Target = crate::BGLib::SaveDataCore::SaveDataHandler_1<
        *mut crate::BeatSaber::GameSettings::MainSettings,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::MainSettingsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
impl crate::BeatSaber::GameSettings::MainSettingsHandler {
    #[cfg(
        feature = "BeatSaber+GameSettings+MainSettingsHandler+_InternalLoadAsync_d__11"
    )]
    pub type _InternalLoadAsync_d__11 = crate::BeatSaber::GameSettings::MainSettingsHandler__InternalLoadAsync_d__11;
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
    pub fn New_IFileStorage0(
        fileStorage: *mut IFileStorage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage))?;
        Ok(__cordl_object)
    }
    pub fn New_MainSettings1(
        fileStorage: *mut IFileStorage,
        instance: *mut crate::BeatSaber::GameSettings::MainSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage, instance))?;
        Ok(__cordl_object)
    }
    pub fn PerformPostLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::SaveDataResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::SaveDataResult = __cordl_object
            .invoke("PerformPostLoad", ())?;
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
    pub fn _ctor_MainSettings1(
        &mut self,
        fileStorage: *mut IFileStorage,
        instance: *mut crate::BeatSaber::GameSettings::MainSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage, instance))?;
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
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettingsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::MainSettingsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
