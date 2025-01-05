#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthorityKeyIdentifierStructure {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X509::AuthorityKeyIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Extension::AuthorityKeyIdentifierStructure =>
    "Org.BouncyCastle.X509.Extension"."AuthorityKeyIdentifierStructure"
);
#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
impl std::ops::Deref
for crate::Org::BouncyCastle::X509::Extension::AuthorityKeyIdentifierStructure {
    type Target = crate::Org::BouncyCastle::Asn1::X509::AuthorityKeyIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Extension::AuthorityKeyIdentifierStructure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
impl crate::Org::BouncyCastle::X509::Extension::AuthorityKeyIdentifierStructure {
    pub fn FromCertificate(
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromKey(
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromKey", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1OctetString0(
        encodedValue: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedValue))?;
        Ok(__cordl_object.into())
    }
    pub fn New_AsymmetricKeyParameter2(
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509Certificate1(
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Asn1OctetString0(
        &mut self,
        encodedValue: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsymmetricKeyParameter2(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509Certificate1(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+AuthorityKeyIdentifierStructure")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Extension::AuthorityKeyIdentifierStructure {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
