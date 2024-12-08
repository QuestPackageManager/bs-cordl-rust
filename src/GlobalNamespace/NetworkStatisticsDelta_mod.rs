#[cfg(feature = "NetworkStatisticsDelta")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NetworkStatisticsDelta {
    pub packetsSentDelta: i64,
    pub packetsReceivedDelta: i64,
    pub bytesSentDelta: i64,
    pub bytesReceivedDelta: i64,
    pub packetsLostDelta: i64,
    pub packetsSentEncryptedDelta: i64,
    pub packetsSentPlaintextDelta: i64,
    pub packetsSentRejectedDelta: i64,
    pub packetsReceivedEncryptedDelta: i64,
    pub packetsReceivedPlaintextDelta: i64,
    pub packetsReceivedRejectedDelta: i64,
    pub encryptionProcessingTimeDelta: i64,
    pub decryptionProcessingTimeDelta: i64,
}
#[cfg(feature = "NetworkStatisticsDelta")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkStatisticsDelta => ""
    ."NetworkStatisticsDelta"
);
#[cfg(feature = "NetworkStatisticsDelta")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "NetworkStatisticsDelta")]
impl crate::GlobalNamespace::NetworkStatisticsDelta {
    pub fn _ctor(
        &mut self,
        packetsSentDelta: i64,
        packetsReceivedDelta: i64,
        bytesSentDelta: i64,
        bytesReceivedDelta: i64,
        packetsLostDelta: i64,
        packetsSentEncryptedDelta: i64,
        packetsSentPlaintextDelta: i64,
        packetsSentRejectedDelta: i64,
        packetsReceivedEncryptedDelta: i64,
        packetsReceivedPlaintextDelta: i64,
        packetsReceivedRejectedDelta: i64,
        encryptionProcessingTimeDelta: i64,
        decryptionProcessingTimeDelta: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                packetsSentDelta,
                packetsReceivedDelta,
                bytesSentDelta,
                bytesReceivedDelta,
                packetsLostDelta,
                packetsSentEncryptedDelta,
                packetsSentPlaintextDelta,
                packetsSentRejectedDelta,
                packetsReceivedEncryptedDelta,
                packetsReceivedPlaintextDelta,
                packetsReceivedRejectedDelta,
                encryptionProcessingTimeDelta,
                decryptionProcessingTimeDelta,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
