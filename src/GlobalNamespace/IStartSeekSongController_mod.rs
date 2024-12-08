#[cfg(feature = "IStartSeekSongController")]
#[repr(C)]
#[derive(Debug)]
pub struct IStartSeekSongController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IStartSeekSongController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IStartSeekSongController => ""
    ."IStartSeekSongController"
);
#[cfg(feature = "IStartSeekSongController")]
impl std::ops::Deref for crate::GlobalNamespace::IStartSeekSongController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IStartSeekSongController")]
impl std::ops::DerefMut for crate::GlobalNamespace::IStartSeekSongController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IStartSeekSongController")]
impl crate::GlobalNamespace::IStartSeekSongController {
    pub fn SeekTo(
        &mut self,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SeekTo", (songTime))?;
        Ok(__cordl_ret)
    }
    pub fn StartSong(
        &mut self,
        offsetTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", (offsetTime))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_waitUntilIsReadyToStartTheSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::WaitUntil> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::WaitUntil = __cordl_object
            .invoke("get_waitUntilIsReadyToStartTheSong", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IStartSeekSongController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IStartSeekSongController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
