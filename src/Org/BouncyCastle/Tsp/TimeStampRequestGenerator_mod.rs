#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampRequestGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub reqPolicy: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub certReq: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
    pub extensions: *mut crate::System::Collections::IDictionary,
    pub extOrdering: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Tsp::TimeStampRequestGenerator => "Org.BouncyCastle.Tsp"
    ."TimeStampRequestGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampRequestGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampRequestGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampRequestGenerator {
    pub fn AddExtension_DerObjectIdentifier_Asn1Encodable2(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        extValue: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extValue))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_DerObjectIdentifier_Il2CppArray3(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        extValue: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extValue))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_Il2CppString_Asn1Encodable0(
        &mut self,
        oid: *mut quest_hook::libil2cpp::Il2CppString,
        critical: bool,
        value: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_Il2CppString_Il2CppArray1(
        &mut self,
        oid: *mut quest_hook::libil2cpp::Il2CppString,
        critical: bool,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, value))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_DerObjectIdentifier2(
        &mut self,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest = __cordl_object
            .invoke("Generate", (digestAlgorithm, digest))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_DerObjectIdentifier_BigInteger3(
        &mut self,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        nonce: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest = __cordl_object
            .invoke("Generate", (digestAlgorithm, digest, nonce))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_Il2CppString0(
        &mut self,
        digestAlgorithm: *mut quest_hook::libil2cpp::Il2CppString,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest = __cordl_object
            .invoke("Generate", (digestAlgorithm, digest))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_Il2CppString_BigInteger1(
        &mut self,
        digestAlgorithmOid: *mut quest_hook::libil2cpp::Il2CppString,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        nonce: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest = __cordl_object
            .invoke("Generate", (digestAlgorithmOid, digest, nonce))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCertReq(
        &mut self,
        certReq: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCertReq", (certReq))?;
        Ok(__cordl_ret)
    }
    pub fn SetReqPolicy(
        &mut self,
        reqPolicy: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReqPolicy", (reqPolicy))?;
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
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampRequestGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampRequestGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
