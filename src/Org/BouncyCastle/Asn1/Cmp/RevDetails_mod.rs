#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
#[repr(C)]
#[derive(Debug)]
pub struct RevDetails {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certDetails: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    pub crlEntryDetails: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::RevDetails =>
    "Org.BouncyCastle.Asn1.Cmp"."RevDetails"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::RevDetails {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::RevDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
impl crate::Org::BouncyCastle::Asn1::Cmp::RevDetails {
    pub fn get_CrlEntryDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_CrlEntryDetails", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertTemplate1(
        &mut self,
        certDetails: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certDetails))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertTemplate_X509Extensions2(
        &mut self,
        certDetails: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        crlEntryDetails: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certDetails, crlEntryDetails))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate = __cordl_object
            .invoke("get_CertDetails", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_CertTemplate1(
        certDetails: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certDetails))?;
        Ok(__cordl_object)
    }
    pub fn New_CertTemplate_X509Extensions2(
        certDetails: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        crlEntryDetails: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certDetails, crlEntryDetails))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevDetails")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::RevDetails {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
