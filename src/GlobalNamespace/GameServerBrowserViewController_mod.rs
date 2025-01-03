#[cfg(feature = "GameServerBrowserViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerBrowserViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _filterServersButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _filterText: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServersFilterText,
    >,
    pub _createServerButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _gameServersListTableView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServersListTableView,
    >,
    pub _mainLoadingControl: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LoadingControl,
    >,
    pub _smallLoadingControl: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LoadingControl,
    >,
}
#[cfg(feature = "GameServerBrowserViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerBrowserViewController
    => ""."GameServerBrowserViewController"
);
#[cfg(feature = "GameServerBrowserViewController")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerBrowserViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerBrowserViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerBrowserViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerBrowserViewController")]
impl crate::GlobalNamespace::GameServerBrowserViewController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "GameServerBrowserViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerBrowserViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
