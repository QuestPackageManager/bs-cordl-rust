#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct V2TbsCertListGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub thisUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub nextUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    pub crlEntries: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator =>
    "Org.BouncyCastle.Asn1.X509"."V2TbsCertListGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    pub fn AddCrlEntry_Asn1Sequence0(
        &mut self,
        crlEntry: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (crlEntry))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_DerInteger_DerUtcTime_i32_1(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        revocationDate: *mut crate::Org::BouncyCastle::Asn1::DerUtcTime,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (userCertificate, revocationDate, reason))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_DerInteger_Time_X509Extensions4(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        revocationDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (userCertificate, revocationDate, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_DerInteger_Time_i32_2(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        revocationDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (userCertificate, revocationDate, reason))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_DerInteger_Time_i32_DerGeneralizedTime3(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        revocationDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
        reason: i32,
        invalidityDate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddCrlEntry",
                (userCertificate, revocationDate, reason, invalidityDate),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTbsCertList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList = __cordl_object
            .invoke("GenerateTbsCertList", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetExtensions(
        &mut self,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExtensions", (extensions))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuer(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret)
    }
    pub fn SetNextUpdate_DerUtcTime0(
        &mut self,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerUtcTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNextUpdate", (nextUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn SetNextUpdate_Time1(
        &mut self,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNextUpdate", (nextUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn SetSignature(
        &mut self,
        signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignature", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn SetThisUpdate_DerUtcTime0(
        &mut self,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerUtcTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetThisUpdate", (thisUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn SetThisUpdate_Time1(
        &mut self,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetThisUpdate", (thisUpdate))?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
