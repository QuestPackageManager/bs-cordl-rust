#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
#[repr(C)]
#[derive(Debug)]
pub struct CAKeyUpdAnnContent {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub oldWithNew: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    pub newWithOld: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    pub newWithNew: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CAKeyUpdAnnContent
    => "Org.BouncyCastle.Asn1.Cmp"."CAKeyUpdAnnContent"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CAKeyUpdAnnContent {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CAKeyUpdAnnContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CAKeyUpdAnnContent {
    pub fn get_NewWithNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate = __cordl_object
            .invoke("get_NewWithNew", ())?;
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
    pub fn _ctor(
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
    pub fn get_NewWithOld(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate = __cordl_object
            .invoke("get_NewWithOld", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OldWithNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate = __cordl_object
            .invoke("get_OldWithNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CAKeyUpdAnnContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CAKeyUpdAnnContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
