#[cfg(feature = "cordl_class_System+Runtime+Serialization+DataContractSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct DataContractSerializer {
    __cordl_parent: crate::System::Runtime::Serialization::XmlObjectSerializer,
    pub rootType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub rootContract:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    pub needsContractNsAtRoot: bool,
    pub rootName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    pub rootNamespace: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    pub maxItemsInObjectGraph: i32,
    pub ignoreExtensionDataObject: bool,
    pub preserveObjectReferences: bool,
    pub dataContractSurrogate:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataContractSurrogate>,
    pub knownTypeList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub knownDataContracts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
            quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
        >,
    >,
    pub dataContractResolver:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContractResolver>,
    pub serializeReadOnlyTypes: bool,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+DataContractSerializer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::DataContractSerializer
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "DataContractSerializer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Runtime+Serialization+DataContractSerializer")]
impl std::ops::Deref for crate::System::Runtime::Serialization::DataContractSerializer {
    type Target = crate::System::Runtime::Serialization::XmlObjectSerializer;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataContractSerializer")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::DataContractSerializer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataContractSerializer")]
impl crate::System::Runtime::Serialization::DataContractSerializer {
    pub fn GetDataContract(
        declaredTypeContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        3usize,
                    >("GetDataContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataContract", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (declaredTypeContract, declaredType, objectType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDeserializeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "GetDeserializeType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDeserializeType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializeType(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetSerializeType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSerializeType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, (graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSurrogatedType(
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataContractSurrogate,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Type>, 2usize>(
                        "GetSurrogatedType"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSurrogatedType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked((), (dataContractSurrogate, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        maxItemsInObjectGraph: i32,
        ignoreExtensionDataObject: bool,
        preserveObjectReferences: bool,
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
        serializeReadOnlyTypes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        i32,
                        bool,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataContractSurrogate,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 8usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    _cordl_type,
                    knownTypes,
                    maxItemsInObjectGraph,
                    ignoreExtensionDataObject,
                    preserveObjectReferences,
                    dataContractSurrogate,
                    dataContractResolver,
                    serializeReadOnlyTypes,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsStartObject(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlReaderDelegator,
                    >), bool, 1usize>("InternalIsStartObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalIsStartObject",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalReadObject_DataContractResolver1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        verifyObjectName: bool,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 3usize>(
                        "InternalReadObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalReadObject",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlReader, verifyObjectName, dataContractResolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalReadObject_XmlReaderDelegator__cordl_bool0(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        verifyObjectName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlReaderDelegator,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 2usize>(
                        "InternalReadObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalReadObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlReader, verifyObjectName))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteEndObject(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlWriterDelegator,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "InternalWriteEndObject"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteEndObject",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteObjectContent_DataContractResolver1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InternalWriteObjectContent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteObjectContent",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (writer, graph, dataContractResolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteObjectContent_XmlWriterDelegator_Il2CppObject0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalWriteObjectContent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteObjectContent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteObject_DataContractResolver1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InternalWriteObject"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteObject",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (writer, graph, dataContractResolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteObject_XmlWriterDelegator_Il2CppObject0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalWriteObject"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteStartObject(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalWriteStartObject"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalWriteStartObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_IEnumerable_1_1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, knownTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_i32__cordl_bool__cordl_bool_IDataContractSurrogate2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        maxItemsInObjectGraph: i32,
        ignoreExtensionDataObject: bool,
        preserveObjectReferences: bool,
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                _cordl_type,
                knownTypes,
                maxItemsInObjectGraph,
                ignoreExtensionDataObject,
                preserveObjectReferences,
                dataContractSurrogate,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_i32__cordl_bool__cordl_bool_IDataContractSurrogate_DataContractResolver3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        maxItemsInObjectGraph: i32,
        ignoreExtensionDataObject: bool,
        preserveObjectReferences: bool,
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                _cordl_type,
                knownTypes,
                maxItemsInObjectGraph,
                ignoreExtensionDataObject,
                preserveObjectReferences,
                dataContractSurrogate,
                dataContractResolver,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadObject_XmlDictionaryReader1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryReader>,
        verifyObjectName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryReader>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 2usize>(
                        "ReadObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (reader, verifyObjectName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject_XmlReader0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        verifyObjectName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 2usize>(
                        "ReadObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (reader, verifyObjectName))? };
        Ok(__cordl_ret.into())
    }
    pub fn SurrogateToDataContractType(
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
        oldObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        surrogatedDeclaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objType: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::System::Type>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataContractSurrogate,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 4usize>(
                        "SurrogateToDataContractType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SurrogateToDataContractType",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    dataContractSurrogate,
                    oldObj,
                    surrogatedDeclaredType,
                    objType,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndObject_XmlDictionaryWriter1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::XmlDictionaryWriter,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteEndObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEndObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndObject_XmlWriter0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteEndObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEndObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectContent_XmlDictionaryWriter1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteObjectContent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteObjectContent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectContent_XmlWriter0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteObjectContent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteObjectContent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartObject_XmlDictionaryWriter1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteStartObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteStartObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartObject_XmlWriter0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteStartObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteStartObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer, graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type, knownTypes))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_i32__cordl_bool__cordl_bool_IDataContractSurrogate2(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        maxItemsInObjectGraph: i32,
        ignoreExtensionDataObject: bool,
        preserveObjectReferences: bool,
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        i32,
                        bool,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataContractSurrogate,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    _cordl_type,
                    knownTypes,
                    maxItemsInObjectGraph,
                    ignoreExtensionDataObject,
                    preserveObjectReferences,
                    dataContractSurrogate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_i32__cordl_bool__cordl_bool_IDataContractSurrogate_DataContractResolver3(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        knownTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        maxItemsInObjectGraph: i32,
        ignoreExtensionDataObject: bool,
        preserveObjectReferences: bool,
        dataContractSurrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        >,
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        i32,
                        bool,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::IDataContractSurrogate,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    _cordl_type,
                    knownTypes,
                    maxItemsInObjectGraph,
                    ignoreExtensionDataObject,
                    preserveObjectReferences,
                    dataContractSurrogate,
                    dataContractResolver,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DataContractResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContractResolver>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::DataContractResolver,
                    >, 0usize>("get_DataContractResolver")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_DataContractResolver",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DataContractSurrogate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataContractSurrogate>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::IDataContractSurrogate,
                    >, 0usize>("get_DataContractSurrogate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_DataContractSurrogate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataContractSurrogate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreExtensionDataObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IgnoreExtensionDataObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IgnoreExtensionDataObject",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_KnownDataContracts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                        >,
                    >, 0usize>("get_KnownDataContracts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_KnownDataContracts",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxItemsInObjectGraph(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_MaxItemsInObjectGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_MaxItemsInObjectGraph",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PreserveObjectReferences(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_PreserveObjectReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_PreserveObjectReferences",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RootContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        0usize,
                    >("get_RootContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_RootContract", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SerializeReadOnlyTypes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_SerializeReadOnlyTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_SerializeReadOnlyTypes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+DataContractSerializer")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::DataContractSerializer
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
