#[cfg(feature = "ILobbyPlayerData")]
#[repr(C)]
#[derive(Debug)]
pub struct ILobbyPlayerData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ILobbyPlayerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ILobbyPlayerData => ""
    ."ILobbyPlayerData"
);
#[cfg(feature = "ILobbyPlayerData")]
impl std::ops::Deref for crate::GlobalNamespace::ILobbyPlayerData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyPlayerData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ILobbyPlayerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyPlayerData")]
impl crate::GlobalNamespace::ILobbyPlayerData {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInLobby(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInLobby", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartyOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPartyOwner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isActive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isActive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isInLobby(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isInLobby", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isPartyOwner(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isPartyOwner", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isReady(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isReady", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ILobbyPlayerData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ILobbyPlayerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ILobbyPlayerData")]
impl AsRef<crate::GlobalNamespace::ILevelGameplaySetupData>
for crate::GlobalNamespace::ILobbyPlayerData {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelGameplaySetupData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayerData")]
impl AsMut<crate::GlobalNamespace::ILevelGameplaySetupData>
for crate::GlobalNamespace::ILobbyPlayerData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelGameplaySetupData {
        unsafe { std::mem::transmute(self) }
    }
}
