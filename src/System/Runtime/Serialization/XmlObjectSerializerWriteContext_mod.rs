#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContext"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerWriteContext {
    __cordl_parent: crate::System::Runtime::Serialization::XmlObjectSerializerContext,
    pub byValObjectsInScope: crate::System::Runtime::Serialization::ObjectReferenceStack,
    pub xmlSerializableWriter: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlSerializableWriter,
    >,
    pub preserveObjectReferences: bool,
    pub serializedObjects: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ObjectToIdCache,
    >,
    pub isGetOnlyCollection: bool,
    pub unsafeTypeForwardingEnabled: bool,
    pub serializeReadOnlyTypes: bool,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContext"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerWriteContext";
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
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContext")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext {
    type Target = crate::System::Runtime::Serialization::XmlObjectSerializerContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContext")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContext")]
impl crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext {
    pub fn CreateContext_DataContractSerializer_DataContract_DataContractResolver0(
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
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
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
                            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
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
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (serializer, rootTypeDataContract, dataContractResolver),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateContext_NetDataContractSerializer_Hashtable1(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::NetDataContractSerializer,
        >,
        surrogateDataContracts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::NetDataContractSerializer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Hashtable,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
                        >,
                        2usize,
                    >("CreateContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateContext", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (serializer, surrogateDataContracts))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultValue<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), T, 0usize>("GetDefaultValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultValue", 0usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHasValue<T>(
        value: crate::System::Nullable_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Nullable_1<T>),
                        bool,
                        1usize,
                    >("GetHasValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHasValue", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNullableValue<T>(
        value: crate::System::Nullable_1<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Nullable_1<T>),
                        T,
                        1usize,
                    >("GetNullableValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNullableValue", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializable,
        >,
        serInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ISerializable,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::SerializationInfo,
                            >,
                            crate::System::Runtime::Serialization::StreamingContext,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GetObjectData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetObjectData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj, serInfo, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGraphAtTopLevel(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandleGraphAtTopLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleGraphAtTopLevel", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (writer, obj, contract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementArrayCount(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IncrementArrayCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementArrayCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, array))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementCollectionCountGeneric<T>(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ICollection_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::ICollection_1<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IncrementCollectionCountGeneric")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementCollectionCountGeneric", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, collection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementCollectionCount_ICollection0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        collection: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::ICollection,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IncrementCollectionCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementCollectionCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, collection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementCollectionCount_i32_1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IncrementCollectionCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementCollectionCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSerialize(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isDeclaredType: bool,
        writeXsiType: bool,
        declaredTypeID: i32,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            bool,
                            i32,
                            crate::System::RuntimeTypeHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InternalSerialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalSerialize", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        xmlWriter,
                        obj,
                        isDeclaredType,
                        writeXsiType,
                        declaredTypeID,
                        declaredTypeHandle,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSerializeReference(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isDeclaredType: bool,
        writeXsiType: bool,
        declaredTypeID: i32,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            bool,
                            i32,
                            crate::System::RuntimeTypeHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("InternalSerializeReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalSerializeReference", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        xmlWriter,
                        obj,
                        isDeclaredType,
                        writeXsiType,
                        declaredTypeID,
                        declaredTypeHandle,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DataContractSerializer_DataContract_DataContractResolver0(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractSerializer,
        >,
        rootTypeDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        resolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer, rootTypeDataContract, resolver))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NetDataContractSerializer1(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::NetDataContractSerializer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer))?;
        Ok(__cordl_object.into())
    }
    pub fn OnEndHandleReference(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        canContainCyclicReference: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnEndHandleReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEndHandleReference", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, obj, canContainCyclicReference))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnHandleIsReference(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        3usize,
                    >("OnHandleIsReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnHandleIsReference", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, contract, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnHandleReference(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        canContainCyclicReference: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                        ),
                        bool,
                        3usize,
                    >("OnHandleReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnHandleReference", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, obj, canContainCyclicReference))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveType(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        >,
        typeNamespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Xml::XmlDictionaryString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Xml::XmlDictionaryString,
                                >,
                            >,
                        ),
                        bool,
                        4usize,
                    >("ResolveType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolveType", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (objectType, declaredType, typeName, typeNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeAndVerifyType(
        &mut self,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        verifyKnownType: bool,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeAndVerifyType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SerializeAndVerifyType", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        dataContract,
                        xmlWriter,
                        obj,
                        verifyKnownType,
                        declaredTypeHandle,
                        declaredType,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeWithXsiType(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectTypeHandle: crate::System::RuntimeTypeHandle,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        declaredTypeID: i32,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            i32,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SerializeWithXsiType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SerializeWithXsiType", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        xmlWriter,
                        obj,
                        objectTypeHandle,
                        objectType,
                        declaredTypeID,
                        declaredTypeHandle,
                        declaredType,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeWithXsiTypeAtTopLevel(
        &mut self,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        originalDeclaredTypeHandle: crate::System::RuntimeTypeHandle,
        graphType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SerializeWithXsiTypeAtTopLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SerializeWithXsiTypeAtTopLevel", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (dataContract, xmlWriter, obj, originalDeclaredTypeHandle, graphType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeWithoutXsiType(
        &mut self,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::RuntimeTypeHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SerializeWithoutXsiType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SerializeWithoutXsiType", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (dataContract, xmlWriter, obj, declaredTypeHandle),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StoreIsGetOnlyCollection(
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
                    >("StoreIsGetOnlyCollection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StoreIsGetOnlyCollection", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowRequiredMemberMustBeEmitted(
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ThrowRequiredMemberMustBeEmitted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowRequiredMemberMustBeEmitted", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (memberName, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryWriteDeserializedExtensionData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IDataNode,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryWriteDeserializedExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryWriteDeserializedExtensionData", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteAnyType(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteAnyType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteAnyType", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArraySize(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteArraySize", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteBase64_XmlDictionaryString_XmlDictionaryString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteBase64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteBase64", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteBase64_XmlWriterDelegator_Il2CppArray0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteBase64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteBase64", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteClrTypeInfo_DataContract0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                        ),
                        bool,
                        2usize,
                    >("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteClrTypeInfo", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteClrTypeInfo_Type_Il2CppString_Il2CppString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataContractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        clrTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clrAssemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        4usize,
                    >("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteClrTypeInfo", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlWriter, dataContractType, clrTypeName, clrAssemblyName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteClrTypeInfo_Type_SerializationInfo2(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataContractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        serInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::SerializationInfo,
                            >,
                        ),
                        bool,
                        3usize,
                    >("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteClrTypeInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, dataContractType, serInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDataContractValue(
        &mut self,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        declaredTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::RuntimeTypeHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteDataContractValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteDataContractValue", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (dataContract, xmlWriter, obj, declaredTypeHandle),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionClassData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionClassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionClassData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionCollectionData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::CollectionDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionCollectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionCollectionData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteExtensionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, extensionData, memberIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionDataMember(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataMember,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataMember,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionDataMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionDataMember", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, member))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionDataTypeInfo(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionDataTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionDataTypeInfo", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionDataValue(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::IDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionDataValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionISerializableData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializableDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ISerializableDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionISerializableData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionISerializableData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteExtensionXmlData(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataNode: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlDataNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlDataNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteExtensionXmlData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteExtensionXmlData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteISerializable(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ISerializable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteISerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteISerializable", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteIXmlSerializable_XmlSerializableWriter1(
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        xmlSerializableWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlSerializableWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlSerializableWriter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteIXmlSerializable", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xmlWriter, obj, xmlSerializableWriter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteIXmlSerializable_XmlWriterDelegator_Il2CppObject0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteIXmlSerializable", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteNull_Type__cordl_bool0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isMemberTypeSerializable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteNull", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlWriter, memberType, isMemberTypeSerializable),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteNull_Type__cordl_bool_XmlDictionaryString_XmlDictionaryString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isMemberTypeSerializable: bool,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("WriteNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteNull", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (xmlWriter, memberType, isMemberTypeSerializable, name, ns),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteNull_XmlWriterDelegator2(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteNull", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteQName_XmlDictionaryString_XmlDictionaryString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlQualifiedName,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteQName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteQName", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteQName_XmlWriterDelegator_XmlQualifiedName0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlQualifiedName,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteQName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteQName", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteResolvedTypeInfo(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        declaredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteResolvedTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteResolvedTypeInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (writer, objectType, declaredType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteRootIXmlSerializable(
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteRootIXmlSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteRootIXmlSerializable", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (xmlWriter, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteSerializationInfo(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        objType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        serInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::SerializationInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteSerializationInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteSerializationInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, objType, serInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteString_XmlDictionaryString_XmlDictionaryString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteString", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteString_XmlWriterDelegator_Il2CppString0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeInfo_DataContract_DataContract0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        declaredContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataContract,
                            >,
                        ),
                        bool,
                        3usize,
                    >("WriteTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteTypeInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (writer, contract, declaredContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeInfo_Il2CppString_Il2CppString1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
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
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
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
                    >("WriteTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteTypeInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (writer, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeInfo_XmlDictionaryString_XmlDictionaryString2(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        dataContractName: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlDictionaryString,
        >,
        dataContractNamespace: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlDictionaryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteTypeInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (writer, dataContractName, dataContractNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteUri_XmlDictionaryString_XmlDictionaryString1(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteUri")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteUri", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteUri_XmlWriterDelegator_Uri0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteUri")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteUri", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataContractSerializer_DataContract_DataContractResolver0(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractSerializer,
        >,
        rootTypeDataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        >,
        resolver: quest_hook::libil2cpp::Gc<
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
                .invoke_unchecked(self, (serializer, rootTypeDataContract, resolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NetDataContractSerializer1(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::NetDataContractSerializer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::NetDataContractSerializer,
                        >),
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
            cordl_method_info.invoke_unchecked(self, (serializer))?
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
    pub fn get_SerializeReadOnlyTypes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_SerializeReadOnlyTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SerializeReadOnlyTypes", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SerializedObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ObjectToIdCache>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ObjectToIdCache,
                        >,
                        0usize,
                    >("get_SerializedObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SerializedObjects", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectToIdCache,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UnsafeTypeForwardingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_UnsafeTypeForwardingEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_UnsafeTypeForwardingEnabled", 0usize
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
    feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContext"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
