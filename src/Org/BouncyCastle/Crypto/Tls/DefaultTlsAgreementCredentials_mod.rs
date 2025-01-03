#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsAgreementCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsAgreementCredentials,
    pub mCertificate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    >,
    pub mPrivateKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
    pub mBasicAgreement: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBasicAgreement,
    >,
    pub mTruncateAgreement: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsAgreementCredentials"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsAgreementCredentials;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    pub fn GenerateAgreement(
        &mut self,
        peerPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateAgreement", (peerPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate, privateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificate, privateKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        > = __cordl_object.invoke("get_Certificate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
