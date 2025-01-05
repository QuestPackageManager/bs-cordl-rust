#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ProofOfPossessionSigningKeyBuilder {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _certRequest: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Crmf::CertRequest,
    >,
    pub _pubKeyInfo: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    >,
    pub _name: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub _publicKeyMAC: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder =>
    "Org.BouncyCastle.Crmf"."ProofOfPossessionSigningKeyBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
impl crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder {
    pub fn Build(
        &mut self,
        signer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKey,
        > = __cordl_object.invoke("Build", (signer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc0(
        certRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::CertRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certRequest))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc1(
        pubKeyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKeyInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn SetPublicKeyMac(
        &mut self,
        generator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PKMacBuilder,
        >,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder,
        > = __cordl_object.invoke("SetPublicKeyMac", (generator, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSender(
        &mut self,
        name: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder,
        > = __cordl_object.invoke("SetSender", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        certRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::CertRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        pubKeyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pubKeyInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+ProofOfPossessionSigningKeyBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::ProofOfPossessionSigningKeyBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
