#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SaveDataHandler_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _state_k__BackingField: crate::BGLib::SaveDataCore::LoaderState,
    pub _fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    pub _loadFileTask: *mut crate::System::Threading::Tasks::Task_1<
        crate::BGLib::SaveDataCore::SaveDataResult,
    >,
    pub _saveFileTask: *mut crate::System::Threading::Tasks::Task,
    pub _instance: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::SaveDataHandler_1 < T > =>
    "BGLib.SaveDataCore"."SaveDataHandler`1" < T >
);
#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BGLib::SaveDataCore::SaveDataHandler_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BGLib::SaveDataCore::SaveDataHandler_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
impl<T: quest_hook::libil2cpp::Type> crate::BGLib::SaveDataCore::SaveDataHandler_1<T> {
    #[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1+_InternalLoadAsync_d__26")]
    pub type _InternalLoadAsync_d__26 = crate::BGLib::SaveDataCore::SaveDataHandler_1__InternalLoadAsync_d__26<
        T,
    >;
    #[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1+_LoadAsync_d__22")]
    pub type _LoadAsync_d__22 = crate::BGLib::SaveDataCore::SaveDataHandler_1__LoadAsync_d__22<
        T,
    >;
    #[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1+_ResetChangesAsync_d__25")]
    pub type _ResetChangesAsync_d__25 = crate::BGLib::SaveDataCore::SaveDataHandler_1__ResetChangesAsync_d__25<
        T,
    >;
    #[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1+_SaveAsync_d__24")]
    pub type _SaveAsync_d__24 = crate::BGLib::SaveDataCore::SaveDataHandler_1__SaveAsync_d__24<
        T,
    >;
    pub fn BGLib_SaveDataCore_ISaveDataHandler_get_instance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BGLib::SaveDataCore::VersionableSaveData,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BGLib::SaveDataCore::VersionableSaveData = __cordl_object
            .invoke("BGLib.SaveDataCore.ISaveDataHandler.get_instance", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeleteAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DeleteAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::LoaderState>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::LoaderState = __cordl_object
            .invoke("GetState", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalLoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("InternalLoadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::SaveDataResult>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::SaveDataResult = __cordl_object
            .invoke("Load", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("LoadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IFileStorage0(
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage))?;
        Ok(__cordl_object)
    }
    pub fn New_T1(
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
        instance: T,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage, instance))?;
        Ok(__cordl_object)
    }
    pub fn ResetChangesAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("ResetChangesAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn Save(
        &mut self,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::SaveDataResult>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::SaveDataResult = __cordl_object
            .invoke("Save", (force))?;
        Ok(__cordl_ret)
    }
    pub fn SaveAsync(
        &mut self,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("SaveAsync", (force))?;
        Ok(__cordl_ret)
    }
    pub fn TestFullUpdateLoop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TestFullUpdateLoop", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVersionLoop(
        &mut self,
        deserializedJson: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::SaveDataResult>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::SaveDataResult = __cordl_object
            .invoke("UpdateVersionLoop", (deserializedJson))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IFileStorage0(
        &mut self,
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_T1(
        &mut self,
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
        instance: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage, instance))?;
        Ok(__cordl_ret)
    }
    pub fn get_fileNameWithExtension(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_fileNameWithExtension", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_firstVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_firstVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_instance(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_instance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredStorageLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StoragePreference>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::StoragePreference = __cordl_object
            .invoke("get_preferredStorageLocation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::LoaderState>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::LoaderState = __cordl_object
            .invoke("get_state", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_state(
        &mut self,
        value: crate::BGLib::SaveDataCore::LoaderState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_state", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataHandler_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BGLib::SaveDataCore::SaveDataHandler_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
