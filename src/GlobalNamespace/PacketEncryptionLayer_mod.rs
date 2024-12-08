#[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketEncryptionLayer_EncryptionStatistics {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsReceivedEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedEncrypted", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsReceivedPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedPlaintext", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsReceivedRejected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsReceivedRejected", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsSentEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentEncrypted", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsSentPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentPlaintext", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementPacketsSentRejected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementPacketsSentRejected", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_decryptionProcessingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_decryptionProcessingTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_encryptionProcessingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_encryptionProcessingTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsReceivedEncrypted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_packetsReceivedEncrypted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsReceivedPlaintext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_packetsReceivedPlaintext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsReceivedRejected(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsReceivedRejected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsSentEncrypted(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentEncrypted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsSentPlaintext(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentPlaintext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packetsSentRejected(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_packetsSentRejected", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "PacketEncryptionLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketEncryptionLayer {
    __cordl_parent: crate::LiteNetLib::Layers::PacketLayerBase,
    pub statistics: *mut crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _encryptionStates: *mut ExpiringDictionary_2<
        *mut crate::System::Net::IPEndPoint,
        *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    >,
    pub _pendingEncryptionStates: *mut ExpiringDictionary_2<
        *mut crate::System::Net::IPAddress,
        *mut crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList,
    >,
    pub _unencryptedTrafficFilter: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _filterUnencryptedTraffic_k__BackingField: bool,
    pub _enableStatistics_k__BackingField: bool,
}
#[cfg(feature = "PacketEncryptionLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PacketEncryptionLayer => ""."PacketEncryptionLayer"
);
#[cfg(feature = "PacketEncryptionLayer")]
impl std::ops::Deref for PacketEncryptionLayer {
    type Target = crate::LiteNetLib::Layers::PacketLayerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl std::ops::DerefMut for PacketEncryptionLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl PacketEncryptionLayer {
    pub const kEncryptedPacketType: u8 = 1u8;
    pub const kEncryptionStateTimeoutMs: i64 = 300000i64;
    pub const kPendingEncryptionStateTimeoutMs: i64 = 10000i64;
    pub const kPlaintextPacketType: u8 = 0u8;
    #[cfg(feature = "PacketEncryptionLayer+EncryptionStatistics")]
    pub type EncryptionStatistics = crate::GlobalNamespace::PacketEncryptionLayer_EncryptionStatistics;
    #[cfg(feature = "PacketEncryptionLayer+_AddEncryptedEndpointAsync_d__25")]
    pub type _AddEncryptedEndpointAsync_d__25 = crate::GlobalNamespace::PacketEncryptionLayer__AddEncryptedEndpointAsync_d__25;
    #[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList")]
    pub type PendingEncryptionStateList = crate::GlobalNamespace::PacketEncryptionLayer_PendingEncryptionStateList;
    #[cfg(feature = "PacketEncryptionLayer+_AddPendingEncryptedEndpointAsync_d__27")]
    pub type _AddPendingEncryptedEndpointAsync_d__27 = crate::GlobalNamespace::PacketEncryptionLayer__AddPendingEncryptedEndpointAsync_d__27;
    pub fn AddEncryptedEndpoint(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        preMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        clientRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState = __cordl_object
            .invoke(
                "AddEncryptedEndpoint",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddEncryptedEndpointAsync(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        preMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        clientRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        > = __cordl_object
            .invoke(
                "AddEncryptedEndpointAsync",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddPendingEncryptedEndpointAsync(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        preMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        clientRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "AddPendingEncryptedEndpointAsync",
                (endPoint, preMasterSecret, serverRandom, clientRandom, isClient),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchesFilter(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesFilter", (data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_object)
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessInboundPacket(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInboundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessInboundPacketInternal(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
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
        Ok(__cordl_ret)
    }
    pub fn ProcessOutBoundPacket(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessOutBoundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessOutBoundPacketInternal(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
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
        Ok(__cordl_ret)
    }
    pub fn PromotePendingEncryptionState(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        state: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PromotePendingEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAllEndpoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllEndpoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveEncryptedEndpoint(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        encryptedState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveEncryptedEndpoint", (endPoint, encryptedState))?;
        Ok(__cordl_ret)
    }
    pub fn SetUnencryptedTrafficFilter(
        &mut self,
        unencryptedTrafficFilter: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnencryptedTrafficFilter", (unencryptedTrafficFilter))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetEncryptionState(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        state: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPendingEncryptionState(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        state: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPendingEncryptionState", (endPoint, state))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPotentialPendingEncryptionStates(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        encryptionStates: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_ret)
    }
    pub fn get_enableStatistics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableStatistics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_filterUnencryptedTraffic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_filterUnencryptedTraffic", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PacketEncryptionLayer")]
impl quest_hook::libil2cpp::ObjectType for PacketEncryptionLayer {
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
    __cordl_parent: crate::System::Object,
    pub _pendingStatesByPort: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
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
    type Target = crate::System::Object;
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
    #[cfg(feature = "PacketEncryptionLayer+PendingEncryptionStateList+__c")]
    pub type __c = crate::GlobalNamespace::PendingEncryptionStateList___c;
    #[cfg(
        feature = "PacketEncryptionLayer+PendingEncryptionStateList+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::GlobalNamespace::PendingEncryptionStateList___c__DisplayClass8_0;
    #[cfg(
        feature = "PacketEncryptionLayer+PendingEncryptionStateList+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::PendingEncryptionStateList___c__DisplayClass4_0;
    pub fn Add(
        &mut self,
        port: i32,
        encryptionState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (port, encryptionState))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSortedEncryptionStates(
        &mut self,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        > = __cordl_object.invoke("GetSortedEncryptionStates", (port))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove_EncryptionUtility_IEncryptionState1(
        &mut self,
        port: i32,
        encryptionState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Remove", (port, encryptionState))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_i32_0(&mut self, port: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (port))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetEncryptionState(
        &mut self,
        port: i32,
        encryptionState: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetEncryptionState", (port, encryptionState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret)
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