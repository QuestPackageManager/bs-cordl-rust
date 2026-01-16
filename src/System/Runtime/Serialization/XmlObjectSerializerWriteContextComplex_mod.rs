#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlObjectSerializerWriteContextComplex {
    __cordl_parent: crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
    pub dataContractSurrogate:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::IDataContractSurrogate>,
    pub mode: crate::System::Runtime::Serialization::SerializationMode,
    pub binder:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::SerializationBinder>,
    pub surrogateSelector:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ISurrogateSelector>,
    pub streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    pub surrogateDataContracts: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContextComplex
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlObjectSerializerWriteContextComplex";
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
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
impl std::ops::Deref
    for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContextComplex
{
    type Target = crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
impl std::ops::DerefMut
    for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContextComplex
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
impl crate::System::Runtime::Serialization::XmlObjectSerializerWriteContextComplex {
    pub fn CheckIfTypeSerializable(
        &mut self,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isMemberTypeSerializable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
            cordl_method_info.invoke_unchecked(self, (memberType, isMemberTypeSerializable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfTypeSerializableForSharedTypeMode(
        &mut self,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<crate::System::Type>), bool, 1usize>(
                        "CheckIfTypeSerializableForSharedTypeMode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckIfTypeSerializableForSharedTypeMode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (memberType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataContractSkipValidation(
        &mut self,
        typeId: i32,
        typeHandle: crate::System::RuntimeTypeHandle,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            crate::System::RuntimeTypeHandle,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        3usize,
                    >("GetDataContractSkipValidation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataContractSkipValidation", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (typeId, typeHandle, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataContract_RuntimeTypeHandle_Type0(
        &mut self,
        typeHandle: crate::System::RuntimeTypeHandle,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        > = unsafe { cordl_method_info.invoke_unchecked(self, (typeHandle, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataContract_i32_RuntimeTypeHandle1(
        &mut self,
        id: i32,
        typeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::DataContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
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
                        bool,
                        bool,
                        i32,
                        crate::System::RuntimeTypeHandle,
                    ), quest_hook::libil2cpp::Void, 6usize>("InternalSerialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSerialize",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
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
    pub fn InternalSerializeWithSurrogate(
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
                        bool,
                        bool,
                        i32,
                        crate::System::RuntimeTypeHandle,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "InternalSerializeWithSurrogate"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSerializeWithSurrogate",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
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
        dataContractResolver: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (serializer, rootTypeDataContract, dataContractResolver),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_NetDataContractSerializer_Hashtable1(
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::NetDataContractSerializer,
        >,
        surrogateDataContracts: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer, surrogateDataContracts))?;
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
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "OnEndHandleReference"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEndHandleReference",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, obj, canContainCyclicReference))?
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
                        bool,
                    ), bool, 3usize>("OnHandleReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnHandleReference",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, obj, canContainCyclicReference))?
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
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteAnyType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteAnyType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArraySize(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        _cordl_size: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteArraySize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, _cordl_size))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                    ), quest_hook::libil2cpp::Void, 4usize>("WriteBase64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteBase64",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteBase64_XmlWriterDelegator_Il2CppArray0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteBase64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteBase64",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                    ), bool, 2usize>("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteClrTypeInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, dataContract))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 4usize>("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteClrTypeInfo",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::SerializationInfo,
                        >,
                    ), bool, 3usize>("WriteClrTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteClrTypeInfo",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xmlWriter, dataContractType, serInfo))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                    ), quest_hook::libil2cpp::Void, 4usize>("WriteQName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteQName",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteQName_XmlWriterDelegator_XmlQualifiedName0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
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
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteQName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteQName",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                    ), quest_hook::libil2cpp::Void, 4usize>("WriteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteString",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteString_XmlWriterDelegator_Il2CppString0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteString",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlWriterDelegator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                    ), quest_hook::libil2cpp::Void, 4usize>("WriteUri")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteUri",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value, name, ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteUri_XmlWriterDelegator_Uri0(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
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
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    ), quest_hook::libil2cpp::Void, 2usize>("WriteUri")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteUri",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xmlWriter, value))? };
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
                            crate::System::Runtime::Serialization::DataContractSerializer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContract,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataContractResolver,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (serializer, rootTypeDataContract, dataContractResolver),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NetDataContractSerializer_Hashtable1(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::NetDataContractSerializer,
        >,
        surrogateDataContracts: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::NetDataContractSerializer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (serializer, surrogateDataContracts))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Runtime::Serialization::SerializationMode>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: crate::System::Runtime::Serialization::SerializationMode =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlObjectSerializerWriteContextComplex")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::XmlObjectSerializerWriteContextComplex
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
