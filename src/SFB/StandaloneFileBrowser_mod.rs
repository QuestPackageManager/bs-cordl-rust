#[cfg(feature = "SFB+StandaloneFileBrowser")]
#[repr(C)]
#[derive(Debug)]
pub struct StandaloneFileBrowser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SFB+StandaloneFileBrowser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::SFB::StandaloneFileBrowser => "SFB"
    ."StandaloneFileBrowser"
);
#[cfg(feature = "SFB+StandaloneFileBrowser")]
impl std::ops::Deref for crate::SFB::StandaloneFileBrowser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SFB+StandaloneFileBrowser")]
impl std::ops::DerefMut for crate::SFB::StandaloneFileBrowser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SFB+StandaloneFileBrowser")]
impl crate::SFB::StandaloneFileBrowser {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OpenFilePanelAsync_Il2CppArray1(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extensions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        >,
        multiselect: bool,
        cb: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OpenFilePanelAsync",
                (title, directory, extensions, multiselect, cb),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenFilePanelAsync_Il2CppString0(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        multiselect: bool,
        cb: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OpenFilePanelAsync",
                (title, directory, extension, multiselect, cb),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenFilePanel_Il2CppArray1(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extensions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        >,
        multiselect: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenFilePanel", (title, directory, extensions, multiselect))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenFilePanel_Il2CppString0(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        multiselect: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenFilePanel", (title, directory, extension, multiselect))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenFolderPanel(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        multiselect: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenFolderPanel", (title, directory, multiselect))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenFolderPanelAsync(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        multiselect: bool,
        cb: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenFolderPanelAsync", (title, directory, multiselect, cb))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveFilePanelAsync_Il2CppArray1(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extensions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        >,
        cb: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SaveFilePanelAsync",
                (title, directory, defaultName, extensions, cb),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveFilePanelAsync_Il2CppString0(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cb: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SaveFilePanelAsync",
                (title, directory, defaultName, extension, cb),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveFilePanel_Il2CppArray1(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extensions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::SFB::ExtensionFilter>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveFilePanel", (title, directory, defaultName, extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveFilePanel_Il2CppString0(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveFilePanel", (title, directory, defaultName, extension))?;
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
#[cfg(feature = "SFB+StandaloneFileBrowser")]
impl quest_hook::libil2cpp::ObjectType for crate::SFB::StandaloneFileBrowser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
