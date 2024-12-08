#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsProtocolHandler {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsProtocolHandler => "Org.BouncyCastle.Crypto.Tls"
    ."TlsProtocolHandler"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocolHandler {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocolHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsProtocolHandler {
    pub fn _ctor_SecureRandom0(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_SecureRandom1(
        &mut self,
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn New_SecureRandom0(
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_SecureRandom1(
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocolHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocolHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
