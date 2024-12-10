#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReaderInterpreter {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationReader,
    pub _typeMap: *mut crate::System::Xml::Serialization::XmlMapping,
    pub _format: crate::System::Xml::Serialization::SerializationFormat,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReaderInterpreter =>
    "System.Xml.Serialization"."XmlSerializationReaderInterpreter"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter {
    type Target = crate::System::Xml::Serialization::XmlSerializationReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
impl crate::System::Xml::Serialization::XmlSerializationReaderInterpreter {
    #[cfg(
        feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
    )]
    pub type FixupCallbackInfo = crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo;
    #[cfg(
        feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
    )]
    pub type ReaderCallbackInfo = crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo;
    pub fn AddListValue(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        list: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        canCreateInstance: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddListValue", (listType, list, index, value, canCreateInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyEnumerableList(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyEnumerableList", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("CreateInstance", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateList(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("CreateList", (listType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillList(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        items: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillList", (list, items))?;
        Ok(__cordl_ret.into())
    }
    pub fn FixupMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        obfixup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixupMembers", (map, obfixup, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumValue(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEnumValue", (typeMap, val))?;
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
    pub fn GetValueFromXmlString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValueFromXmlString", (value, typeData, typeMap))?;
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
    pub fn InitIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIDs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeList(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("InitializeList", (listType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsReadOnly(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        memType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsReadOnly", (member, memType, ob, isValueList))?;
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
    pub fn ProcessUnknownAttribute(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessUnknownAttribute", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessUnknownElement(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessUnknownElement", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttributeMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAttributeMembers", (map, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadClassInstance(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
        checkType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ReadClassInstance", (typeMap, isNullable, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadClassInstanceMembers(
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
            .invoke("ReadClassInstanceMembers", (typeMap, ob))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadEncodedObject(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadEncodedObject", (typeMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadEnumElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadEnumElement", (typeMap, isNullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadListElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        canCreateInstance: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ReadListElement", (typeMap, isNullable, list, canCreateInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadListString(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadListString", (typeMap, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
        readBySoapOrder: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadMembers", (map, ob, isValueList, readBySoapOrder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMessage(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMembersMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadMessage", (typeMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
        checkType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadObject", (typeMap, isNullable, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadObjectElement(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadObjectElement", (elem))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPrimitiveElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadPrimitiveElement", (typeMap, isNullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPrimitiveValue(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadPrimitiveValue", (elem))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRoot_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRoot_XmlTypeMapping1(
        &mut self,
        rootMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadRoot", (rootMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlNode(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        >,
        wrapped: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadXmlNode", (_cordl_type, wrapped))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlNodeElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadXmlNodeElement", (typeMap, isNullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlSerializableElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadXmlSerializableElement", (typeMap, isNullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetListMembersDefaults(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetListMembersDefaults", (map, ob, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMemberValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMemberValue", (member, ob, value, isValueList))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMemberValueFromAttr(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMemberValueFromAttr", (member, ob, value, isValueList))?;
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
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReaderInterpreter_FixupCallbackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sri: *mut crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
    pub _map: *mut crate::System::Xml::Serialization::ClassMap,
    pub _isValueList: bool,
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo =>
    "System.Xml.Serialization"."XmlSerializationReaderInterpreter/FixupCallbackInfo"
);
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
impl crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo {
    pub fn FixupMembers(
        &mut self,
        fixup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixupMembers", (fixup))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sri: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
        >,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sri, map, isValueList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sri: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
        >,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sri, map, isValueList))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+FixupCallbackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_FixupCallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReaderInterpreter_ReaderCallbackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sri: *mut crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
    pub _typeMap: *mut crate::System::Xml::Serialization::XmlTypeMapping,
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo =>
    "System.Xml.Serialization"."XmlSerializationReaderInterpreter/ReaderCallbackInfo"
);
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
impl crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo {
    pub fn New(
        sri: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sri, typeMap))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sri: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReaderInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sri, typeMap))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationReaderInterpreter+ReaderCallbackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReaderInterpreter_ReaderCallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
