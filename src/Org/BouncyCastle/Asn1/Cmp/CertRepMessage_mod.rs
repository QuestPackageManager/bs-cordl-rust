#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct CertRepMessage {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub caPubs: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub response: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CertRepMessage =>
    "Org.BouncyCastle.Asn1.Cmp"."CertRepMessage"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CertRepMessage {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CertRepMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CertRepMessage {
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
    pub fn GetResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CertResponse,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CertResponse,
        > = __cordl_object.invoke("GetResponse", ())?;
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
    pub fn _ctor_Il2CppArray_Il2CppArray1(
        &mut self,
        caPubs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
        response: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CertResponse,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (caPubs, response))?;
        Ok(__cordl_ret)
    }
    pub fn GetCAPubs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        > = __cordl_object.invoke("GetCAPubs", ())?;
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
    pub fn New_Il2CppArray_Il2CppArray1(
        caPubs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
        response: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CertResponse,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (caPubs, response))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertRepMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CertRepMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
