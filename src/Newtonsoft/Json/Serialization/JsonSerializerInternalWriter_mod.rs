#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalWriter {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase,
    pub _rootType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _rootLevel: i32,
    pub _serializeStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonSerializerInternalWriter";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    type Target = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    pub fn CalculatePropertyValues(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        memberContract: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::JsonContract,
            >,
        >,
        memberValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Newtonsoft::Json::Serialization::JsonContract,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        7usize,
                    >("CalculatePropertyValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CalculatePropertyValues", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        property,
                        memberContract,
                        memberValue,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckForCircularReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        6usize,
                    >("CheckForCircularReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckForCircularReference", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        property,
                        contract,
                        containerContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetContract(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::JsonContract,
                        >,
                        1usize,
                    >("GetContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetContract", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = unsafe { method.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetContractSafe(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::JsonContract,
                        >,
                        1usize,
                    >("GetContractSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetContractSafe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = unsafe { method.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetInternalSerializer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
                        >,
                        0usize,
                    >("GetInternalSerializer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetInternalSerializer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        escape: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("GetPropertyName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetPropertyName", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (writer, name, contract, escape))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("GetReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetReference", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (writer, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleError(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        initialDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HandleError", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, initialDepth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasCreatorParameter(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        2usize,
                    >("HasCreatorParameter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasCreatorParameter", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (contract, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_DefaultValueHandling_DefaultValueHandling0(
        &mut self,
        value: crate::Newtonsoft::Json::DefaultValueHandling,
        flag: crate::Newtonsoft::Json::DefaultValueHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Newtonsoft::Json::DefaultValueHandling,
                            crate::Newtonsoft::Json::DefaultValueHandling,
                        ),
                        bool,
                        2usize,
                    >("HasFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasFlag", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value, flag))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_PreserveReferencesHandling_PreserveReferencesHandling1(
        &mut self,
        value: crate::Newtonsoft::Json::PreserveReferencesHandling,
        flag: crate::Newtonsoft::Json::PreserveReferencesHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Newtonsoft::Json::PreserveReferencesHandling,
                            crate::Newtonsoft::Json::PreserveReferencesHandling,
                        ),
                        bool,
                        2usize,
                    >("HasFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasFlag", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value, flag))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_TypeNameHandling_TypeNameHandling2(
        &mut self,
        value: crate::Newtonsoft::Json::TypeNameHandling,
        flag: crate::Newtonsoft::Json::TypeNameHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Newtonsoft::Json::TypeNameHandling,
                            crate::Newtonsoft::Json::TypeNameHandling,
                        ),
                        bool,
                        2usize,
                    >("HasFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasFlag", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value, flag))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSpecified(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        3usize,
                    >("IsSpecified")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSpecified", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (writer, property, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer))?;
        Ok(__cordl_object.into())
    }
    pub fn OnSerialized(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnSerialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnSerialized", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, contract, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSerializing(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnSerializing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnSerializing", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, contract, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveIsReference(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        crate::System::Nullable_1<bool>,
                        4usize,
                    >("ResolveIsReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ResolveIsReference", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<bool> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (contract, property, collectionContract, containerProperty),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        jsonWriter: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Serialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Serialize", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (jsonWriter, value, objectType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeConvertable(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        converter: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonConverter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeConvertable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeConvertable", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        converter,
                        value,
                        contract,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeDictionary(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IDictionary,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeDictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeDictionary", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        values,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeDynamic(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::IDynamicMetaObjectProvider,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Dynamic::IDynamicMetaObjectProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeDynamic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeDynamic", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeISerializable(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializable,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ISerializable,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeISerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeISerializable", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeList(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerable,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonArrayContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeList", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        values,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeMultidimensionalArray_JsonContainerContract_JsonProperty0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonArrayContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeMultidimensionalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeMultidimensionalArray", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        values,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeMultidimensionalArray_i32_Il2CppArray1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        initialDepth: i32,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonArrayContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeMultidimensionalArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeMultidimensionalArray", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (writer, values, contract, member, initialDepth, indices),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonObjectContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeObject", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializePrimitive(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializePrimitive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializePrimitive", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        containerContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeString(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonStringContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonStringContract,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SerializeString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeString", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, value, contract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeValue(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        valueContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SerializeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeValue", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        valueContract,
                        member,
                        containerContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ShouldSerialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldSerialize", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (writer, property, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteDynamicProperty(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("ShouldWriteDynamicProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldWriteDynamicProperty", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (memberValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteProperty(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonObjectContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ShouldWriteProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldWriteProperty", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (memberValue, containerContract, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        valueContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldWriteReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldWriteReference", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        value,
                        property,
                        valueContract,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteType(
        &mut self,
        typeNameHandlingFlag: crate::Newtonsoft::Json::TypeNameHandling,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Newtonsoft::Json::TypeNameHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldWriteType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldWriteType", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        typeNameHandlingFlag,
                        contract,
                        member,
                        containerContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertToString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        s: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryConvertToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryConvertToString", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, _cordl_type, s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectStart(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("WriteObjectStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteObjectStart", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        value,
                        contract,
                        member,
                        collectionContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteReference", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteReferenceIdProperty(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteReferenceIdProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteReferenceIdProperty", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, _cordl_type, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartArray(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonArrayContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContainerContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        bool,
                        6usize,
                    >("WriteStartArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteStartArray", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        writer,
                        values,
                        contract,
                        member,
                        containerContract,
                        containerProperty,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeProperty(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonWriter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteTypeProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteTypeProperty", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::JsonSerializer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializer))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
