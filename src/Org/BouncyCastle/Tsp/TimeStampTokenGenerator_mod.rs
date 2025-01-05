#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampTokenGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub accuracySeconds: i32,
    pub accuracyMillis: i32,
    pub accuracyMicros: i32,
    pub ordering: bool,
    pub tsa: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub key: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
    pub cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    pub digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub signedAttr: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    >,
    pub unsignedAttr: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    >,
    pub x509Certs: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::IX509Store,
    >,
    pub x509Crls: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::IX509Store,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::TimeStampTokenGenerator
    => "Org.BouncyCastle.Tsp"."TimeStampTokenGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    pub fn Generate(
        &mut self,
        request: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        >,
        serialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        genTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Tsp::TimeStampToken,
        > = __cordl_object.invoke("Generate", (request, serialNumber, genTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AsymmetricKeyParameter_X509Certificate_Il2CppString_Il2CppString0(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, cert, digestOID, tsaPolicyOID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_AttributeTable_AttributeTable1(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
        unsignedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetAccuracyMicros(
        &mut self,
        accuracyMicros: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracyMicros", (accuracyMicros))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAccuracyMillis(
        &mut self,
        accuracyMillis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracyMillis", (accuracyMillis))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAccuracySeconds(
        &mut self,
        accuracySeconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracySeconds", (accuracySeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCertificates(
        &mut self,
        certificates: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCertificates", (certificates))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCrls(
        &mut self,
        crls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCrls", (crls))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOrdering(
        &mut self,
        ordering: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOrdering", (ordering))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTsa(
        &mut self,
        tsa: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTsa", (tsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsymmetricKeyParameter_X509Certificate_Il2CppString_Il2CppString0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, cert, digestOID, tsaPolicyOID))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AttributeTable_AttributeTable1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
        unsignedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
