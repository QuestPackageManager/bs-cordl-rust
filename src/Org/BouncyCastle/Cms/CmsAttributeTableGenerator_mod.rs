#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAttributeTableGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAttributeTableGenerator => "Org.BouncyCastle.Cms"
    ."CmsAttributeTableGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
    pub fn GetAttributes(
        &mut self,
        parameters: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("GetAttributes", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
