#[cfg(feature = "GameServerListTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerListTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _serverName: *mut crate::HMUI::CurvedTextMeshPro,
    pub _difficultiesText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _musicPackText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _playerCount: *mut crate::HMUI::CurvedTextMeshPro,
    pub _passwordProtected: *mut crate::UnityEngine::GameObject,
    pub _songPackMasksModel: *mut crate::GlobalNamespace::SongPackMasksModel,
}
#[cfg(feature = "GameServerListTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerListTableCell => ""
    ."GameServerListTableCell"
);
#[cfg(feature = "GameServerListTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerListTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerListTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListTableCell")]
impl crate::GlobalNamespace::GameServerListTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (player))?;
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
#[cfg(feature = "GameServerListTableCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerListTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
