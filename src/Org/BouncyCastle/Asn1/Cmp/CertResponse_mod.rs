#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct CertResponse {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    pub certifiedKeyPair: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair,
    pub rspInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CertResponse =>
    "Org.BouncyCastle.Asn1.Cmp"."CertResponse"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CertResponse {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CertResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CertResponse {
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo = __cordl_object
            .invoke("get_Status", ())?;
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
    pub fn _ctor_DerInteger_PkiStatusInfo1(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, status))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerInteger_PkiStatusInfo_CertifiedKeyPair_Asn1OctetString2(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        certifiedKeyPair: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair,
        rspInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, status, certifiedKeyPair, rspInfo))?;
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
    pub fn get_CertifiedKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair = __cordl_object
            .invoke("get_CertifiedKeyPair", ())?;
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
    pub fn New_DerInteger_PkiStatusInfo1(
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, status))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger_PkiStatusInfo_CertifiedKeyPair_Asn1OctetString2(
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        certifiedKeyPair: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair,
        rspInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, status, certifiedKeyPair, rspInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CertResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
