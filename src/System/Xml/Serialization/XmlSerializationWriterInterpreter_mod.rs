#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter_CallbackInfo {
    __cordl_parent: crate::System::Object,
    pub _swi: *mut crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
    pub _typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo =>
    "System.Xml.Serialization"."XmlSerializationWriterInterpreter/CallbackInfo"
);
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    pub fn New(
        swi: *mut crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (swi, typeMap))?;
        Ok(__cordl_object)
    }
    pub fn WriteEnum(
        &mut self,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnum", (ob))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObject(
        &mut self,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (ob))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        swi: *mut crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (swi, typeMap))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationWriter,
    pub _typeMap: *mut crate::System::Xml::Serialization::XmlMapping,
    pub _format: crate::System::Xml::Serialization::SerializationFormat,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationWriterInterpreter =>
    "System.Xml.Serialization"."XmlSerializationWriterInterpreter"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    type Target = crate::System::Xml::Serialization::XmlSerializationWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    #[cfg(
        feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
    )]
    pub type CallbackInfo = crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo;
    pub fn GetEnumXmlValue(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEnumXmlValue", (typeMap, ob))?;
        Ok(__cordl_ret)
    }
    pub fn GetListCount(
        &mut self,
        listType: *mut crate::System::Xml::Serialization::TypeData,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetListCount", (listType, ob))?;
        Ok(__cordl_ret)
    }
    pub fn GetMemberValue(
        &mut self,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMember,
        ob: *mut crate::System::Object,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetMemberValue", (member, ob, isValueList))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringValue(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        _cordl_type: *mut crate::System::Xml::Serialization::TypeData,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringValue", (typeMap, _cordl_type, value))?;
        Ok(__cordl_ret)
    }
    pub fn InitCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn MemberHasValue(
        &mut self,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMember,
        ob: *mut crate::System::Object,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MemberHasValue", (member, ob, isValueList))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        typeMap: *mut crate::System::Xml::Serialization::XmlMapping,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeMap))?;
        Ok(__cordl_object)
    }
    pub fn WriteAnyElementContent(
        &mut self,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
        memberValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAnyElementContent", (member, memberValue))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAttributeMembers(
        &mut self,
        map: *mut crate::System::Xml::Serialization::ClassMap,
        ob: *mut crate::System::Object,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAttributeMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementMembers(
        &mut self,
        map: *mut crate::System::Xml::Serialization::ClassMap,
        ob: *mut crate::System::Object,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEnumElement(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
        element: *mut crate::System::String,
        namesp: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnumElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret)
    }
    pub fn WriteListContent(
        &mut self,
        container: *mut crate::System::Object,
        listType: *mut crate::System::Xml::Serialization::TypeData,
        map: *mut crate::System::Xml::Serialization::ListMap,
        ob: *mut crate::System::Object,
        targetString: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteListContent", (container, listType, map, ob, targetString))?;
        Ok(__cordl_ret)
    }
    pub fn WriteListElement(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
        element: *mut crate::System::String,
        namesp: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteListElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMemberElement(
        &mut self,
        elem: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        memberValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMemberElement", (elem, memberValue))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMembers(
        &mut self,
        map: *mut crate::System::Xml::Serialization::ClassMap,
        ob: *mut crate::System::Object,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMessage(
        &mut self,
        membersMap: *mut crate::System::Xml::Serialization::XmlMembersMapping,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMessage", (membersMap, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObject(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
        element: *mut crate::System::String,
        namesp: *mut crate::System::String,
        isNullable: bool,
        needType: bool,
        writeWrappingElem: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteObject",
                (typeMap, ob, element, namesp, isNullable, needType, writeWrappingElem),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectElement(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
        element: *mut crate::System::String,
        namesp: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectElementAttributes(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElementAttributes", (typeMap, ob))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectElementElements(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElementElements", (typeMap, ob))?;
        Ok(__cordl_ret)
    }
    pub fn WritePrimitiveElement(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        ob: *mut crate::System::Object,
        element: *mut crate::System::String,
        namesp: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePrimitiveElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret)
    }
    pub fn WritePrimitiveValueEncoded(
        &mut self,
        memberValue: *mut crate::System::Object,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
        mappedType: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        wrapped: bool,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WritePrimitiveValueEncoded",
                (
                    memberValue,
                    name,
                    ns,
                    xsiType,
                    mappedType,
                    typeData,
                    wrapped,
                    isNullable,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WritePrimitiveValueLiteral(
        &mut self,
        memberValue: *mut crate::System::Object,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        mappedType: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        wrapped: bool,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WritePrimitiveValueLiteral",
                (memberValue, name, ns, mappedType, typeData, wrapped, isNullable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteRoot(
        &mut self,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRoot", (ob))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        typeMap: *mut crate::System::Xml::Serialization::XmlMapping,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeMap))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
