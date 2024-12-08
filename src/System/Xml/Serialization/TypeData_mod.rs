#[cfg(feature = "System+Xml+Serialization+TypeData")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeData {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: *mut crate::System::Type,
    pub elementName: *mut crate::System::String,
    pub sType: crate::System::Xml::Serialization::SchemaTypes,
    pub listItemType: *mut crate::System::Type,
    pub typeName: *mut crate::System::String,
    pub fullTypeName: *mut crate::System::String,
    pub listItemTypeData: *mut crate::System::Xml::Serialization::TypeData,
    pub mappedType: *mut crate::System::Xml::Serialization::TypeData,
    pub facet: *mut crate::System::Xml::Schema::XmlSchemaPatternFacet,
    pub typeConvertor: *mut crate::System::Reflection::MethodInfo,
    pub hasPublicConstructor: bool,
    pub nullableOverride: bool,
}
#[cfg(feature = "System+Xml+Serialization+TypeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::TypeData =>
    "System.Xml.Serialization"."TypeData"
);
#[cfg(feature = "System+Xml+Serialization+TypeData")]
impl std::ops::Deref for crate::System::Xml::Serialization::TypeData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeData")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::TypeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeData")]
impl crate::System::Xml::Serialization::TypeData {
    pub fn ConvertForAssignment(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertForAssignment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn LookupTypeConvertor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LookupTypeConvertor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_TypeData_XmlSchemaPatternFacet1(
        _cordl_type: *mut crate::System::Type,
        elementName: *mut crate::System::String,
        isPrimitive: bool,
        mappedType: *mut crate::System::Xml::Serialization::TypeData,
        facet: *mut crate::System::Xml::Schema::XmlSchemaPatternFacet,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, elementName, isPrimitive, mappedType, facet),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Type_String__cordl_bool0(
        _cordl_type: *mut crate::System::Type,
        elementName: *mut crate::System::String,
        isPrimitive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, elementName, isPrimitive))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_TypeData_XmlSchemaPatternFacet1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        elementName: *mut crate::System::String,
        isPrimitive: bool,
        mappedType: *mut crate::System::Xml::Serialization::TypeData,
        facet: *mut crate::System::Xml::Schema::XmlSchemaPatternFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, elementName, isPrimitive, mappedType, facet),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_String__cordl_bool0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        elementName: *mut crate::System::String,
        isPrimitive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, elementName, isPrimitive))?;
        Ok(__cordl_ret)
    }
    pub fn get_FullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasPublicConstructor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasPublicConstructor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsComplexType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsComplexType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsListType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsListType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNullable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNullable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsValueType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsXsdType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsXsdType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ListItemType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ListItemType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ListItemTypeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::TypeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::TypeData = __cordl_object
            .invoke("get_ListItemTypeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Serialization::SchemaTypes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Serialization::SchemaTypes = __cordl_object
            .invoke("get_SchemaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsNullable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsNullable", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeData")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::TypeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
