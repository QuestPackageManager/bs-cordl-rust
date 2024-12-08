#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
#[repr(C)]
#[derive(Debug)]
pub struct PollRepContent {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub checkAfter: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub reason: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PollRepContent =>
    "Org.BouncyCastle.Asn1.Cmp"."PollRepContent"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PollRepContent {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PollRepContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PollRepContent {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger_DerInteger1(
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        checkAfter: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, checkAfter))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger_DerInteger_PkiFreeText2(
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        checkAfter: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        reason: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certReqId, checkAfter, reason))?;
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
    pub fn _ctor_DerInteger_DerInteger1(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        checkAfter: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, checkAfter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerInteger_DerInteger_PkiFreeText2(
        &mut self,
        certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        checkAfter: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        reason: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certReqId, checkAfter, reason))?;
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
    pub fn get_CheckAfter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_CheckAfter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiFreeText = __cordl_object
            .invoke("get_Reason", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PollRepContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PollRepContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
