#[cfg(feature = "FileSystemFileStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _persistentDataPath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _commandQueueMap: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BackgroundCommandQueue>,
    >,
}
#[cfg(feature = "FileSystemFileStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileSystemFileStorage => ""
    ."FileSystemFileStorage"
);
#[cfg(feature = "FileSystemFileStorage")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemFileStorage {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileSystemFileStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl crate::GlobalNamespace::FileSystemFileStorage {
    #[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
    pub type DeleteFileCommand = crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand;
    #[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
    pub type FileExistsCommand = crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand;
    #[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
    pub type LoadFileCommand = crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand;
    #[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
    pub type SaveFileCommand = crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand;
    pub fn DeleteFileAsync(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("DeleteFileAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileExistsAsync(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = __cordl_object
            .invoke("FileExistsAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBackupFilePath(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBackupFilePath", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommandQueue(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BackgroundCommandQueue>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BackgroundCommandQueue,
        > = __cordl_object.invoke("GetCommandQueue", (fileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFilePath(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetFilePath", (fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTempFilePath(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTempFilePath", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFileAsync(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("LoadFileAsync", (fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SaveFileAsync(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveFileAsync", (fileName, value, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemFileStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>>
for crate::GlobalNamespace::FileSystemFileStorage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>>
for crate::GlobalNamespace::FileSystemFileStorage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage_DeleteFileCommand {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SyncBackgroundCommand,
    >,
    pub _filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand => ""
    ."FileSystemFileStorage/DeleteFileCommand"
);
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
impl std::ops::Deref
for crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SyncBackgroundCommand,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
impl crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand {
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage_FileExistsCommand {
    __cordl_parent: quest_hook::libil2cpp::Gc<bool>,
    pub _filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_FileExistsCommand => ""
    ."FileSystemFileStorage/FileExistsCommand"
);
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
impl std::ops::Deref
for crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand {
    type Target = quest_hook::libil2cpp::Gc<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
impl crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand {
    pub fn ExecuteInternal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage_LoadFileCommand {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_LoadFileCommand => ""
    ."FileSystemFileStorage/LoadFileCommand"
);
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
impl crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand {
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage_SaveFileCommand {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SyncBackgroundCommand,
    >,
    pub _filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_SaveFileCommand => ""
    ."FileSystemFileStorage/SaveFileCommand"
);
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SyncBackgroundCommand,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
impl crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand {
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath, value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
