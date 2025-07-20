#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer {
    pub const kClearFlagsMask: u32 = 1073741823u32;
    pub const kDynamicStringFlag: u32 = 1073741824u32;
    pub const kUnicodeStringFlag: u32 = 2147483648u32;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
    )]
    pub type BuiltinTypesSerializer = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString"
    )]
    pub type DynamicString = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
    )]
    type ISerializationAdapter = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
    )]
    type ISerializationAdapter_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
    )]
    pub type ObjectTypeData = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData;
    #[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
    pub type Reader = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
    )]
    pub type TypeSerializer = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer;
    #[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
    pub type Writer = crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer;
    pub fn AddSerializationAdapter(
        serializationAdapters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
        adapter: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
        >,
        forceOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddSerializationAdapter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddSerializationAdapter", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (serializationAdapters, adapter, forceOverride))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeHash(
        pData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: u64,
        hash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ComputeHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ComputeHash", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (pData, _cordl_size, hash))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializationAdapter(
        serializationAdapters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        adapter: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("GetSerializationAdapter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSerializationAdapter", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (serializationAdapters, t, adapter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_BuiltinTypesSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/BuiltinTypesSerializer";
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
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
    )]
    pub type ObjectToStringRemap = crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap;
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
        >,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("Deserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Deserialize", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (reader, t, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindBestSeparator(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        seps: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                        ),
                        char,
                        2usize,
                    >("FindBestSeparator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindBestSeparator", 2usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, (str, seps))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
        >,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u32,
                        2usize,
                    >("Serialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Serialize", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (writer, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Dependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Dependencies", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        bool,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        bool,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        bool,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        bool,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        crate::UnityEngine::Hash128,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        crate::UnityEngine::Hash128,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        crate::UnityEngine::Hash128,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        crate::UnityEngine::Hash128,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i32,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i32,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i32,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i32,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i64,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i64,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i64,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        i64,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_BuiltinTypesSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BinaryStorageBuffer_DynamicString {
    pub stringId: u32,
    pub nextId: u32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/DynamicString";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+DynamicString")]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_DynamicString {}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_ISerializationAdapter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/ISerializationAdapter";
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
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
        >,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("Deserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Deserialize", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (reader, t, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
        >,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u32,
                        2usize,
                    >("Serialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Serialize", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (writer, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Dependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Dependencies", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_ISerializationAdapter_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/ISerializationAdapter`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.ResourceManagement.Util",
                        "BinaryStorageBuffer/ISerializationAdapter`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ISerializationAdapter_1"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
>
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
    T,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BinaryStorageBuffer_ObjectTypeData {
    pub typeId: u32,
    pub objectId: u32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/ObjectTypeData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+ObjectTypeData"
)]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ObjectTypeData {}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_Reader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub m_Adapters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
            >,
        >,
    >,
    pub m_Cache: crate::UnityEngine::ResourceManagement::Util::LRUCache_2<
        u32,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub stringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/Reader";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader {
    pub fn AddSerializationAdapter(
        &mut self,
        a: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddSerializationAdapter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddSerializationAdapter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (a))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        0usize,
                    >("GetBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        maxCachedObjects: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Init", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data, maxCachedObjects, adapters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray_i32_Il2CppArray0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        maxCachedObjects: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data, maxCachedObjects, adapters))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_u32_i32_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: u32,
        maxCachedObjects: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (inputStream, bufferSize, maxCachedObjects, adapters),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ReadAutoEncodedString(
        &mut self,
        id: u32,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ReadAutoEncodedString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadAutoEncodedString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (id, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadDynamicString(
        &mut self,
        id: u32,
        sep: char,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, char, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("ReadDynamicString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadDynamicString", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (id, sep, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObjectArray_Type_u32__cordl_bool1(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        id: u32,
        cacheValues: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, u32, bool),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        3usize,
                    >("ReadObjectArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObjectArray", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, (t, id, cacheValues))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObjectArray_u32__cordl_bool0(
        &mut self,
        id: u32,
        cacheValues: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, bool),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        2usize,
                    >("ReadObjectArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObjectArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, (id, cacheValues))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObjectArray_u32__cordl_bool2<T>(
        &mut self,
        id: u32,
        cacheValues: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("ReadObjectArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObjectArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { method.invoke_unchecked(self, (id, cacheValues))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject_Type_u32__cordl_bool2(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        id: u32,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, u32, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("ReadObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObject", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (t, id, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject_u32__cordl_bool0(
        &mut self,
        id: u32,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("ReadObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObject", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (id, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject_u32__cordl_bool1<T>(
        &mut self,
        offset: u32,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32, bool), T, 2usize>("ReadObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadObject", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked(self, (offset, cacheValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadString(
        &mut self,
        id: u32,
        sep: char,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, char, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("ReadString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadString", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (id, sep, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringInternal(
        &mut self,
        offset: u32,
        enc: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("ReadStringInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadStringInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (offset, enc, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue<T>(&mut self, id: u32) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32), T, 1usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValue", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueArray<T>(
        &mut self,
        id: u32,
        cacheValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, bool),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        2usize,
                    >("ReadValueArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { method.invoke_unchecked(self, (id, cacheValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCachedValue<T>(
        &mut self,
        offset: u32,
        val: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<T>),
                        bool,
                        2usize,
                    >("TryGetCachedValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryGetCachedValue", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (offset, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_i32_Il2CppArray0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        maxCachedObjects: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data, maxCachedObjects, adapters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_u32_i32_Il2CppArray1(
        &mut self,
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: u32,
        maxCachedObjects: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            u32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (inputStream, bufferSize, maxCachedObjects, adapters),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Reader")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_TypeSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/TypeSerializer";
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
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
    )]
    pub type Data = crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data;
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Reader,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("Deserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Deserialize", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (reader, _cordl_type, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
        >,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u32,
                        2usize,
                    >("Serialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Serialize", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (writer, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Dependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Dependencies", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
> for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
> for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl AsRef<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    >,
> for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer"
)]
impl AsMut<
    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    >,
> for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_TypeSerializer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter_1<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryStorageBuffer_Writer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub totalBytes: u32,
    pub defaulChunkSize: u32,
    pub chunks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk,
            >,
        >,
    >,
    pub existingValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::Hash128,
            u32,
        >,
    >,
    pub serializationAdapters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/Writer";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
impl crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk"
    )]
    pub type Chunk = crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
    )]
    pub type StringParts = crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts;
    pub fn ComputeStringSize(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isUnicode: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        u32,
                        2usize,
                    >("ComputeStringSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ComputeStringSize", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (str, isUnicode))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindChunkWithSpace(
        &mut self,
        length: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk,
                        >,
                        1usize,
                    >("FindChunkWithSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindChunkWithSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk,
        > = unsafe { method.invoke_unchecked(self, (length))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsUnicode(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("IsUnicode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsUnicode", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (str))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        chunkSize: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (chunkSize, adapters))?;
        Ok(__cordl_object.into())
    }
    pub fn RecurseDynamicStringParts(
        &mut self,
        parts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts,
            >,
        >,
        index: i32,
        sep: char,
        minSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts,
                                >,
                            >,
                            i32,
                            char,
                            u32,
                        ),
                        u32,
                        4usize,
                    >("RecurseDynamicStringParts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RecurseDynamicStringParts", 4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (parts, index, sep, minSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReserveInternal(
        &mut self,
        dataSize: u32,
        prefixSize: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32, bool), u32, 2usize>("ReserveInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReserveInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (dataSize, prefixSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reserve_0<T>(&mut self) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), u32, 0usize>("Reserve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Reserve", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Reserve_u32_1<T>(&mut self, count: u32) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32), u32, 1usize>("Reserve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Reserve", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (count))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeToByteArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        0usize,
                    >("SerializeToByteArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeToByteArray", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeToStream(
        &mut self,
        str: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        u32,
                        1usize,
                    >("SerializeToStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializeToStream", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (str))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteAutoEncodedString(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        u32,
                        1usize,
                    >("WriteAutoEncodedString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteAutoEncodedString", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (str))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDynamicString(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sep: char,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                        ),
                        u32,
                        2usize,
                    >("WriteDynamicString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDynamicString", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (str, sep))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteInternal_Il2CppObject_u32__cordl_bool0(
        &mut self,
        pData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataSize: u32,
        prefixSize: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u32,
                            bool,
                        ),
                        u32,
                        3usize,
                    >("WriteInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (pData, dataSize, prefixSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteInternal_u32_Il2CppObject_u32__cordl_bool1(
        &mut self,
        id: u32,
        pData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataSize: u32,
        prefixSize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (id, pData, dataSize, prefixSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializeTypeData: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                        ),
                        u32,
                        2usize,
                    >("WriteObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteObject", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (obj, serializeTypeData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjects<T>(
        &mut self,
        objs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        serizalizeTypeData: bool,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<T>,
                            >,
                            bool,
                        ),
                        u32,
                        2usize,
                    >("WriteObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteObjects", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (objs, serizalizeTypeData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteString(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sep: char,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                        ),
                        u32,
                        2usize,
                    >("WriteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteString", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (str, sep))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringInternal(
        &mut self,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enc: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                        ),
                        u32,
                        2usize,
                    >("WriteStringInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteStringInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (val, enc))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnicodeString(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        u32,
                        1usize,
                    >("WriteUnicodeString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteUnicodeString", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (str))? };
        Ok(__cordl_ret.into())
    }
    pub fn Write_ByRefMut0<T>(
        &mut self,
        val: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        u32,
                        1usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppArray__cordl_bool4<T>(
        &mut self,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        hashElements: bool,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            bool,
                        ),
                        u32,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (values, hashElements))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_T1<T>(&mut self, val: T) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(T), u32, 1usize>("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Write_u32_ByRefMut2<T>(
        &mut self,
        offset: u32,
        val: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<T>),
                        u32,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (offset, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Write_u32_Il2CppArray__cordl_bool5<T>(
        &mut self,
        offset: u32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        hashElements: bool,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            bool,
                        ),
                        u32,
                        3usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (offset, values, hashElements))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_u32_T3<T>(
        &mut self,
        offset: u32,
        val: T,
    ) -> quest_hook::libil2cpp::Result<u32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32, T), u32, 2usize>("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Write", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (offset, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        chunkSize: i32,
        adapters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_ISerializationAdapter,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (chunkSize, adapters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), u32, 0usize>("get_Length")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Length", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::BinaryStorageBuffer_Writer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    pub stringId: u32,
    pub separator: char,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/BuiltinTypesSerializer/ObjectToStringRemap";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+BuiltinTypesSerializer+ObjectToStringRemap"
)]
impl crate::UnityEngine::ResourceManagement::Util::BuiltinTypesSerializer_BinaryStorageBuffer_ObjectToStringRemap {}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeSerializer_BinaryStorageBuffer_Data {
    pub assemblyId: u32,
    pub classId: u32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/TypeSerializer/Data";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+TypeSerializer+Data"
)]
impl crate::UnityEngine::ResourceManagement::Util::TypeSerializer_BinaryStorageBuffer_Data {}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
#[repr(C)]
#[derive(Debug)]
pub struct Writer_BinaryStorageBuffer_Chunk {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub position: u32,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/Writer/Chunk";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
impl crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+Chunk")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_Chunk {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Writer_BinaryStorageBuffer_StringParts {
    pub str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub dataSize: u32,
    pub isUnicode: bool,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "BinaryStorageBuffer/Writer/StringParts";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Util+BinaryStorageBuffer+Writer+StringParts"
)]
impl crate::UnityEngine::ResourceManagement::Util::Writer_BinaryStorageBuffer_StringParts {}
