#[cfg(feature = "NodePoseSyncStateNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NodePoseSyncStateNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _state: crate::GlobalNamespace::NodePoseSyncState,
    pub _id_k__BackingField: crate::GlobalNamespace::SyncStateId,
    pub _time_k__BackingField: i64,
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NodePoseSyncStateNetSerializable => ""
    ."NodePoseSyncStateNetSerializable"
);
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
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
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = __cordl_object
            .invoke("get_state", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_id", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_state(
        &mut self,
        value: crate::GlobalNamespace::NodePoseSyncState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_state", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_time", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsRef<
    crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsMut<
    crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncStateNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
