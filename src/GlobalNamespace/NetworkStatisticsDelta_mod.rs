#[cfg(feature = "NetworkStatisticsDelta")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetworkStatisticsDelta";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "NetworkStatisticsDelta")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "NetworkStatisticsDelta")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "NetworkStatisticsDelta")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "NetworkStatisticsDelta")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NetworkStatisticsDelta {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
    }
}
