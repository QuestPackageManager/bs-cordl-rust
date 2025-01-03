#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
#[repr(C)]
#[derive(Debug)]
pub struct V2Form {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub issuerName: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    >,
    pub baseCertificateID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
    >,
    pub objectDigestInfo: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::V2Form =>
    "Org.BouncyCastle.Asn1.X509"."V2Form"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::V2Form {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::V2Form {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
impl crate::Org::BouncyCastle::Asn1::X509::V2Form {
    pub fn GetInstance_Asn1TaggedObject__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
        explicitly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::V2Form>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::V2Form,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (obj, explicitly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Il2CppObject1(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::V2Form>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::V2Form,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence4(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GeneralNames0(
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerName))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GeneralNames_IssuerSerial1(
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        baseCertificateID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerName, baseCertificateID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GeneralNames_IssuerSerial_ObjectDigestInfo3(
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        baseCertificateID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
        >,
        objectDigestInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerName, baseCertificateID, objectDigestInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GeneralNames_ObjectDigestInfo2(
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        objectDigestInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerName, objectDigestInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence4(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralNames0(
        &mut self,
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralNames_IssuerSerial1(
        &mut self,
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        baseCertificateID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerName, baseCertificateID))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralNames_IssuerSerial_ObjectDigestInfo3(
        &mut self,
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        baseCertificateID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
        >,
        objectDigestInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerName, baseCertificateID, objectDigestInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralNames_ObjectDigestInfo2(
        &mut self,
        issuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        >,
        objectDigestInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerName, objectDigestInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseCertificateID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::IssuerSerial>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
        > = __cordl_object.invoke("get_BaseCertificateID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IssuerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralNames>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
        > = __cordl_object.invoke("get_IssuerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectDigestInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ObjectDigestInfo,
        > = __cordl_object.invoke("get_ObjectDigestInfo", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2Form")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::X509::V2Form {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
