#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContext"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerReadContext {
    __cordl_parent: crate::System::Runtime::Serialization::XmlObjectSerializerContext,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Attributes,
    >,
    pub deserializedObjects: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::HybridObjectCache,
    >,
    pub xmlSerializableReader: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlSerializableReader,
    >,
    pub xmlDocument: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    pub attributesInXmlData: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Attributes,
    >,
    pub extensionDataReader: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlReaderDelegator,
    >,
    pub getOnlyCollectionValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub isGetOnlyCollection: bool,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContext"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerReadContext";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContext")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContext {
    type Target = crate::System::Runtime::Serialization::XmlObjectSerializerContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContext")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContext")]
impl crate::System::Runtime::Serialization::XmlObjectSerializerReadContext {
    pub fn AddNamespaceDeclaration(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
                        2usize,
                    >("AddNamespaceDeclaration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddNamespaceDeclaration", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = unsafe {
            cordl_method_info.invoke_unchecked(self, (prefix, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddNewObject(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddNewObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddNewObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddNewObjectWithId(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddNewObjectWithId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddNewObjectWithId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (id, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckEndOfArray(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        arraySize: i32,
        itemName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        itemNamespace: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("CheckEndOfArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckEndOfArray", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, arraySize, itemName, itemNamespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateContext(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractSerializer,
        >,
        rootTypeDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerReadContext,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContractSerializer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContractResolver,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlObjectSerializerReadContext,
                        >,
                        3usize,
                    >("CreateContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateContext", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerReadContext,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (serializer, rootTypeDataContract, dataContractResolver),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateReaderDelegatorForReader(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >,
                        1usize,
                    >("CreateReaderDelegatorForReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateReaderDelegatorForReader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (xmlReader))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateReaderOverChildNodes(
        &mut self,
        xmlAttributes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
            >,
        >,
        xmlChildNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >,
                        2usize,
                    >("CreateReaderOverChildNodes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateReaderOverChildNodes", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlAttributes, xmlChildNodes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnexpectedStateException(
        expectedState: crate::System::Xml::XmlNodeType,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Xml::XmlNodeType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        2usize,
                    >("CreateUnexpectedStateException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUnexpectedStateException", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            cordl_method_info.invoke_unchecked((), (expectedState, xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWrapperXmlElement(
        document: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
        xmlAttributes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
            >,
        >,
        xmlChildNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
            >,
        >,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                        6usize,
                    >("CreateWrapperXmlElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateWrapperXmlElement", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (document, xmlAttributes, xmlChildNodes, prefix, localName, ns),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeFromExtensionData(
        &mut self,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IDataNode,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        4usize,
                    >("DeserializeFromExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DeserializeFromExtensionData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (dataNode, _cordl_type, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureArraySize<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("EnsureArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureArraySize", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (array, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetArraySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetArraySize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCollectionMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("GetCollectionMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCollectionMember", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExistingObject(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        4usize,
                    >("GetExistingObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExistingObject", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (id, _cordl_type, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetExistingObjectOrExtensionData(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetExistingObjectOrExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExistingObjectOrExtensionData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberIndex(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
            >,
        >,
        memberNamespaces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
            >,
        >,
        memberIndex: i32,
        extensionData: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataObject,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlDictionaryString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlDictionaryString,
                                    >,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                        ),
                        i32,
                        5usize,
                    >("GetMemberIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMemberIndex", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        xmlReader,
                        memberNames,
                        memberNamespaces,
                        memberIndex,
                        extensionData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberIndexWithRequiredMembers(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
            >,
        >,
        memberNamespaces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
            >,
        >,
        memberIndex: i32,
        requiredIndex: i32,
        extensionData: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataObject,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlDictionaryString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlDictionaryString,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                        ),
                        i32,
                        6usize,
                    >("GetMemberIndexWithRequiredMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMemberIndexWithRequiredMembers", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        xmlReader,
                        memberNames,
                        memberNamespaces,
                        memberIndex,
                        requiredIndex,
                        extensionData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("GetObjectId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetObjectId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRealObject(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IObjectReference,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IObjectReference,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("GetRealObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRealObject", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (obj, id))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMemberNotFound(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        extensionData: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataObject,
        >,
        memberIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandleMemberNotFound")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleMemberNotFound", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, extensionData, memberIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleUnknownElement(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        extensionData: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataObject,
        >,
        memberIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandleUnknownElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleUnknownElement", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, extensionData, memberIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeExtensionDataNode(
        &mut self,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IDataNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitializeExtensionDataNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeExtensionDataNode", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (dataNode, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserialize_Il2CppString_Il2CppString_Type_ByRefMut3(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dataContract: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Serialization::DataContract,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Runtime::Serialization::DataContract,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("InternalDeserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserialize", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (reader, name, ns, declaredType, dataContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserialize_Type_DataContract_Il2CppString_Il2CppString2(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("InternalDeserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserialize", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, declaredType, dataContract, name, ns),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserialize_Type_Il2CppString_Il2CppString1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        4usize,
                    >("InternalDeserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserialize", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, declaredType, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserialize_i32_RuntimeTypeHandle_Il2CppString_Il2CppString0(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        id: i32,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            i32,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("InternalDeserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserialize", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, id, declaredTypeHandle, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsContentNode(
        &mut self,
        nodeType: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Xml::XmlNodeType),
                        bool,
                        1usize,
                    >("IsContentNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsContentNode", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (nodeType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsReadingClassExtensionData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        bool,
                        1usize,
                    >("IsReadingClassExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsReadingClassExtensionData", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsReadingCollectionExtensionData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        bool,
                        1usize,
                    >("IsReadingCollectionExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsReadingCollectionExtensionData", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextElement(
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        bool,
                        1usize,
                    >("MoveToNextElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveToNextElement", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractSerializer,
        >,
        rootTypeDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (serializer, rootTypeDataContract, dataContractResolver),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseQualifiedName(
        qname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        ns: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ParseQualifiedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseQualifiedName", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (qname, xmlReader, name, ns, prefix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Read")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Read",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadAndResolveUnknownXmlData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        namespaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IDictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        4usize,
                    >("ReadAndResolveUnknownXmlData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadAndResolveUnknownXmlData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, namespaces, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttributes(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReadAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadAttributes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadDataContractValue(
        &mut self,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("ReadDataContractValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadDataContractValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (dataContract, reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadExtensionDataMember(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        memberIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataMember,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ExtensionDataMember,
                        >,
                        2usize,
                    >("ReadExtensionDataMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadExtensionDataMember", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataMember,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader, memberIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadExtensionDataValue_Il2CppString_Il2CppString1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        3usize,
                    >("ReadExtensionDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadExtensionDataValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadExtensionDataValue_XmlReaderDelegator0(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        1usize,
                    >("ReadExtensionDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadExtensionDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (xmlReader))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadIXmlSerializable_XmlReaderDelegator_XmlDataContract__cordl_bool0(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        xmlDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlDataContract,
        >,
        isMemberType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlDataContract,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("ReadIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadIXmlSerializable", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, xmlDataContract, isMemberType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadIXmlSerializable_XmlSerializableReader_XmlReaderDelegator_XmlDataContract__cordl_bool1(
        xmlSerializableReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlSerializableReader,
        >,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        xmlDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlDataContract,
        >,
        isMemberType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlSerializableReader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlDataContract,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        4usize,
                    >("ReadIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadIXmlSerializable", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (xmlSerializableReader, xmlReader, xmlDataContract, isMemberType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadIfNullOrRef(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isMemberTypeSerializable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("ReadIfNullOrRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadIfNullOrRef", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, memberType, isMemberTypeSerializable),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPrimitiveExtensionDataValue(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        3usize,
                    >("ReadPrimitiveExtensionDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadPrimitiveExtensionDataValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadRootIXmlSerializable(
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        xmlDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlDataContract,
        >,
        isMemberType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlDataContract,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("ReadRootIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadRootIXmlSerializable", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xmlReader, xmlDataContract, isMemberType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadSerializationInfo(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::SerializationInfo,
                        >,
                        2usize,
                    >("ReadSerializationInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadSerializationInfo", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnknownClassData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ClassDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ClassDataNode,
                        >,
                        3usize,
                    >("ReadUnknownClassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadUnknownClassData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnknownCollectionData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataNode,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataNode,
                        >,
                        3usize,
                    >("ReadUnknownCollectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadUnknownCollectionData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnknownISerializableData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializableDataNode,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ISerializableDataNode,
                        >,
                        3usize,
                    >("ReadUnknownISerializableData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadUnknownISerializableData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializableDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnknownPrimitiveData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        4usize,
                    >("ReadUnknownPrimitiveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadUnknownPrimitiveData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, _cordl_type, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnknownXmlData(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataNode,
                        >,
                        3usize,
                    >("ReadUnknownXmlData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadUnknownXmlData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceDeserializedObject(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oldObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ReplaceDeserializedObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReplaceDeserializedObject", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (id, oldObj, newObj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceScopedKnownTypesTop(
        &mut self,
        knownDataContracts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::DataContract,
                >,
            >,
        >,
        knownTypesAddedInCurrentScope: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlQualifiedName,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Runtime::Serialization::DataContract,
                                    >,
                                >,
                            >,
                            bool,
                        ),
                        bool,
                        2usize,
                    >("ReplaceScopedKnownTypesTop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReplaceScopedKnownTypesTop", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (knownDataContracts, knownTypesAddedInCurrentScope),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ResetAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetAttributes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveDataContractFromTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        0usize,
                    >("ResolveDataContractFromTypeName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveDataContractFromTypeName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SkipUnknownElement(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SkipUnknownElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SkipUnknownElement", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartReadExtensionDataValue(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("StartReadExtensionDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartReadExtensionDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StoreCollectionMemberInfo(
        &mut self,
        collectionMember: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("StoreCollectionMemberInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StoreCollectionMemberInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (collectionMember))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArrayExceededSizeException(
        arraySize: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ThrowArrayExceededSizeException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowArrayExceededSizeException", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (arraySize, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowNullValueReturnedForGetOnlyCollectionException(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ThrowNullValueReturnedForGetOnlyCollectionException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowNullValueReturnedForGetOnlyCollectionException", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowRequiredMemberMissingException(
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        memberIndex: i32,
        requiredIndex: i32,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::XmlDictionaryString,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ThrowRequiredMemberMissingException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowRequiredMemberMissingException", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (xmlReader, memberIndex, requiredIndex, memberNames),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimArraySize<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("TrimArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimArraySize", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (array, _cordl_size))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryHandleNullOrRef(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        retObj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlReaderDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        5usize,
                    >("TryHandleNullOrRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryHandleNullOrRef", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (reader, declaredType, name, ns, retObj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractSerializer,
        >,
        rootTypeDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContractSerializer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContractResolver,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (serializer, rootTypeDataContract, dataContractResolver),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DeserializedObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::HybridObjectCache,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::HybridObjectCache,
                        >,
                        0usize,
                    >("get_DeserializedObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DeserializedObjects", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::HybridObjectCache,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                        0usize,
                    >("get_Document")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Document", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsGetOnlyCollection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsGetOnlyCollection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsGetOnlyCollection", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsGetOnlyCollection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_IsGetOnlyCollection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_IsGetOnlyCollection", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContext"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
