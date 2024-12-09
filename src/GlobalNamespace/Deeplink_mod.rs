#[cfg(feature = "Deeplink")]
#[repr(C)]
#[derive(Debug)]
pub struct Deeplink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Destination: *mut quest_hook::libil2cpp::Il2CppString,
    pub LevelID: *mut quest_hook::libil2cpp::Il2CppString,
    pub PackID: *mut quest_hook::libil2cpp::Il2CppString,
    pub Difficulty: *mut quest_hook::libil2cpp::Il2CppString,
    pub Characteristic: *mut quest_hook::libil2cpp::Il2CppString,
    pub MultiplayerLobbyCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub MultiplayerSecret: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "Deeplink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Deeplink => ""."Deeplink"
);
#[cfg(feature = "Deeplink")]
impl std::ops::Deref for crate::GlobalNamespace::Deeplink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Deeplink")]
impl std::ops::DerefMut for crate::GlobalNamespace::Deeplink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Deeplink")]
impl crate::GlobalNamespace::Deeplink {
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
}
#[cfg(feature = "Deeplink")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Deeplink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
