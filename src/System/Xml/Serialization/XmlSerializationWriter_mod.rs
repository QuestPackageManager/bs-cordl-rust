#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriter_WriteCallbackInfo {
    __cordl_parent: crate::System::Object,
    pub Type: *mut crate::System::Type,
    pub TypeName: *mut crate::System::String,
    pub TypeNs: *mut crate::System::String,
    pub Callback: *mut crate::System::Xml::Serialization::XmlSerializationWriteCallback,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo =>
    "System.Xml.Serialization"."XmlSerializationWriter/WriteCallbackInfo"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
impl crate::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo {
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
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriter {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationGeneratedCode,
    pub idGenerator: *mut crate::System::Runtime::Serialization::ObjectIDGenerator,
    pub qnameCount: i32,
    pub topLevelElement: bool,
    pub namespaces: *mut crate::System::Collections::ArrayList,
    pub writer: *mut crate::System::Xml::XmlWriter,
    pub referencedElements: *mut crate::System::Collections::Queue,
    pub callbacks: *mut crate::System::Collections::Hashtable,
    pub serializedObjects: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationWriter => "System.Xml.Serialization"
    ."XmlSerializationWriter"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializationWriter {
    type Target = crate::System::Xml::Serialization::XmlSerializationGeneratedCode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializationWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
impl crate::System::Xml::Serialization::XmlSerializationWriter {
    #[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter+WriteCallbackInfo")]
    pub type WriteCallbackInfo = crate::System::Xml::Serialization::XmlSerializationWriter_WriteCallbackInfo;
    pub fn AddWriteCallback(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        typeName: *mut crate::System::String,
        typeNs: *mut crate::System::String,
        callback: *mut crate::System::Xml::Serialization::XmlSerializationWriteCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWriteCallback", (_cordl_type, typeName, typeNs, callback))?;
        Ok(__cordl_ret)
    }
    pub fn AlreadyQueued(
        &mut self,
        ob: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AlreadyQueued", (ob))?;
        Ok(__cordl_ret)
    }
    pub fn CheckReferenceQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckReferenceQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownAnyElementException(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownAnyElementException", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownTypeException_Object0(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownTypeException", (o))?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnknownTypeException_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateUnknownTypeException", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn FromXmlQualifiedName(
        &mut self,
        xmlQualifiedName: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FromXmlQualifiedName", (xmlQualifiedName))?;
        Ok(__cordl_ret)
    }
    pub fn GetId(
        &mut self,
        o: *mut crate::System::Object,
        addToReferencesList: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetId", (o, addToReferencesList))?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespacePrefix(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNamespacePrefix", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetQualifiedName(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetQualifiedName", (name, ns))?;
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
    pub fn Initialize(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
        nss: *mut crate::System::Xml::Serialization::XmlSerializerNamespaces,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (writer, nss))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrimitiveArray(
        &mut self,
        td: *mut crate::System::Xml::Serialization::TypeData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrimitiveArray", (td))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TopLevelElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TopLevelElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        o: *mut crate::System::Object,
        td: *mut crate::System::Xml::Serialization::TypeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (o, td))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAttribute_String1(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAttribute", (prefix, localName, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAttribute_String_String_String0(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAttribute", (localName, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementEncoded(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        isNullable: bool,
        any: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementEncoded", (node, name, ns, isNullable, any))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementLiteral(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        isNullable: bool,
        any: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementLiteral", (node, name, ns, isNullable, any))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementQualifiedName_String_String_XmlQualifiedName0(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementQualifiedName", (localName, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementQualifiedName_XmlQualifiedName1(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::Xml::XmlQualifiedName,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementQualifiedName", (localName, ns, value, xsiType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementString_String_String_String0(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementString", (localName, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteElementString_XmlQualifiedName1(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteElementString", (localName, ns, value, xsiType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndElement_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndElement_Object1(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", (o))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNamespaceDeclarations(
        &mut self,
        xmlns: *mut crate::System::Xml::Serialization::XmlSerializerNamespaces,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNamespaceDeclarations", (xmlns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullTagEncoded(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullTagEncoded", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullTagLiteral(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullTagLiteral", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullableQualifiedNameEncoded(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::Xml::XmlQualifiedName,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullableQualifiedNameEncoded", (name, ns, value, xsiType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullableQualifiedNameLiteral(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullableQualifiedNameLiteral", (name, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullableStringEncoded(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullableStringEncoded", (name, ns, value, xsiType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullableStringLiteral(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullableStringLiteral", (name, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn WritePotentiallyReferencingElement(
        &mut self,
        n: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
        ambientType: *mut crate::System::Type,
        suppressReference: bool,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WritePotentiallyReferencingElement",
                (n, ns, o, ambientType, suppressReference, isNullable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteReferencedElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReferencedElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteReferencingElement(
        &mut self,
        n: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReferencingElement", (n, ns, o, isNullable))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSerializable_IXmlSerializable_String_String__cordl_bool0(
        &mut self,
        serializable: *mut crate::System::Xml::Serialization::IXmlSerializable,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSerializable", (serializable, name, ns, isNullable))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSerializable__cordl_bool1(
        &mut self,
        serializable: *mut crate::System::Xml::Serialization::IXmlSerializable,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        isNullable: bool,
        wrapped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSerializable", (serializable, name, ns, isNullable, wrapped))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartDocument(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocument", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement_Object2(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (name, ns, o))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement_Object__cordl_bool3(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
        writePrefixed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (name, ns, o, writePrefixed))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement_Object__cordl_bool_ICollection4(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
        writePrefixed: bool,
        namespaces: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (name, ns, o, writePrefixed, namespaces))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement_String_String0(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement__cordl_bool1(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        writePrefixed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (name, ns, writePrefixed))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTypedPrimitive(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        o: *mut crate::System::Object,
        xsiType: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTypedPrimitive", (name, ns, o, xsiType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteValue(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteXmlAttribute(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        container: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlAttribute", (node, container))?;
        Ok(__cordl_ret)
    }
    pub fn WriteXmlNode(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn WriteXsiType(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXsiType", (name, ns))?;
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
    pub fn get_Writer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlWriter = __cordl_object
            .invoke("get_Writer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
