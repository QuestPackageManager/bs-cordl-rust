#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct StringFacetsChecker {
    __cordl_parent: crate::System::Xml::Schema::FacetsChecker,
}
#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::StringFacetsChecker =>
    "System.Xml.Schema"."StringFacetsChecker"
);
#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
impl std::ops::Deref for crate::System::Xml::Schema::StringFacetsChecker {
    type Target = crate::System::Xml::Schema::FacetsChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
impl std::ops::DerefMut for crate::System::Xml::Schema::StringFacetsChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
impl crate::System::Xml::Schema::StringFacetsChecker {
    pub fn CheckBuiltInFacets(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
        typeCode: crate::System::Xml::Schema::XmlTypeCode,
        verifyUri: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckBuiltInFacets", (s, typeCode, verifyUri))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Il2CppObject0(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Il2CppString1(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Il2CppString__cordl_bool2(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
        verifyUri: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype, verifyUri))?;
        Ok(__cordl_ret)
    }
    pub fn MatchEnumeration_Il2CppObject0(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
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
    pub fn MatchEnumeration_Il2CppString1(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "System+Xml+Schema+StringFacetsChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::StringFacetsChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
