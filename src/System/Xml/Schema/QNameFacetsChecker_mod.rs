#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct QNameFacetsChecker {
    __cordl_parent: crate::System::Xml::Schema::FacetsChecker,
}
#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::QNameFacetsChecker =>
    "System.Xml.Schema"."QNameFacetsChecker"
);
#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
impl std::ops::Deref for crate::System::Xml::Schema::QNameFacetsChecker {
    type Target = crate::System::Xml::Schema::FacetsChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
impl std::ops::DerefMut for crate::System::Xml::Schema::QNameFacetsChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
impl crate::System::Xml::Schema::QNameFacetsChecker {
    pub fn MatchEnumeration_Object_XmlSchemaDatatype0(
        &mut self,
        value: *mut crate::System::Object,
        enumeration: *mut crate::System::Collections::ArrayList,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchEnumeration", (value, enumeration, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn MatchEnumeration_XmlQualifiedName1(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
        enumeration: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchEnumeration", (value, enumeration))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Object0(
        &mut self,
        value: *mut crate::System::Object,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_XmlQualifiedName1(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+QNameFacetsChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::QNameFacetsChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
