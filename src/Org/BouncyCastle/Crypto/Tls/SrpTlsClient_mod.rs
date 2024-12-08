#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
#[repr(C)]
#[derive(Debug)]
pub struct SrpTlsClient {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient,
    pub mGroupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
    pub mIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPassword: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::SrpTlsClient =>
    "Org.BouncyCastle.Crypto.Tls"."SrpTlsClient"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsClient {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
impl crate::Org::BouncyCastle::Crypto::Tls::SrpTlsClient {
    pub fn CreateSrpKeyExchange(
        &mut self,
        keyExchange: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange = __cordl_object
            .invoke("CreateSrpKeyExchange", (keyExchange))?;
        Ok(__cordl_ret)
    }
    pub fn GetAuthentication(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication = __cordl_object
            .invoke("GetAuthentication", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCipherSuites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetCipherSuites", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClientExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("GetClientExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange = __cordl_object
            .invoke("GetKeyExchange", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (identity, password))?;
        Ok(__cordl_object)
    }
    pub fn New_TlsCipherFactory_Il2CppArray_Il2CppArray1(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, identity, password))?;
        Ok(__cordl_object)
    }
    pub fn New_TlsCipherFactory_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray2(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        groupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, groupVerifier, identity, password))?;
        Ok(__cordl_object)
    }
    pub fn ProcessServerExtensions(
        &mut self,
        serverExtensions: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerExtensions", (serverExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (identity, password))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsCipherFactory_Il2CppArray_Il2CppArray1(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, identity, password))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsCipherFactory_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray2(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        groupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, groupVerifier, identity, password))?;
        Ok(__cordl_ret)
    }
    pub fn get_RequireSrpServerExtension(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequireSrpServerExtension", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrpTlsClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SrpTlsClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
