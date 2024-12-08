#[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage_DeleteFileCommand {
    __cordl_parent: SyncBackgroundCommand,
    pub _filePath: *mut crate::System::String,
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
    type Target = SyncBackgroundCommand;
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
    pub fn _ctor(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object)
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
    __cordl_parent: SyncBackgroundCommand_1<bool>,
    pub _filePath: *mut crate::System::String,
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
    type Target = SyncBackgroundCommand_1<bool>;
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
    pub fn _ctor(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteInternal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object)
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
#[cfg(feature = "FileSystemFileStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemFileStorage {
    __cordl_parent: crate::System::Object,
    pub _persistentDataPath: *mut crate::System::String,
    pub _commandQueueMap: *mut crate::System::Collections::Concurrent::ConcurrentDictionary_2<
        *mut crate::System::String,
        *mut BackgroundCommandQueue,
    >,
}
#[cfg(feature = "FileSystemFileStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileSystemFileStorage => ""."FileSystemFileStorage"
);
#[cfg(feature = "FileSystemFileStorage")]
impl std::ops::Deref for FileSystemFileStorage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl std::ops::DerefMut for FileSystemFileStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl FileSystemFileStorage {
    #[cfg(feature = "FileSystemFileStorage+__c")]
    pub type __c = crate::GlobalNamespace::FileSystemFileStorage___c;
    #[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
    pub type LoadFileCommand = crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand;
    #[cfg(feature = "FileSystemFileStorage+DeleteFileCommand")]
    pub type DeleteFileCommand = crate::GlobalNamespace::FileSystemFileStorage_DeleteFileCommand;
    #[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
    pub type SaveFileCommand = crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand;
    #[cfg(feature = "FileSystemFileStorage+FileExistsCommand")]
    pub type FileExistsCommand = crate::GlobalNamespace::FileSystemFileStorage_FileExistsCommand;
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
    pub fn GetFilePath(
        &mut self,
        fileName: *mut crate::System::String,
        storageLocation: StoragePreference,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetFilePath", (fileName, storageLocation))?;
        Ok(__cordl_ret)
    }
    pub fn GetCommandQueue(
        &mut self,
        fileName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BackgroundCommandQueue> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BackgroundCommandQueue = __cordl_object
            .invoke("GetCommandQueue", (fileName))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "FileSystemFileStorage")]
impl quest_hook::libil2cpp::ObjectType for FileSystemFileStorage {
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
    __cordl_parent: SyncBackgroundCommand_1<*mut crate::System::String>,
    pub _filePath: *mut crate::System::String,
}
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_LoadFileCommand => ""
    ."FileSystemFileStorage/LoadFileCommand"
);
#[cfg(feature = "FileSystemFileStorage+LoadFileCommand")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemFileStorage_LoadFileCommand {
    type Target = SyncBackgroundCommand_1<*mut crate::System::String>;
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath))?;
        Ok(__cordl_object)
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
    __cordl_parent: SyncBackgroundCommand,
    pub _filePath: *mut crate::System::String,
    pub _value: *mut crate::System::String,
}
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FileSystemFileStorage_SaveFileCommand => ""
    ."FileSystemFileStorage/SaveFileCommand"
);
#[cfg(feature = "FileSystemFileStorage+SaveFileCommand")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemFileStorage_SaveFileCommand {
    type Target = SyncBackgroundCommand;
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
    pub fn _ctor(
        &mut self,
        filePath: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filePath, value))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filePath: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filePath, value))?;
        Ok(__cordl_object)
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
