#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerReadContextComplex {
    __cordl_parent: crate::System::Runtime::Serialization::XmlObjectSerializerReadContext,
    pub preserveObjectReferences: bool,
    pub dataContractSurrogate: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IDataContractSurrogate,
    >,
    pub mode: crate::System::Runtime::Serialization::SerializationMode,
    pub binder: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationBinder,
    >,
    pub surrogateSelector: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISurrogateSelector,
    >,
    pub assemblyFormat: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    pub surrogateDataContracts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerReadContextComplex";
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
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex {
    type Target = crate::System::Runtime::Serialization::XmlObjectSerializerReadContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex")]
impl crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex {
    #[cfg(
        feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
    )]
    pub type TopLevelAssemblyTypeResolver = crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver;
    #[cfg(
        feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
    )]
    pub type XmlObjectDataContractTypeInfo = crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo;
    #[cfg(
        feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
    )]
    pub type XmlObjectDataContractTypeKey = crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey;
    pub fn CheckIfTypeSerializable(
        &mut self,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isMemberTypeSerializable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CheckIfTypeSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckIfTypeSerializable", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (memberType, isMemberTypeSerializable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfTypeSerializableForSharedTypeMode(
        &mut self,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("CheckIfTypeSerializableForSharedTypeMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckIfTypeSerializableForSharedTypeMode", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (memberType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckTypeForwardedTo(
        sourceAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        destinationAssembly: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        >,
        resolvedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::Assembly,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::Assembly,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CheckTypeForwardedTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckTypeForwardedTo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (sourceAssembly, destinationAssembly, resolvedType),
                )?
        };
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
    pub fn GetDataContract_RuntimeTypeHandle_Type1(
        &mut self,
        typeHandle: crate::System::RuntimeTypeHandle,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        2usize,
                    >("GetDataContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataContract", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (typeHandle, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataContract_i32_RuntimeTypeHandle0(
        &mut self,
        id: i32,
        typeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::System::RuntimeTypeHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        2usize,
                    >("GetDataContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataContract", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (id, typeHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSurrogatedType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetSurrogatedType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSurrogatedType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserializeInSharedTypeMode(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredTypeID: i32,
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
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("InternalDeserializeInSharedTypeMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserializeInSharedTypeMode", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, declaredTypeID, declaredType, name, ns),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserializeWithSurrogate(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        surrogateDataContract: quest_hook::libil2cpp::Gc<
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
                    >("InternalDeserializeWithSurrogate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalDeserializeWithSurrogate", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlReader, declaredType, surrogateDataContract, name, ns),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDeserialize_Type_DataContract_Il2CppString2(
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
    pub fn InternalDeserialize_Type_Il2CppString1(
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
    pub fn InternalDeserialize_i32_RuntimeTypeHandle_Il2CppString0(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlReaderDelegator,
        >,
        declaredTypeID: i32,
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
                .invoke_unchecked(
                    self,
                    (xmlReader, declaredTypeID, declaredTypeHandle, name, ns),
                )?
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
    pub fn ResolveDataContractInSharedTypeMode(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assembly: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        >,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Reflection::Assembly,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        4usize,
                    >("ResolveDataContractInSharedTypeMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveDataContractInSharedTypeMode", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (assemblyName, typeName, assembly, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveDataContractTypeInSharedTypeMode(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assembly: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Reflection::Assembly,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        3usize,
                    >("ResolveDataContractTypeInSharedTypeMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveDataContractTypeInSharedTypeMode", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, (assemblyName, typeName, assembly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSimpleAssemblyName_AssemblyName0(
        assemblyName: quest_hook::libil2cpp::Gc<crate::System::Reflection::AssemblyName>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::AssemblyName,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                        1usize,
                    >("ResolveSimpleAssemblyName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveSimpleAssemblyName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = unsafe { cordl_method_info.invoke_unchecked((), (assemblyName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSimpleAssemblyName_Il2CppString1(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                        1usize,
                    >("ResolveSimpleAssemblyName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveSimpleAssemblyName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = unsafe { cordl_method_info.invoke_unchecked((), (assemblyName))? };
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
    pub fn get_Mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::SerializationMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Runtime::Serialization::SerializationMode,
                        0usize,
                    >("get_Mode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Mode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::Serialization::SerializationMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub topLevelAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerReadContextComplex/TopLevelAssemblyTypeResolver";
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
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
impl crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    pub fn New(
        topLevelAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (topLevelAssembly))?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveType(
        &mut self,
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        simpleTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::Assembly,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        3usize,
                    >("ResolveType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveType", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (assembly, simpleTypeName, ignoreCase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        topLevelAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (topLevelAssembly))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+TopLevelAssemblyTypeResolver"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_TopLevelAssemblyTypeResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerReadContextComplex/XmlObjectDataContractTypeInfo";
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
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
impl crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    pub fn New(
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assembly, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::Assembly,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (assembly, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Assembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                        0usize,
                    >("get_Assembly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Assembly", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        0usize,
                    >("get_Type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Type", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerReadContextComplex/XmlObjectDataContractTypeKey";
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
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
impl crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyName, typeName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (assemblyName, typeName))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerReadContextComplex+XmlObjectDataContractTypeKey"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerReadContextComplex_XmlObjectDataContractTypeKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
