#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlReflectionImporter {
    __cordl_parent: crate::System::Object,
    pub initialDefaultNamespace: *mut crate::System::String,
    pub attributeOverrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
    pub includedTypes: *mut crate::System::Collections::ArrayList,
    pub helper: *mut crate::System::Xml::Serialization::ReflectionHelper,
    pub arrayChoiceCount: i32,
    pub relatedMaps: *mut crate::System::Collections::ArrayList,
    pub allowPrivateTypes: bool,
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlReflectionImporter => "System.Xml.Serialization"
    ."XmlReflectionImporter"
);
#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlReflectionImporter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlReflectionImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
impl crate::System::Xml::Serialization::XmlReflectionImporter {
    #[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter+__c")]
    pub type __c = crate::System::Xml::Serialization::XmlReflectionImporter___c;
    pub fn CanBeNull(
        &mut self,
        _cordl_type: *mut crate::System::Xml::Serialization::TypeData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanBeNull", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMapMember(
        &mut self,
        declaringType: *mut crate::System::Type,
        rmember: *mut crate::System::Xml::Serialization::XmlReflectionMember,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMember = __cordl_object
            .invoke("CreateMapMember", (declaringType, rmember, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTypeMapping(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultXmlType: *mut crate::System::String,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke(
                "CreateTypeMapping",
                (typeData, root, defaultXmlType, defaultNamespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultValue(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        defaultValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetDefaultValue", (typeData, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn GetReflectionMembers(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Xml::Serialization::XmlReflectionMember,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Xml::Serialization::XmlReflectionMember,
        > = __cordl_object.invoke("GetReflectionMembers", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeNamespace(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTypeNamespace", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportAnyElementInfo(
        &mut self,
        defaultNamespace: *mut crate::System::String,
        rmember: *mut crate::System::Xml::Serialization::XmlReflectionMember,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMemberElement,
        atts: *mut crate::System::Xml::Serialization::XmlAttributes,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList = __cordl_object
            .invoke("ImportAnyElementInfo", (defaultNamespace, rmember, member, atts))?;
        Ok(__cordl_ret)
    }
    pub fn ImportClassMapping_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
        isBaseType: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke(
                "ImportClassMapping",
                (_cordl_type, root, defaultNamespace, isBaseType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportClassMapping_TypeData1(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
        isBaseType: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke(
                "ImportClassMapping",
                (typeData, root, defaultNamespace, isBaseType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportElementInfo(
        &mut self,
        cls: *mut crate::System::Type,
        defaultName: *mut crate::System::String,
        defaultNamespace: *mut crate::System::String,
        defaultType: *mut crate::System::Type,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMemberElement,
        atts: *mut crate::System::Xml::Serialization::XmlAttributes,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList = __cordl_object
            .invoke(
                "ImportElementInfo",
                (cls, defaultName, defaultNamespace, defaultType, member, atts),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportEnumMapping(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportEnumMapping", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportIncludedTypes(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportIncludedTypes", (_cordl_type, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportListMapping_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
        atts: *mut crate::System::Xml::Serialization::XmlAttributes,
        nestingLevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke(
                "ImportListMapping",
                (_cordl_type, root, defaultNamespace, atts, nestingLevel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportListMapping_TypeData1(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
        atts: *mut crate::System::Xml::Serialization::XmlAttributes,
        nestingLevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke(
                "ImportListMapping",
                (typeData, root, defaultNamespace, atts, nestingLevel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportPrimitiveMapping(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportPrimitiveMapping", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportTextElementInfo(
        &mut self,
        list: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
        defaultType: *mut crate::System::Type,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMemberElement,
        atts: *mut crate::System::Xml::Serialization::XmlAttributes,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ImportTextElementInfo",
                (list, defaultType, member, atts, defaultNamespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ImportTypeMapping_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportTypeMapping", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ImportTypeMapping_TypeData_XmlRootAttribute_String3(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportTypeMapping", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportTypeMapping_Type_String1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportTypeMapping", (_cordl_type, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportTypeMapping_Type_XmlRootAttribute_String2(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportTypeMapping", (_cordl_type, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportXmlNodeMapping(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportXmlNodeMapping", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ImportXmlSerializableMapping(
        &mut self,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("ImportXmlSerializableMapping", (typeData, root, defaultNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn IncludeType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncludeType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        attributeOverrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributeOverrides, defaultNamespace))?;
        Ok(__cordl_object)
    }
    pub fn RegisterDerivedMap(
        &mut self,
        map: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        derivedMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDerivedMap", (map, derivedMap))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        attributeOverrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributeOverrides, defaultNamespace))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionImporter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlReflectionImporter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
