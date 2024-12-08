#[cfg(feature = "TrailerGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TrailerGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _disableMainCamera: bool,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _gameSongController: *mut crate::GlobalNamespace::GameSongController,
    pub _mainCamera: *mut crate::GlobalNamespace::MainCamera,
}
#[cfg(feature = "TrailerGameplayManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TrailerGameplayManager => ""
    ."TrailerGameplayManager"
);
#[cfg(feature = "TrailerGameplayManager")]
impl std::ops::Deref for crate::GlobalNamespace::TrailerGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrailerGameplayManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::TrailerGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrailerGameplayManager")]
impl crate::GlobalNamespace::TrailerGameplayManager {
    #[cfg(feature = "TrailerGameplayManager+_Start_d__4")]
    pub type _Start_d__4 = crate::GlobalNamespace::TrailerGameplayManager__Start_d__4;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "TrailerGameplayManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TrailerGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
