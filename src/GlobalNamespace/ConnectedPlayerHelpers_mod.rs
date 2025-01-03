#[cfg(feature = "ConnectedPlayerHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ConnectedPlayerHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConnectedPlayerHelpers => ""
    ."ConnectedPlayerHelpers"
);
#[cfg(feature = "ConnectedPlayerHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl crate::GlobalNamespace::ConnectedPlayerHelpers {
    pub fn HasFinishedLevel(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasFinishedLevel", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActive(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsActive", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActiveOrFinished(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsActiveOrFinished", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFailed(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFailed", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WantsToPlayNextLevel(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WantsToPlayNextLevel", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WasActiveAtLevelStart(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WasActiveAtLevelStart", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
