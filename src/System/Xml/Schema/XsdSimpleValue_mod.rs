#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdSimpleValue {
    __cordl_parent: crate::System::Object,
    pub xmlType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    pub typedValue: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdSimpleValue =>
    "System.Xml.Schema"."XsdSimpleValue"
);
#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdSimpleValue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdSimpleValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
impl crate::System::Xml::Schema::XsdSimpleValue {
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSimpleType = __cordl_object
            .invoke("get_XmlType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_TypedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        st: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (st, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        st: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (st, value))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdSimpleValue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XsdSimpleValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
