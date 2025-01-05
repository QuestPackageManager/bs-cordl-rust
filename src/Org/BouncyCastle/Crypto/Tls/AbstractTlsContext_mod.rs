#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mNonceRandom: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator,
    >,
    pub mSecureRandom: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
    pub mSecurityParameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    >,
    pub mClientVersion: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    >,
    pub mServerVersion: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    >,
    pub mSession: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    >,
    pub mUserObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext => "Org.BouncyCastle.Crypto.Tls"
    ."AbstractTlsContext"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ExportKeyingMaterial(
        &mut self,
        asciiLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context_value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("ExportKeyingMaterial", (asciiLabel, context_value, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        securityParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_object.into())
    }
    pub fn NextCounterValue() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NextCounterValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClientVersion(
        &mut self,
        clientVersion: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClientVersion", (clientVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResumableSession(
        &mut self,
        session: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetResumableSession", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetServerVersion(
        &mut self,
        serverVersion: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetServerVersion", (serverVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        securityParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, securityParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClientVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = __cordl_object.invoke("get_ClientVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsServer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NonceRandomGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator,
        > = __cordl_object.invoke("get_NonceRandomGenerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResumableSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsSession>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
        > = __cordl_object.invoke("get_ResumableSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SecureRandom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        > = __cordl_object.invoke("get_SecureRandom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SecurityParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        > = __cordl_object.invoke("get_SecurityParameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServerVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = __cordl_object.invoke("get_ServerVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_UserObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UserObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserObject", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsContext {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsContext")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext {
        unsafe { std::mem::transmute(self) }
    }
}
