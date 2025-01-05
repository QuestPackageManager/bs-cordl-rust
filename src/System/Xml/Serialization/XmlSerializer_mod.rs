#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub customSerializer: bool,
    pub typeMapping: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlMapping,
    >,
    pub serializerData: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlSerializer_SerializerData,
    >,
    pub onUnreferencedObject: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::UnreferencedObjectEventHandler,
    >,
    pub onUnknownAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlAttributeEventHandler,
    >,
    pub onUnknownElement: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlElementEventHandler,
    >,
    pub onUnknownNode: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlNodeEventHandler,
    >,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlSerializer =>
    "System.Xml.Serialization"."XmlSerializer"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl crate::System::Xml::Serialization::XmlSerializer {
    #[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
    pub type SerializerData = crate::System::Xml::Serialization::XmlSerializer_SerializerData;
    pub fn CreateReader_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReader,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReader,
        > = __cordl_object.invoke("CreateReader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateReader_XmlMapping1(
        &mut self,
        typeMapping: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReader,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReader,
        > = __cordl_object.invoke("CreateReader", (typeMapping))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWriter_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        > = __cordl_object.invoke("CreateWriter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWriter_XmlMapping1(
        &mut self,
        typeMapping: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        > = __cordl_object.invoke("CreateWriter", (typeMapping))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_TextReader0(
        &mut self,
        textReader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Deserialize", (textReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_XmlReader1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Deserialize", (xmlReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_XmlSerializationReader2(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationReader,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        overrides: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeOverrides,
        >,
        extraTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        defaultNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, overrides, extraTypes, root, defaultNamespace),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OnUnknownAttribute(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownAttribute", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUnknownElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlElementEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownElement", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUnknownNode(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlNodeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownNode", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUnreferencedObject(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::UnreferencedObjectEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnreferencedObject", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_Il2CppObject_XmlSerializationWriter0(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (o, writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_TextWriter_Il2CppObject1(
        &mut self,
        textWriter: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (textWriter, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_XmlWriter_Il2CppObject2(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (xmlWriter, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_XmlWriter_Il2CppObject_XmlSerializerNamespaces3(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        namespaces: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializerNamespaces,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (xmlWriter, o, namespaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        overrides: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeOverrides,
        >,
        extraTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        defaultNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, overrides, extraTypes, root, defaultNamespace),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlMapping>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMapping,
        > = __cordl_object.invoke("get_Mapping", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializer_SerializerData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ReaderMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub WriterType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub WriterMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub Implementation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlSerializerImplementation,
    >,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializer_SerializerData =>
    "System.Xml.Serialization"."XmlSerializer/SerializerData"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    pub fn CreateWriter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        > = __cordl_object.invoke("CreateWriter", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
