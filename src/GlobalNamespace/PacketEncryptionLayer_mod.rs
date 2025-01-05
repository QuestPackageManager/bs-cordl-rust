#[cfg(feature = "PacketEncryptionLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketEncryptionLayer {
    __cordl_parent: crate::LiteNetLib::Layers::PacketLayerBase,
    pub statistics: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics,
    >,
    pub _taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    pub _encryptionStates: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ExpiringDictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    >,
    pub _pendingEncryptionStates: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ExpiringDictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList,
            >,
        >,
    >,
    pub _unencryptedTrafficFilter: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _filterUnencryptedTraffic_k__BackingField: bool,
    pub _enableStatistics_k__BackingField: bool,
}
#[cfg(feature = "PacketEncryptionLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PacketEncryptionLayer => ""
    ."PacketEncryptionLayer"
);
#[cfg(feature = "PacketEncryptionLayer")]
impl std::ops::Deref for crate::GlobalNamespace::PacketEncryptionLayer {
    type Target = crate::LiteNetLib::Layers::PacketLayerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PacketEncryptionLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl crate::GlobalNamespace::PacketEncryptionLayer {
    pub const kEncryptedPacketType: u8 = 1u8;
    pub const kEncryptionStateTimeoutMs: i64 = 300000i64;
    pub const kPendingEncryptionStateTimeoutMs: i64 = 10000i64;
    pub const kPlaintextPacketType: u8 = 0u8;
    #[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
    pub type EncryptionStatistics = crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics;
    #[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
    pub type PendingEncryptionStateList = crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList;
    pub fn AddEncryptedEndpoint(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        > = __cordl_object
            .invoke(
                "AddEncryptedEndpoint",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEncryptedEndpointAsync(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
                >,
            >,
        > = __cordl_object
            .invoke(
                "AddEncryptedEndpointAsync",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPendingEncryptedEndpointAsync(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientRandom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "AddPendingEncryptedEndpointAsync",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesFilter(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesFilter", (data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_object.into())
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInboundPacket(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInboundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInboundPacketInternal(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        encrypted: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ProcessInboundPacketInternal",
                (remoteEndPoint, data, offset, length, encrypted),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOutBoundPacket(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessOutBoundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOutBoundPacketInternal(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        encrypted: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ProcessOutBoundPacketInternal",
                (remoteEndPoint, data, offset, length, encrypted),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PromotePendingEncryptionState(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PromotePendingEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllEndpoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllEndpoints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveEncryptedEndpoint(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        encryptedState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveEncryptedEndpoint", (endPoint, encryptedState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUnencryptedTrafficFilter(
        &mut self,
        unencryptedTrafficFilter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnencryptedTrafficFilter", (unencryptedTrafficFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetEncryptionState(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        state: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPendingEncryptionState(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        state: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPendingEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPotentialPendingEncryptionStates(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        encryptionStates: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetPotentialPendingEncryptionStates",
                (endPoint, encryptionStates),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableStatistics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableStatistics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_filterUnencryptedTraffic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_filterUnencryptedTraffic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableStatistics(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableStatistics", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_filterUnencryptedTraffic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_filterUnencryptedTraffic", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PacketEncryptionLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketEncryptionLayer_EncryptionStatistics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _packetsReceivedPlaintext: i64,
    pub _packetsReceivedEncrypted: i64,
    pub _packetsReceivedRejected: i64,
    pub _packetsSentPlaintext: i64,
    pub _packetsSentEncrypted: i64,
    pub _packetsSentRejected: i64,
    pub _encryptionProcessingTime: i64,
    pub _decryptionProcessingTime: i64,
}
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics => ""
    ."PacketEncryptionLayer/EncryptionStatistics"
);
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
impl std::ops::Deref
for crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
impl crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics {
    pub fn AddDecryptionProcessingTime(
        &mut self,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDecryptionProcessingTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEncryptionProcessingTime(
        &mut self,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEncryptionProcessingTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsReceivedEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedEncrypted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsReceivedPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedPlaintext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsReceivedRejected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedRejected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsSentEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentEncrypted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsSentPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentPlaintext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementPacketsSentRejected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentRejected", ())?;
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
    pub fn get_decryptionProcessingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_decryptionProcessingTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_encryptionProcessingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_encryptionProcessingTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsReceivedEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_packetsReceivedEncrypted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsReceivedPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_packetsReceivedPlaintext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsReceivedRejected(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsReceivedRejected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsSentEncrypted(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentEncrypted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsSentPlaintext(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentPlaintext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packetsSentRejected(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentRejected", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketEncryptionLayer_PendingEncryptionStateList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pendingStatesByPort: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    >,
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList => ""
    ."PacketEncryptionLayer/PendingEncryptionStateList"
);
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl std::ops::Deref
for crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    pub fn Add(
        &mut self,
        port: i32,
        encryptionState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (port, encryptionState))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetSortedEncryptionStates(
        &mut self,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        > = __cordl_object.invoke("GetSortedEncryptionStates", (port))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove_EncryptionUtility_IEncryptionState1(
        &mut self,
        port: i32,
        encryptionState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Remove", (port, encryptionState))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_i32_0(&mut self, port: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (port))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetEncryptionState(
        &mut self,
        port: i32,
        encryptionState: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetEncryptionState", (port, encryptionState))?;
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
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
