#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeValueCache_XmlWellFormedWriter_BufferChunk {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub index: i32,
    pub count: i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk => "System.Xml"
    ."XmlWellFormedWriter/AttributeValueCache/BufferChunk"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
impl std::ops::Deref
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
impl std::ops::DerefMut
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
impl crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk {
    pub fn New(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (buffer, index, count))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeValueCache_XmlWellFormedWriter_Item {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_ItemType,
    pub data: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item => "System.Xml"
    ."XmlWellFormedWriter/AttributeValueCache/Item"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
impl std::ops::Deref
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
impl std::ops::DerefMut
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
impl crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Set(
        &mut self,
        _cordl_type: crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_ItemType,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (_cordl_type, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+ItemType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeValueCache_XmlWellFormedWriter_ItemType {
    CharEntity = 1i32,
    EntityRef = 0i32,
    Raw = 6i32,
    RawChars = 7i32,
    String = 4i32,
    StringChars = 5i32,
    SurrogateCharEntity = 2i32,
    ValueString = 8i32,
    Whitespace = 3i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+ItemType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::AttributeValueCache_XmlWellFormedWriter_ItemType => "System.Xml"
    ."XmlWellFormedWriter/AttributeValueCache/ItemType"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlWellFormedWriter {
    __cordl_parent: crate::System::Xml::XmlWriter,
    pub writer: *mut crate::System::Xml::XmlWriter,
    pub rawWriter: *mut crate::System::Xml::XmlRawWriter,
    pub predefinedNamespaces: *mut crate::System::Xml::IXmlNamespaceResolver,
    pub nsStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlWellFormedWriter_Namespace,
    >,
    pub nsTop: i32,
    pub nsHashtable: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i32,
    >,
    pub useNsHashtable: bool,
    pub elemScopeStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlWellFormedWriter_ElementScope,
    >,
    pub elemTop: i32,
    pub attrStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlWellFormedWriter_AttrName,
    >,
    pub attrCount: i32,
    pub attrHashTable: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i32,
    >,
    pub specAttr: crate::System::Xml::XmlWellFormedWriter_SpecialAttribute,
    pub attrValueCache: *mut crate::System::Xml::XmlWellFormedWriter_AttributeValueCache,
    pub curDeclPrefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub stateTable: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlWellFormedWriter_State,
    >,
    pub currentState: crate::System::Xml::XmlWellFormedWriter_State,
    pub checkCharacters: bool,
    pub omitDuplNamespaces: bool,
    pub writeEndDocumentOnClose: bool,
    pub conformanceLevel: crate::System::Xml::ConformanceLevel,
    pub dtdWritten: bool,
    pub xmlDeclFollows: bool,
    pub xmlCharType: crate::System::Xml::XmlCharType,
    pub hasher: *mut crate::System::Xml::SecureStringHasher,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter => "System.Xml"
    ."XmlWellFormedWriter"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
impl std::ops::Deref for crate::System::Xml::XmlWellFormedWriter {
    type Target = crate::System::Xml::XmlWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
impl std::ops::DerefMut for crate::System::Xml::XmlWellFormedWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
impl crate::System::Xml::XmlWellFormedWriter {
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+AttrName")]
    pub type AttrName = crate::System::Xml::XmlWellFormedWriter_AttrName;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
    pub type AttributeValueCache = crate::System::Xml::XmlWellFormedWriter_AttributeValueCache;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+ElementScope")]
    pub type ElementScope = crate::System::Xml::XmlWellFormedWriter_ElementScope;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+Namespace")]
    pub type Namespace = crate::System::Xml::XmlWellFormedWriter_Namespace;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceKind")]
    pub type NamespaceKind = crate::System::Xml::XmlWellFormedWriter_NamespaceKind;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
    pub type NamespaceResolverProxy = crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+SpecialAttribute")]
    pub type SpecialAttribute = crate::System::Xml::XmlWellFormedWriter_SpecialAttribute;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+State")]
    pub type State = crate::System::Xml::XmlWellFormedWriter_State;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+Token")]
    pub type Token = crate::System::Xml::XmlWellFormedWriter_Token;
    pub fn AddAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (prefix, localName, namespaceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kind: crate::System::Xml::XmlWellFormedWriter_NamespaceKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespace", (prefix, ns, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToAttrHashTable(
        &mut self,
        attributeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToAttrHashTable", (attributeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToNamespaceHashtable(
        &mut self,
        namespaceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToNamespaceHashtable", (namespaceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdvanceState(
        &mut self,
        token: crate::System::Xml::XmlWellFormedWriter_Token,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdvanceState", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNCName(
        &mut self,
        ncname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNCName", (ncname))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GeneratePrefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupLocalNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupLocalNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupNamespaceIndex(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LookupNamespaceIndex", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupPrefix(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupPrefix", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        settings: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriterSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer, settings))?;
        Ok(__cordl_object.into())
    }
    pub fn PopNamespaces(
        &mut self,
        indexFrom: i32,
        indexTo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopNamespaces", (indexFrom, indexTo))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushNamespaceExplicit(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PushNamespaceExplicit", (prefix, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushNamespaceImplicit(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushNamespaceImplicit", (prefix, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpecialAttribute(
        &mut self,
        special: crate::System::Xml::XmlWellFormedWriter_SpecialAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSpecialAttribute", (special))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartElementContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartElementContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartFragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartFragment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidStateTransition(
        &mut self,
        token: crate::System::Xml::XmlWellFormedWriter_Token,
        currentState: crate::System::Xml::XmlWellFormedWriter_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowInvalidStateTransition", (token, currentState))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteBase64(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBase64", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteBinHex(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBinHex", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCData(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCData", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCharEntity(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCharEntity", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteChars(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteComment(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteComment", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDocType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDocType", (name, pubid, sysid, subset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndDocument(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndDocument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntityRef(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEntityRef", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFullEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteFullEndElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteProcessingInstruction(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteProcessingInstruction", (name, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppString1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartAttribute", (prefix, localName, namespaceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartDocumentImpl(
        &mut self,
        standalone: crate::System::Xml::XmlStandalone,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocumentImpl", (standalone))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartDocument_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartDocument__cordl_bool1(
        &mut self,
        standalone: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocument", (standalone))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (prefix, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteString(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSurrogateCharEntity(
        &mut self,
        lowChar: char,
        highChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSurrogateCharEntity", (lowChar, highChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWhitespace(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteWhitespace", (ws))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        settings: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriterSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InBase64(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InBase64", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsClosedOrErrorState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsClosedOrErrorState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RawWriter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter> = __cordl_object
            .invoke("get_RawWriter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SaveAttrValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SaveAttrValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::WriteState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::WriteState = __cordl_object
            .invoke("get_WriteState", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlWellFormedWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttrName")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlWellFormedWriter_AttrName {
    pub prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub namespaceUri: *mut quest_hook::libil2cpp::Il2CppString,
    pub localName: *mut quest_hook::libil2cpp::Il2CppString,
    pub prev: i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttrName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_AttrName =>
    "System.Xml"."XmlWellFormedWriter/AttrName"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttrName")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlWellFormedWriter_AttrName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttrName")]
impl crate::System::Xml::XmlWellFormedWriter_AttrName {
    pub fn IsDuplicate(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsDuplicate",
            (prefix, localName, namespaceUri),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, localName, namespaceUri),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlWellFormedWriter_AttributeValueCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stringValue: *mut crate::System::Text::StringBuilder,
    pub singleStringValue: *mut quest_hook::libil2cpp::Il2CppString,
    pub items: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item,
    >,
    pub firstItem: i32,
    pub lastItem: i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlWellFormedWriter_AttributeValueCache => "System.Xml"
    ."XmlWellFormedWriter/AttributeValueCache"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
impl std::ops::Deref for crate::System::Xml::XmlWellFormedWriter_AttributeValueCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
impl std::ops::DerefMut for crate::System::Xml::XmlWellFormedWriter_AttributeValueCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
impl crate::System::Xml::XmlWellFormedWriter_AttributeValueCache {
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+BufferChunk")]
    pub type BufferChunk = crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_BufferChunk;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+Item")]
    pub type Item = crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_Item;
    #[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache+ItemType")]
    pub type ItemType = crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_ItemType;
    pub fn AddItem(
        &mut self,
        _cordl_type: crate::System::Xml::AttributeValueCache_XmlWellFormedWriter_ItemType,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (_cordl_type, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Replay(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartComplexValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartComplexValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Trim", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCharEntity(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCharEntity", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteChars(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntityRef(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEntityRef", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppString1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteString(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSurrogateCharEntity(
        &mut self,
        lowChar: char,
        highChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSurrogateCharEntity", (lowChar, highChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWhitespace(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteWhitespace", (ws))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StringValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_StringValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+AttributeValueCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlWellFormedWriter_AttributeValueCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+ElementScope")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlWellFormedWriter_ElementScope {
    pub prevNSTop: i32,
    pub prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub localName: *mut quest_hook::libil2cpp::Il2CppString,
    pub namespaceUri: *mut quest_hook::libil2cpp::Il2CppString,
    pub xmlSpace: crate::System::Xml::XmlSpace,
    pub xmlLang: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+ElementScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_ElementScope =>
    "System.Xml"."XmlWellFormedWriter/ElementScope"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+ElementScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlWellFormedWriter_ElementScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+ElementScope")]
impl crate::System::Xml::XmlWellFormedWriter_ElementScope {
    pub fn Set(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prevNSTop: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, localName, namespaceUri, prevNSTop),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndElement(
        &mut self,
        rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteEndElement",
            (rawWriter),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFullEndElement(
        &mut self,
        rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteFullEndElement",
            (rawWriter),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Namespace")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlWellFormedWriter_Namespace {
    pub prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub namespaceUri: *mut quest_hook::libil2cpp::Il2CppString,
    pub kind: crate::System::Xml::XmlWellFormedWriter_NamespaceKind,
    pub prevNsIndex: i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Namespace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_Namespace =>
    "System.Xml"."XmlWellFormedWriter/Namespace"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Namespace")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlWellFormedWriter_Namespace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Namespace")]
impl crate::System::Xml::XmlWellFormedWriter_Namespace {
    pub fn Set(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kind: crate::System::Xml::XmlWellFormedWriter_NamespaceKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, namespaceUri, kind),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDecl(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteDecl",
            (writer, rawWriter),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlWellFormedWriter_NamespaceKind {
    Implied = 2i32,
    NeedToWrite = 1i32,
    Special = 3i32,
    Written = 0i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_NamespaceKind
    => "System.Xml"."XmlWellFormedWriter/NamespaceKind"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlWellFormedWriter_NamespaceResolverProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub wfWriter: *mut crate::System::Xml::XmlWellFormedWriter,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy => "System.Xml"
    ."XmlWellFormedWriter/NamespaceResolverProxy"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl std::ops::Deref for crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl std::ops::DerefMut
for crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    pub fn New(
        wfWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWellFormedWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wfWriter))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupPrefix(
        &mut self,
        namespaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        wfWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWellFormedWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (wfWriter))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl AsRef<crate::System::Xml::IXmlNamespaceResolver>
for crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    fn as_ref(&self) -> &crate::System::Xml::IXmlNamespaceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+NamespaceResolverProxy")]
impl AsMut<crate::System::Xml::IXmlNamespaceResolver>
for crate::System::Xml::XmlWellFormedWriter_NamespaceResolverProxy {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IXmlNamespaceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+SpecialAttribute")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlWellFormedWriter_SpecialAttribute {
    DefaultXmlns = 1i32,
    No = 0i32,
    PrefixedXmlns = 2i32,
    XmlLang = 4i32,
    XmlSpace = 3i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+SpecialAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlWellFormedWriter_SpecialAttribute => "System.Xml"
    ."XmlWellFormedWriter/SpecialAttribute"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlWellFormedWriter_State {
    AfterRootEle = 7i32,
    AfterRootLevelAttr = 14i32,
    Attribute = 8i32,
    B64Attribute = 6i32,
    B64Content = 5i32,
    Closed = 15i32,
    Content = 4i32,
    Document = 2i32,
    Element = 3i32,
    EndAttrEEle = 108i32,
    EndAttrSAttr = 111i32,
    EndAttrSCont = 109i32,
    EndAttrSEle = 107i32,
    EndDocument = 10i32,
    Error = 16i32,
    PostB64Attr = 113i32,
    PostB64Cont = 112i32,
    PostB64RootAttr = 114i32,
    RootLevelAttr = 11i32,
    RootLevelB64Attr = 13i32,
    RootLevelSpecAttr = 12i32,
    SpecialAttr = 9i32,
    Start = 0i32,
    StartContent = 101i32,
    StartContentB64 = 103i32,
    StartContentEle = 102i32,
    StartDoc = 104i32,
    StartDocEle = 106i32,
    StartFragB64 = 117i32,
    StartFragCont = 116i32,
    StartFragEle = 115i32,
    StartRootLevelAttr = 118i32,
    TopLevel = 1i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_State =>
    "System.Xml"."XmlWellFormedWriter/State"
);
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Token")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlWellFormedWriter_Token {
    AtomicValue = 11i32,
    Base64 = 12i32,
    CData = 10i32,
    Comment = 3i32,
    Dtd = 4i32,
    EndAttribute = 8i32,
    EndDocument = 1i32,
    EndElement = 6i32,
    PI = 2i32,
    RawData = 13i32,
    StartAttribute = 7i32,
    StartDocument = 0i32,
    StartElement = 5i32,
    Text = 9i32,
    Whitespace = 14i32,
}
#[cfg(feature = "System+Xml+XmlWellFormedWriter+Token")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWellFormedWriter_Token =>
    "System.Xml"."XmlWellFormedWriter/Token"
);
