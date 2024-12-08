#[cfg(feature = "IFilePathSongAudioClipProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IFilePathSongAudioClipProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IFilePathSongAudioClipProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IFilePathSongAudioClipProvider => ""
    ."IFilePathSongAudioClipProvider"
);
#[cfg(feature = "IFilePathSongAudioClipProvider")]
impl std::ops::Deref for IFilePathSongAudioClipProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IFilePathSongAudioClipProvider")]
impl std::ops::DerefMut for IFilePathSongAudioClipProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IFilePathSongAudioClipProvider")]
impl IFilePathSongAudioClipProvider {
    pub fn get_songAudioClipPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songAudioClipPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IFilePathSongAudioClipProvider")]
impl quest_hook::libil2cpp::ObjectType for IFilePathSongAudioClipProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
