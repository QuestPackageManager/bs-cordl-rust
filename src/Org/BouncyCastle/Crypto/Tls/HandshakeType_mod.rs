#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
#[repr(C)]
#[derive(Debug)]
pub struct HandshakeType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::HandshakeType =>
    "Org.BouncyCastle.Crypto.Tls"."HandshakeType"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::HandshakeType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::HandshakeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
impl crate::Org::BouncyCastle::Crypto::Tls::HandshakeType {
    pub const certificate: u8 = 11u8;
    pub const certificate_request: u8 = 13u8;
    pub const certificate_status: u8 = 22u8;
    pub const certificate_url: u8 = 21u8;
    pub const certificate_verify: u8 = 15u8;
    pub const client_hello: u8 = 1u8;
    pub const client_key_exchange: u8 = 16u8;
    pub const finished: u8 = 20u8;
    pub const hello_request: u8 = 0u8;
    pub const hello_verify_request: u8 = 3u8;
    pub const server_hello: u8 = 2u8;
    pub const server_hello_done: u8 = 14u8;
    pub const server_key_exchange: u8 = 12u8;
    pub const session_ticket: u8 = 4u8;
    pub const supplemental_data: u8 = 23u8;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HandshakeType")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::HandshakeType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
