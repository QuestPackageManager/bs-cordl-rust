#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IFilePathSongPreviewAudioClipProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IFilePathSongPreviewAudioClipProvider => ""
    ."IFilePathSongPreviewAudioClipProvider"
);
#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
impl crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_songPreviewAudioClipPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songPreviewAudioClipPath", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IFilePathSongPreviewAudioClipProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
