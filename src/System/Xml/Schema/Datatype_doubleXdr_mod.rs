#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_doubleXdr {
    __cordl_parent: crate::System::Xml::Schema::Datatype_double,
}
#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_doubleXdr =>
    "System.Xml.Schema"."Datatype_doubleXdr"
);
#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_doubleXdr {
    type Target = crate::System::Xml::Schema::Datatype_double;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_doubleXdr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
impl crate::System::Xml::Schema::Datatype_doubleXdr {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ParseValue(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        nsmgr: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ParseValue", (s, nameTable, nsmgr))?;
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
}
#[cfg(feature = "System+Xml+Schema+Datatype_doubleXdr")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_doubleXdr {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
