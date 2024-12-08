#[cfg(feature = "IStartSeekSongControllerProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IStartSeekSongControllerProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IStartSeekSongControllerProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IStartSeekSongControllerProvider => ""
    ."IStartSeekSongControllerProvider"
);
#[cfg(feature = "IStartSeekSongControllerProvider")]
impl std::ops::Deref for IStartSeekSongControllerProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IStartSeekSongControllerProvider")]
impl std::ops::DerefMut for IStartSeekSongControllerProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IStartSeekSongControllerProvider")]
impl IStartSeekSongControllerProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_songController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IStartSeekSongController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IStartSeekSongController = __cordl_object
            .invoke("get_songController", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IStartSeekSongControllerProvider")]
impl quest_hook::libil2cpp::ObjectType for IStartSeekSongControllerProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
