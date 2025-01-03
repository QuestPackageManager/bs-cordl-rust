#[cfg(feature = "MultiplayerMenuClient")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerMenuClient {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _menuPlayerController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuPlayerController,
    >,
    pub _nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INodePoseSyncStateManager,
    >,
}
#[cfg(feature = "MultiplayerMenuClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerMenuClient => ""
    ."MultiplayerMenuClient"
);
#[cfg(feature = "MultiplayerMenuClient")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerMenuClient {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMenuClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerMenuClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMenuClient")]
impl crate::GlobalNamespace::MultiplayerMenuClient {
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "MultiplayerMenuClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerMenuClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
