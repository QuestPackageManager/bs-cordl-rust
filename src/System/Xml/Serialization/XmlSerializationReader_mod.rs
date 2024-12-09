#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReader {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationGeneratedCode,
    pub document: *mut crate::System::Xml::XmlDocument,
    pub reader: *mut crate::System::Xml::XmlReader,
    pub fixups: *mut crate::System::Collections::ArrayList,
    pub collFixups: *mut crate::System::Collections::Hashtable,
    pub collItemFixups: *mut crate::System::Collections::ArrayList,
    pub typesCallbacks: *mut crate::System::Collections::Hashtable,
    pub noIDTargets: *mut crate::System::Collections::ArrayList,
    pub targets: *mut crate::System::Collections::Hashtable,
    pub delayedListFixups: *mut crate::System::Collections::Hashtable,
    pub eventSource: *mut crate::System::Xml::Serialization::XmlSerializer,
    pub delayedFixupId: i32,
    pub referencedObjects: *mut crate::System::Collections::Hashtable,
    pub readCount: i32,
    pub whileIterationCount: i32,
    pub w3SchemaNS: *mut crate::System::String,
    pub w3InstanceNS: *mut crate::System::String,
    pub w3InstanceNS2000: *mut crate::System::String,
    pub w3InstanceNS1999: *mut crate::System::String,
    pub soapNS: *mut crate::System::String,
    pub wsdlNS: *mut crate::System::String,
    pub nullX: *mut crate::System::String,
    pub nil: *mut crate::System::String,
    pub typeX: *mut crate::System::String,
    pub arrayType: *mut crate::System::String,
    pub arrayQName: *mut crate::System::Xml::XmlQualifiedName,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReader => "System.Xml.Serialization"
    ."XmlSerializationReader"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializationReader {
    type Target = crate::System::Xml::Serialization::XmlSerializationGeneratedCode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializationReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
impl crate::System::Xml::Serialization::XmlSerializationReader {
    #[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
    pub type CollectionFixup = crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup;
    #[cfg(
        feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup"
    )]
    pub type CollectionItemFixup = crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup;
    #[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
    pub type Fixup = crate::System::Xml::Serialization::XmlSerializationReader_Fixup;
    #[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
    pub type WriteCallbackInfo = crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo;
    pub fn AddFixup_XmlSerializationReader_CollectionFixup0(
        &mut self,
        fixup: *mut crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixup", (fixup))?;
        Ok(__cordl_ret)
    }
    pub fn AddFixup_XmlSerializationReader_CollectionItemFixup2(
        &mut self,
        fixup: *mut crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixup", (fixup))?;
        Ok(__cordl_ret)
    }
    pub fn AddFixup_XmlSerializationReader_Fixup1(
        &mut self,
        fixup: *mut crate::System::Xml::Serialization::XmlSerializationReader_Fixup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixup", (fixup))?;
        Ok(__cordl_ret)
    }
    pub fn AddReadCallback(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        read: *mut crate::System::Xml::Serialization::XmlSerializationReadCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReadCallback", (name, ns, _cordl_type, read))?;
        Ok(__cordl_ret)
    }
    pub fn AddTarget(
        &mut self,
        id: *mut crate::System::String,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTarget", (id, o))?;
        Ok(__cordl_ret)
    }
    pub fn CreateReadOnlyCollectionException(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateReadOnlyCollectionException", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownConstantException(
        &mut self,
        value: *mut crate::System::String,
        enumType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownConstantException", (value, enumType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownNodeException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownNodeException", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownTypeException(
        &mut self,
        _cordl_type: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownTypeException", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CurrentTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("CurrentTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureArrayIndex(
        &mut self,
        a: *mut crate::System::Array,
        index: i32,
        elementType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Array> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Array = __cordl_object
            .invoke("EnsureArrayIndex", (a, index, elementType))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureArrayList(
        &mut self,
        list: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("EnsureArrayList", (list))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureHashtable(
        &mut self,
        hash: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("EnsureHashtable", (hash))?;
        Ok(__cordl_ret)
    }
    pub fn GetCallbackInfo(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo = __cordl_object
            .invoke("GetCallbackInfo", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn GetNullAttr(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNullAttr", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTarget(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetTarget", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetXsiType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("GetXsiType", ())?;
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
    pub fn InitIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIDs", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        eventSource: *mut crate::System::Xml::Serialization::XmlSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (reader, eventSource))?;
        Ok(__cordl_ret)
    }
    pub fn IsXmlnsAttribute(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXmlnsAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnUnknownNode(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        o: *mut crate::System::Object,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownNode", (node, o, qnames))?;
        Ok(__cordl_ret)
    }
    pub fn ParseWsdlArrayType(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseWsdlArrayType", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn ReadElementQualifiedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("ReadElementQualifiedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadList(
        &mut self,
        resultList: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadList", (resultList))?;
        Ok(__cordl_ret)
    }
    pub fn ReadNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadNullableQualifiedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("ReadNullableQualifiedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadNullableString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadNullableString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencedElement_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadReferencedElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencedElement_String_String1(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadReferencedElement", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencedElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadReferencedElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencingElement_ByRefMut0(
        &mut self,
        fixupReference: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadReferencingElement", (fixupReference))?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencingElement_String_String_ByRefMut1(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        fixupReference: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadReferencingElement", (name, ns, fixupReference))?;
        Ok(__cordl_ret)
    }
    pub fn ReadReferencingElement_String_String__cordl_bool_ByRefMut2(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        elementCanBeType: bool,
        fixupReference: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "ReadReferencingElement",
                (name, ns, elementCanBeType, fixupReference),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadSerializable(
        &mut self,
        serializable: *mut crate::System::Xml::Serialization::IXmlSerializable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::IXmlSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::IXmlSerializable = __cordl_object
            .invoke("ReadSerializable", (serializable))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTypedPrimitive_XmlQualifiedName0(
        &mut self,
        _cordl_type: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadTypedPrimitive", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTypedPrimitive__cordl_bool1(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
        reportUnknown: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadTypedPrimitive", (qname, reportUnknown))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXmlDocument(
        &mut self,
        wrapped: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocument> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocument = __cordl_object
            .invoke("ReadXmlDocument", (wrapped))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXmlNode(
        &mut self,
        wrapped: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("ReadXmlNode", (wrapped))?;
        Ok(__cordl_ret)
    }
    pub fn ShrinkArray(
        &mut self,
        a: *mut crate::System::Array,
        length: i32,
        elementType: *mut crate::System::Type,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Array> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Array = __cordl_object
            .invoke("ShrinkArray", (a, length, elementType, isNullable))?;
        Ok(__cordl_ret)
    }
    pub fn TargetReady(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TargetReady", (id))?;
        Ok(__cordl_ret)
    }
    pub fn ToXmlQualifiedName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("ToXmlQualifiedName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UnknownAttribute(
        &mut self,
        o: *mut crate::System::Object,
        attr: *mut crate::System::Xml::XmlAttribute,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnknownAttribute", (o, attr, qnames))?;
        Ok(__cordl_ret)
    }
    pub fn UnknownElement(
        &mut self,
        o: *mut crate::System::Object,
        elem: *mut crate::System::Xml::XmlElement,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnknownElement", (o, elem, qnames))?;
        Ok(__cordl_ret)
    }
    pub fn UnknownNode_Object0(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnknownNode", (o))?;
        Ok(__cordl_ret)
    }
    pub fn UnknownNode_String1(
        &mut self,
        o: *mut crate::System::Object,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnknownNode", (o, qnames))?;
        Ok(__cordl_ret)
    }
    pub fn UnreferencedObject(
        &mut self,
        id: *mut crate::System::String,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnreferencedObject", (id, o))?;
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
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocument> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocument = __cordl_object
            .invoke("get_Document", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlReader = __cordl_object
            .invoke("get_Reader", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReader_CollectionFixup {
    __cordl_parent: crate::System::Object,
    pub callback: *mut crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback,
    pub collection: *mut crate::System::Object,
    pub collectionItems: *mut crate::System::Object,
    pub id: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReader_CollectionFixup =>
    "System.Xml.Serialization"."XmlSerializationReader/CollectionFixup"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
impl crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup {
    pub fn New(
        collection: *mut crate::System::Object,
        callback: *mut crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection, callback, id))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        collection: *mut crate::System::Object,
        callback: *mut crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection, callback, id))?;
        Ok(__cordl_ret)
    }
    pub fn get_Callback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback = __cordl_object
            .invoke("get_Callback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Collection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Collection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CollectionItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_CollectionItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CollectionItems(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CollectionItems", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionFixup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionFixup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReader_CollectionItemFixup {
    __cordl_parent: crate::System::Object,
    pub list: *mut crate::System::Array,
    pub index: i32,
    pub id: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup =>
    "System.Xml.Serialization"."XmlSerializationReader/CollectionItemFixup"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
impl crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup {
    pub fn New(
        list: *mut crate::System::Array,
        index: i32,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list, index, id))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        list: *mut crate::System::Array,
        index: i32,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list, index, id))?;
        Ok(__cordl_ret)
    }
    pub fn get_Collection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Array> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Array = __cordl_object
            .invoke("get_Collection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Index", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+CollectionItemFixup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReader_CollectionItemFixup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReader_Fixup {
    __cordl_parent: crate::System::Object,
    pub source: *mut crate::System::Object,
    pub ids: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub callback: *mut crate::System::Xml::Serialization::XmlSerializationFixupCallback,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReader_Fixup =>
    "System.Xml.Serialization"."XmlSerializationReader/Fixup"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReader_Fixup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReader_Fixup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
impl crate::System::Xml::Serialization::XmlSerializationReader_Fixup {
    pub fn New(
        o: *mut crate::System::Object,
        callback: *mut crate::System::Xml::Serialization::XmlSerializationFixupCallback,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o, callback, count))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        o: *mut crate::System::Object,
        callback: *mut crate::System::Xml::Serialization::XmlSerializationFixupCallback,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o, callback, count))?;
        Ok(__cordl_ret)
    }
    pub fn get_Callback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationFixupCallback,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationFixupCallback = __cordl_object
            .invoke("get_Callback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Ids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Ids", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Source(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Source", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+Fixup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReader_Fixup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationReader_WriteCallbackInfo {
    __cordl_parent: crate::System::Object,
    pub Type: *mut crate::System::Type,
    pub TypeName: *mut crate::System::String,
    pub TypeNs: *mut crate::System::String,
    pub Callback: *mut crate::System::Xml::Serialization::XmlSerializationReadCallback,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo =>
    "System.Xml.Serialization"."XmlSerializationReader/WriteCallbackInfo"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
impl crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "System+Xml+Serialization+XmlSerializationReader+WriteCallbackInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationReader_WriteCallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
