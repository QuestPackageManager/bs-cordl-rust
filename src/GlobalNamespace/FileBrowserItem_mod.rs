#[cfg(feature = "FileBrowserItem")]
#[repr(C)]
#[derive(Debug)]
pub struct FileBrowserItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _displayName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _fullPath_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _isDirectory_k__BackingField: bool,
}
#[cfg(feature = "FileBrowserItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileBrowserItem => ""
    ."FileBrowserItem"
);
#[cfg(feature = "FileBrowserItem")]
impl std::ops::Deref for crate::GlobalNamespace::FileBrowserItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileBrowserItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserItem")]
impl crate::GlobalNamespace::FileBrowserItem {
    pub fn New(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, fullPath, isDirectory))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, fullPath, isDirectory))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_displayName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_fullPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDirectory(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirectory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_displayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fullPath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fullPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isDirectory(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDirectory", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileBrowserItem")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileBrowserItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
