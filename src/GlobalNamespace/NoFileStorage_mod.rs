#[cfg(feature = "NoFileStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct NoFileStorage {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "NoFileStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoFileStorage => ""
    ."NoFileStorage"
);
#[cfg(feature = "NoFileStorage")]
impl std::ops::Deref for crate::GlobalNamespace::NoFileStorage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoFileStorage")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoFileStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoFileStorage")]
impl crate::GlobalNamespace::NoFileStorage {
    pub fn DeleteFileAsync(
        &mut self,
        fileName: *mut crate::System::String,
        storageLocation: crate::GlobalNamespace::StoragePreference,
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
        storageLocation: crate::GlobalNamespace::StoragePreference,
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
        storageLocation: crate::GlobalNamespace::StoragePreference,
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SaveFileAsync(
        &mut self,
        fileName: *mut crate::System::String,
        value: *mut crate::System::String,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SaveFileAsync", (fileName, value, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoFileStorage")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoFileStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
