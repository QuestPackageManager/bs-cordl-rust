#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsContext {
    __cordl_parent: crate::System::Object,
    pub mNonceRandom: *mut crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator,
    pub mSecureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mSecurityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    pub mClientVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mServerVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mSession: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    pub mUserObject: *mut crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext => "Org.BouncyCastle.Crypto.Tls"
    ."AbstractTlsContext"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    pub fn get_UserObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_UserObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UserObject(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserObject", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetClientVersion(
        &mut self,
        clientVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClientVersion", (clientVersion))?;
        Ok(__cordl_ret)
    }
    pub fn get_ServerVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_ServerVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExportKeyingMaterial(
        &mut self,
        asciiLabel: *mut crate::System::String,
        context_value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ExportKeyingMaterial", (asciiLabel, context_value, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        securityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_NonceRandomGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator = __cordl_object
            .invoke("get_NonceRandomGenerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_ClientVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetResumableSession(
        &mut self,
        session: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetResumableSession", (session))?;
        Ok(__cordl_ret)
    }
    pub fn SetServerVersion(
        &mut self,
        serverVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetServerVersion", (serverVersion))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsServer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResumableSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession = __cordl_object
            .invoke("get_ResumableSession", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SecurityParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters = __cordl_object
            .invoke("get_SecurityParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SecureRandom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Security::SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Security::SecureRandom = __cordl_object
            .invoke("get_SecureRandom", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        securityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
