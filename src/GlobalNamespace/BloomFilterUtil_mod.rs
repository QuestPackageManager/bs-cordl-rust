#[cfg(feature = "BloomFilterUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFilterUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BloomFilterUtil")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BloomFilterUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomFilterUtil";
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
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFilterUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFilterUtil {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl crate::GlobalNamespace::BloomFilterUtil {
    pub fn AddBloomFilterEntry<T>(
        bitMask: T,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
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
                        (
                            T,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        T,
                        4usize,
                    >("AddBloomFilterEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddBloomFilterEntry", 4usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info
                .invoke_unchecked((), (bitMask, value, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBloomFilterEntryHash<T>(
        bitMask: T,
        hash: u32,
        hashCount: i32,
        hashBits: i32,
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
                        (T, u32, i32, i32),
                        T,
                        4usize,
                    >("AddBloomFilterEntryHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddBloomFilterEntryHash", 4usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (bitMask, hash, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsBloomFilterEntry<T>(
        bitMask: T,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
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
                        (
                            T,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        bool,
                        4usize,
                    >("ContainsBloomFilterEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsBloomFilterEntry", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (bitMask, value, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsBloomFilterEntryHash<T>(
        bitMask: T,
        hash: u32,
        hashCount: i32,
        hashBits: i32,
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
                        (T, u32, i32, i32),
                        bool,
                        4usize,
                    >("ContainsBloomFilterEntryHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsBloomFilterEntryHash", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (bitMask, hash, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_IEnumerable_1_1<T>(
        strings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        hashCount: i32,
        hashBits: i32,
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
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        T,
                        3usize,
                    >("ToBloomFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBloomFilter", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (strings, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_IEnumerable_1_2<T>(
        hashes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<u32>,
        >,
        hashCount: i32,
        hashBits: i32,
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
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<u32>,
                            >,
                            i32,
                            i32,
                        ),
                        T,
                        3usize,
                    >("ToBloomFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBloomFilter", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (hashes, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_Il2CppString0<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
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
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        T,
                        3usize,
                    >("ToBloomFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBloomFilter", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (value, hashCount, hashBits))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BloomFilterUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
