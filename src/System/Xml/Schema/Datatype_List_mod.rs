#[cfg(feature = "System+Xml+Schema+Datatype_List")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_List {
    __cordl_parent: crate::System::Xml::Schema::Datatype_anySimpleType,
    pub itemType: *mut crate::System::Xml::Schema::DatatypeImplementation,
    pub minListSize: i32,
}
#[cfg(feature = "System+Xml+Schema+Datatype_List")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_List =>
    "System.Xml.Schema"."Datatype_List"
);
#[cfg(feature = "System+Xml+Schema+Datatype_List")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_List {
    type Target = crate::System::Xml::Schema::Datatype_anySimpleType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_List")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_List")]
impl crate::System::Xml::Schema::Datatype_List {
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
    pub fn CreateValueConverter(
        &mut self,
        schemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlValueConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlValueConverter = __cordl_object
            .invoke("CreateValueConverter", (schemaType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Xml::Schema::DatatypeImplementation,
        minListSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, minListSize))?;
        Ok(__cordl_object)
    }
    pub fn TryParseValue_Object0(
        &mut self,
        value: *mut crate::System::Object,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        namespaceResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
        typedValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("TryParseValue", (value, nameTable, namespaceResolver, typedValue))?;
        Ok(__cordl_ret)
    }
    pub fn TryParseValue_String1(
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
        _cordl_type: *mut crate::System::Xml::Schema::DatatypeImplementation,
        minListSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, minListSize))?;
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
    pub fn get_TokenizedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlTokenizedType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTokenizedType = __cordl_object
            .invoke("get_TokenizedType", ())?;
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
    pub fn get_ValidRestrictionFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::RestrictionFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::RestrictionFlags = __cordl_object
            .invoke("get_ValidRestrictionFlags", ())?;
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
}
#[cfg(feature = "System+Xml+Schema+Datatype_List")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Datatype_List {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}