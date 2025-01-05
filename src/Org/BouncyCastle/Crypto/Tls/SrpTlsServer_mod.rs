#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
#[repr(C)]
#[derive(Debug)]
pub struct SrpTlsServer {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsServer,
    pub mSrpIdentityManager: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager,
    >,
    pub mSrpIdentity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mLoginParameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::SrpTlsServer =>
    "Org.BouncyCastle.Crypto.Tls"."SrpTlsServer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsServer {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsServer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
impl crate::Org::BouncyCastle::Crypto::Tls::SrpTlsServer {
    pub fn CreateSrpKeyExchange(
        &mut self,
        keyExchange: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
        > = __cordl_object.invoke("CreateSrpKeyExchange", (keyExchange))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCipherSuites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetCipherSuites", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
        > = __cordl_object.invoke("GetCredentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaSignerCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
        > = __cordl_object.invoke("GetDsaSignerCredentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
        > = __cordl_object.invoke("GetKeyExchange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaSignerCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
        > = __cordl_object.invoke("GetRsaSignerCredentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedCipherSuite(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSelectedCipherSuite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_TlsCipherFactory_TlsSrpIdentityManager1(
        cipherFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        >,
        srpIdentityManager: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, srpIdentityManager))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TlsSrpIdentityManager0(
        srpIdentityManager: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (srpIdentityManager))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessClientExtensions(
        &mut self,
        clientExtensions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientExtensions", (clientExtensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TlsCipherFactory_TlsSrpIdentityManager1(
        &mut self,
        cipherFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        >,
        srpIdentityManager: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, srpIdentityManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TlsSrpIdentityManager0(
        &mut self,
        srpIdentityManager: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (srpIdentityManager))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsServer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
