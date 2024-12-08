#[cfg(feature = "IFileStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct IFileStorage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IFileStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IFileStorage => ""."IFileStorage"
);
#[cfg(feature = "IFileStorage")]
impl std::ops::Deref for IFileStorage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IFileStorage")]
impl std::ops::DerefMut for IFileStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IFileStorage")]
impl IFileStorage {
    pub fn DeleteFileAsync(
        &mut self,
        fileName: *mut crate::System::String,
        storageLocation: StoragePreference,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DeleteFileAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn FileExistsAsync(
        &mut self,
        fileName: *mut crate::System::String,
        storageLocation: StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("FileExistsAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn LoadFileAsync(
        &mut self,
        fileName: *mut crate::System::String,
        storageLocation: StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("LoadFileAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn SaveFileAsync(
        &mut self,
        fileName: *mut crate::System::String,
        value: *mut crate::System::String,
        storageLocation: StoragePreference,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SaveFileAsync", (fileName, value, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IFileStorage")]
impl quest_hook::libil2cpp::ObjectType for IFileStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}