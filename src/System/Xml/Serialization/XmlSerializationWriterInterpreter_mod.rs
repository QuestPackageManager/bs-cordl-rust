#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationWriter,
    pub _typeMap: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlMapping,
    >,
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
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetEnumXmlValue", (typeMap, ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetListCount(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetListCount", (listType, ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetMemberValue", (member, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStringValue", (typeMap, _cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplicitConvert(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplicitConvert", (obj, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MemberHasValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MemberHasValue", (member, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        typeMap: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeMap))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteAnyElementContent(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
        >,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAnyElementContent", (member, memberValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAttributeMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAttributeMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteElementMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEnumElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnumElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteListContent(
        &mut self,
        container: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ListMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetString: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteListContent", (container, listType, map, ob, targetString))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteListElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteListElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteMemberElement(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMemberElement", (elem, memberValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteMessage(
        &mut self,
        membersMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMembersMapping,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMessage", (membersMap, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElementAttributes(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElementAttributes", (typeMap, ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElementElements(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectElementElements", (typeMap, ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePrimitiveElement", (typeMap, ob, element, namesp))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveValueEncoded(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xsiType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        mappedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveValueLiteral(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mappedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn WriteRoot(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRoot", (ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeMap))?;
        Ok(__cordl_ret.into())
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
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter_CallbackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _swi: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
    >,
    pub _typeMap: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlTypeMapping,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        swi: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (swi, typeMap))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteEnum(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnum", (ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        swi: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (swi, typeMap))?;
        Ok(__cordl_ret.into())
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
