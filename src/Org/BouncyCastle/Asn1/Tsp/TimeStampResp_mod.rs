#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampResp {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub pkiStatusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    pub timeStampToken: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Tsp::TimeStampResp =>
    "Org.BouncyCastle.Asn1.Tsp"."TimeStampResp"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Tsp::TimeStampResp {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Tsp::TimeStampResp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
impl crate::Org::BouncyCastle::Asn1::Tsp::TimeStampResp {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_PkiStatusInfo_ContentInfo1(
        pkiStatusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        timeStampToken: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiStatusInfo, timeStampToken))?;
        Ok(__cordl_object)
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
    pub fn _ctor_PkiStatusInfo_ContentInfo1(
        &mut self,
        pkiStatusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        timeStampToken: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pkiStatusInfo, timeStampToken))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_TimeStampToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo = __cordl_object
            .invoke("get_TimeStampToken", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TimeStampResp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Tsp::TimeStampResp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
