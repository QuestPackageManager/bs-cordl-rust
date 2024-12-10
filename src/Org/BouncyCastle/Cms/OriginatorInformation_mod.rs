#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct OriginatorInformation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub originatorInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::OriginatorInformation =>
    "Org.BouncyCastle.Cms"."OriginatorInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::OriginatorInformation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::OriginatorInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
impl crate::Org::BouncyCastle::Cms::OriginatorInformation {
    pub fn GetCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        > = __cordl_object.invoke("GetCertificates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        > = __cordl_object.invoke("GetCrls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (originatorInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Structure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        > = __cordl_object.invoke("ToAsn1Structure", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        originatorInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (originatorInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::OriginatorInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
