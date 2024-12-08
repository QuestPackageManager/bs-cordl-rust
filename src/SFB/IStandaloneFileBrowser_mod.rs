#[cfg(feature = "SFB+IStandaloneFileBrowser")]
#[repr(C)]
#[derive(Debug)]
pub struct IStandaloneFileBrowser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SFB+IStandaloneFileBrowser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::SFB::IStandaloneFileBrowser => "SFB"
    ."IStandaloneFileBrowser"
);
#[cfg(feature = "SFB+IStandaloneFileBrowser")]
impl std::ops::Deref for crate::SFB::IStandaloneFileBrowser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SFB+IStandaloneFileBrowser")]
impl std::ops::DerefMut for crate::SFB::IStandaloneFileBrowser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SFB+IStandaloneFileBrowser")]
impl crate::SFB::IStandaloneFileBrowser {
    pub fn OpenFilePanel(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        extensions: *mut quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        multiselect: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object
            .invoke("OpenFilePanel", (title, directory, extensions, multiselect))?;
        Ok(__cordl_ret)
    }
    pub fn OpenFilePanelAsync(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        extensions: *mut quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        multiselect: bool,
        cb: *mut crate::System::Action_1<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OpenFilePanelAsync",
                (title, directory, extensions, multiselect, cb),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OpenFolderPanel(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        multiselect: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("OpenFolderPanel", (title, directory, multiselect))?;
        Ok(__cordl_ret)
    }
    pub fn OpenFolderPanelAsync(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        multiselect: bool,
        cb: *mut crate::System::Action_1<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenFolderPanelAsync", (title, directory, multiselect, cb))?;
        Ok(__cordl_ret)
    }
    pub fn SaveFilePanel(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        defaultName: *mut crate::System::String,
        extensions: *mut quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SaveFilePanel", (title, directory, defaultName, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn SaveFilePanelAsync(
        &mut self,
        title: *mut crate::System::String,
        directory: *mut crate::System::String,
        defaultName: *mut crate::System::String,
        extensions: *mut quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        cb: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SaveFilePanelAsync",
                (title, directory, defaultName, extensions, cb),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "SFB+IStandaloneFileBrowser")]
impl quest_hook::libil2cpp::ObjectType for crate::SFB::IStandaloneFileBrowser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
