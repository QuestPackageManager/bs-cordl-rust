#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OriginatorInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub origCerts: *mut crate::System::Collections::IList,
    pub origCrls: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::OriginatorInfoGenerator
    => "Org.BouncyCastle.Cms"."OriginatorInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IX509Store1(
        origCerts: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCerts))?;
        Ok(__cordl_object)
    }
    pub fn New_IX509Store_IX509Store2(
        origCerts: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
        origCrls: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCerts, origCrls))?;
        Ok(__cordl_object)
    }
    pub fn New_X509Certificate0(
        origCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCert))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_IX509Store1(
        &mut self,
        origCerts: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCerts))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IX509Store_IX509Store2(
        &mut self,
        origCerts: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
        origCrls: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCerts, origCrls))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Certificate0(
        &mut self,
        origCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCert))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
