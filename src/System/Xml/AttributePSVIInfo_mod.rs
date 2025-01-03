#[cfg(feature = "System+Xml+AttributePSVIInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributePSVIInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub typedAttributeValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub attributeSchemaInfo: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaInfo,
    >,
}
#[cfg(feature = "System+Xml+AttributePSVIInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::AttributePSVIInfo => "System.Xml"
    ."AttributePSVIInfo"
);
#[cfg(feature = "System+Xml+AttributePSVIInfo")]
impl std::ops::Deref for crate::System::Xml::AttributePSVIInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+AttributePSVIInfo")]
impl std::ops::DerefMut for crate::System::Xml::AttributePSVIInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+AttributePSVIInfo")]
impl crate::System::Xml::AttributePSVIInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+AttributePSVIInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::AttributePSVIInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
