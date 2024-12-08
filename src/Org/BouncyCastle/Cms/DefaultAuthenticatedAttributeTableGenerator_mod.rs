#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultAuthenticatedAttributeTableGenerator {
    __cordl_parent: crate::System::Object,
    pub table: *mut crate::System::Collections::IDictionary,
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::DefaultAuthenticatedAttributeTableGenerator =>
    "Org.BouncyCastle.Cms"."DefaultAuthenticatedAttributeTableGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::DefaultAuthenticatedAttributeTableGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::DefaultAuthenticatedAttributeTableGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
impl crate::Org::BouncyCastle::Cms::DefaultAuthenticatedAttributeTableGenerator {
    pub fn CreateStandardAttributeTable(
        &mut self,
        parameters: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("CreateStandardAttributeTable", (parameters))?;
        Ok(__cordl_ret)
    }
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AttributeTable1(
        &mut self,
        attributeTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributeTable))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_AttributeTable1(
        attributeTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributeTable))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultAuthenticatedAttributeTableGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::DefaultAuthenticatedAttributeTableGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
