#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct HeartbeatMessage {
    __cordl_parent: crate::System::Object,
    pub mType: u8,
    pub mPayload: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPaddingLength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage
    => "Org.BouncyCastle.Crypto.Tls"."HeartbeatMessage"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
impl crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
    pub type PayloadBuffer = crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer;
    pub fn Encode(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (context, output))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: u8,
        payload: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        paddingLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, payload, paddingLength))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: u8,
        payload: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        paddingLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, payload, paddingLength))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct HeartbeatMessage_PayloadBuffer {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer =>
    "Org.BouncyCastle.Crypto.Tls"."HeartbeatMessage/PayloadBuffer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
impl crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToTruncatedByteArray(
        &mut self,
        payloadLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ToTruncatedByteArray", (payloadLength))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HeartbeatMessage+PayloadBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::HeartbeatMessage_PayloadBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
