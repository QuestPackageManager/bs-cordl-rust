#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerLobbyPoseGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub mockNodePoseSyncStateSender: *mut crate::GlobalNamespace::MockNodePoseSyncStateSender,
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerLobbyPoseGenerator =>
    ""."MockPlayerLobbyPoseGenerator"
);
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGenerator")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerLobbyPoseGenerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
