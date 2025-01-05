#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerObservable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerObservable => ""
    ."MultiplayerConnectedPlayerObservable"
);
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    pub fn New(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl AsRef<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl AsMut<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
