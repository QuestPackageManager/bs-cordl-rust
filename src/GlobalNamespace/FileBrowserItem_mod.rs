#[cfg(feature = "FileBrowserItem")]
#[repr(C)]
#[derive(Debug)]
pub struct FileBrowserItem {
    __cordl_parent: crate::System::Object,
    pub _displayName_k__BackingField: *mut crate::System::String,
    pub _fullPath_k__BackingField: *mut crate::System::String,
    pub _isDirectory_k__BackingField: bool,
}
#[cfg(feature = "FileBrowserItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileBrowserItem => ""."FileBrowserItem"
);
#[cfg(feature = "FileBrowserItem")]
impl std::ops::Deref for FileBrowserItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserItem")]
impl std::ops::DerefMut for FileBrowserItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserItem")]
impl FileBrowserItem {
    pub fn set_isDirectory(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDirectory", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_fullPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_fullPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        displayName: *mut crate::System::String,
        fullPath: *mut crate::System::String,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, fullPath, isDirectory))?;
        Ok(__cordl_ret)
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_displayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_displayName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fullPath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fullPath", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isDirectory(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirectory", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        displayName: *mut crate::System::String,
        fullPath: *mut crate::System::String,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, fullPath, isDirectory))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "FileBrowserItem")]
impl quest_hook::libil2cpp::ObjectType for FileBrowserItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
