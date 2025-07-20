#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _currentErrorContext: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ErrorContext,
    >,
    pub _mappings: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub Serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    pub TraceWriter: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ITraceWriter,
    >,
    pub InternalSerializer: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonSerializerInternalBase";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
    )]
    pub type ReferenceEqualsEqualityComparer = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer;
    pub fn ClearErrorContext(
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
                    >("ClearErrorContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearErrorContext", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetErrorContext(
        &mut self,
        currentObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::ErrorContext>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::ErrorContext,
                        >,
                        4usize,
                    >("GetErrorContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetErrorContext", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ErrorContext,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (currentObject, member, path, error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsErrorHandled(
        &mut self,
        currentObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        keyValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lineInfo: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::IJsonLineInfo>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::IJsonLineInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        bool,
                        6usize,
                    >("IsErrorHandled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsErrorHandled", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (currentObject, contract, keyValue, lineInfo, path, ex),
                )?
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
    pub fn ResolvedNullValueHandling(
        &mut self,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::NullValueHandling> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonObjectContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Serialization::JsonProperty,
                            >,
                        ),
                        crate::Newtonsoft::Json::NullValueHandling,
                        2usize,
                    >("ResolvedNullValueHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResolvedNullValueHandling", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::NullValueHandling = unsafe {
            cordl_method_info.invoke_unchecked(self, (containerContract, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn get_DefaultReferenceMappings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_DefaultReferenceMappings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DefaultReferenceMappings", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonSerializerInternalBase/ReferenceEqualsEqualityComparer";
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
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_Generic_IEqualityComparer_System_Object__Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >(
                        "System.Collections.Generic.IEqualityComparer<System.Object>.Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.Generic.IEqualityComparer<System.Object>.Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEqualityComparer_System_Object__GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        i32,
                        1usize,
                    >(
                        "System.Collections.Generic.IEqualityComparer<System.Object>.GetHashCode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.Generic.IEqualityComparer<System.Object>.GetHashCode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
