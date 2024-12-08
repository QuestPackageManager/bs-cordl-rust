#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct CertRequest {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub certTemplate: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    pub controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::CertRequest =>
    "Org.BouncyCastle.Asn1.Crmf"."CertRequest"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::CertRequest {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::CertRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
impl crate::Org::BouncyCastle::Asn1::Crmf::CertRequest {
    pub fn get_Controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls = __cordl_object
            .invoke("get_Controls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertTemplate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate = __cordl_object
            .invoke("get_CertTemplate", ())?;
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
    pub fn _ctor_i32_CertTemplate_Controls1(
        &mut self,
        certReqId: i32,
        certTemplate: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, certTemplate, controls))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerInteger_CertTemplate_Controls2(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        certTemplate: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, certTemplate, controls))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertReqID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_CertReqID", ())?;
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
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_CertTemplate_Controls1(
        certReqId: i32,
        certTemplate: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, certTemplate, controls))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger_CertTemplate_Controls2(
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        certTemplate: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
        controls: *mut crate::Org::BouncyCastle::Asn1::Crmf::Controls,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, certTemplate, controls))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::CertRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
