#[cfg(feature = "System+Xml+Schema+Datatype_long")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_long {
    __cordl_parent: crate::System::Xml::Schema::Datatype_integer,
}
#[cfg(feature = "System+Xml+Schema+Datatype_long")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_long =>
    "System.Xml.Schema"."Datatype_long"
);
#[cfg(feature = "System+Xml+Schema+Datatype_long")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_long {
    type Target = crate::System::Xml::Schema::Datatype_integer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_long")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_long {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_long")]
impl crate::System::Xml::Schema::Datatype_long {
    pub fn get_ListValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ListValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FacetsChecker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::FacetsChecker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::FacetsChecker = __cordl_object
            .invoke("get_FacetsChecker", ())?;
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
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Compare(
        &mut self,
        value1: *mut crate::System::Object,
        value2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (value1, value2))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasValueFacets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValueFacets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryParseValue(
        &mut self,
        s: *mut crate::System::String,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        nsmgr: *mut crate::System::Xml::IXmlNamespaceResolver,
        typedValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("TryParseValue", (s, nameTable, nsmgr, typedValue))?;
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
#[cfg(feature = "System+Xml+Schema+Datatype_long")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Datatype_long {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
