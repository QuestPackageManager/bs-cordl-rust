#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_NCName {
    __cordl_parent: crate::System::Xml::Schema::Datatype_Name,
}
#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_NCName =>
    "System.Xml.Schema"."Datatype_NCName"
);
#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_NCName {
    type Target = crate::System::Xml::Schema::Datatype_Name;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_NCName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
impl crate::System::Xml::Schema::Datatype_NCName {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_NCName")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Datatype_NCName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
