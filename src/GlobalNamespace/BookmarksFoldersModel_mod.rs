#[cfg(feature = "BookmarksFoldersModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BookmarksFoldersModel {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub myFolders: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _bookmarksFolders: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::FileBrowserItem,
    >,
}
#[cfg(feature = "BookmarksFoldersModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BookmarksFoldersModel => ""
    ."BookmarksFoldersModel"
);
#[cfg(feature = "BookmarksFoldersModel")]
impl std::ops::Deref for crate::GlobalNamespace::BookmarksFoldersModel {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BookmarksFoldersModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BookmarksFoldersModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BookmarksFoldersModel")]
impl crate::GlobalNamespace::BookmarksFoldersModel {
    #[cfg(feature = "BookmarksFoldersModel+__c")]
    pub type __c = crate::GlobalNamespace::BookmarksFoldersModel___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_bookmarksFolders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::FileBrowserItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::FileBrowserItem,
        > = __cordl_object.invoke("get_bookmarksFolders", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BookmarksFoldersModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BookmarksFoldersModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
