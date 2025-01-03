#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NodePoseSyncStateDeltaNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _delta: crate::GlobalNamespace::NodePoseSyncState,
    pub _baseId_k__BackingField: crate::GlobalNamespace::SyncStateId,
    pub _timeOffsetMs_k__BackingField: i32,
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable => ""
    ."NodePoseSyncStateDeltaNetSerializable"
);
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
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
    pub fn get_baseId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = __cordl_object
            .invoke("get_baseId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = __cordl_object
            .invoke("get_delta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                *mut crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                *mut crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeOffsetMs(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_timeOffsetMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_baseId(
        &mut self,
        value: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_baseId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_delta(
        &mut self,
        value: crate::GlobalNamespace::NodePoseSyncState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_delta", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_timeOffsetMs(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeOffsetMs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsRef<
    crate::GlobalNamespace::ISyncStateDeltaSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISyncStateDeltaSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsMut<
    crate::GlobalNamespace::ISyncStateDeltaSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISyncStateDeltaSerializable_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateDeltaNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
