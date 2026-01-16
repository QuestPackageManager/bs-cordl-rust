#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StreamingState_xxHash3_StreamingStateData {
    padding: quest_hook::libil2cpp::ValueTypePadding<541usize>,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/StreamingState/StreamingStateData";
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
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
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
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+StreamingState+StreamingStateData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+StreamingState+StreamingStateData")]
impl crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData {}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3")]
#[repr(C)]
#[derive(Debug)]
pub struct xxHash3 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::xxHash3 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3";
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
#[cfg(feature = "Unity+Collections+xxHash3")]
impl std::ops::Deref for crate::Unity::Collections::xxHash3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3")]
impl std::ops::DerefMut for crate::Unity::Collections::xxHash3 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3")]
impl crate::Unity::Collections::xxHash3 {
    pub const ACC_NB: i32 = 8i32;
    pub const BLOCK_LEN: i32 = 1024i32;
    pub const MIDSIZE_LASTOFFSET: i32 = 17i32;
    pub const MIDSIZE_MAX: i32 = 240i32;
    pub const MIDSIZE_STARTOFFSET: i32 = 3i32;
    pub const NB_ROUNDS: i32 = 16i32;
    pub const PRIME32_1: u32 = 2654435761u32;
    pub const PRIME32_2: u32 = 2246822519u32;
    pub const PRIME32_3: u32 = 3266489917u32;
    pub const PRIME32_5: u32 = 374761393u32;
    pub const PRIME64_1: u64 = 11400714785074694791u64;
    pub const PRIME64_2: u64 = 14029467366897019727u64;
    pub const PRIME64_3: u64 = 1609587929392839161u64;
    pub const PRIME64_4: u64 = 9650029242287828579u64;
    pub const PRIME64_5: u64 = 2870177450012600261u64;
    pub const SECRET_CONSUME_RATE: i32 = 8i32;
    pub const SECRET_KEY_MIN_SIZE: i32 = 136i32;
    pub const SECRET_KEY_SIZE: i32 = 192i32;
    pub const SECRET_LASTACC_START: i32 = 7i32;
    pub const SECRET_MERGEACCS_START: i32 = 11i32;
    pub const STRIPE_LEN: i32 = 64i32;
    #[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall")]
    pub type Hash128Long_00000A7A_BurstDirectCall = crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall;
    #[cfg(
        feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate"
    )]
    pub type Hash128Long_00000A7A_PostfixBurstDelegate = crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate;
    #[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall")]
    pub type Hash64Long_00000A73_BurstDirectCall = crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall;
    #[cfg(
        feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate"
    )]
    pub type Hash64Long_00000A73_PostfixBurstDelegate = crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate;
    #[cfg(feature = "Unity+Collections+xxHash3+StreamingState")]
    pub type StreamingState = crate::Unity::Collections::xxHash3_StreamingState;
    #[cfg(feature = "Unity+Collections+xxHash3+ulong2")]
    pub type ulong2 = crate::Unity::Collections::xxHash3_ulong2;
    pub fn Avalanche(h64: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), u64, 1usize>("Avalanche")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Avalanche", 1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (h64))? };
        Ok(__cordl_ret.into())
    }
    pub fn AvalancheH64(h64: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), u64, 1usize>("AvalancheH64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AvalancheH64", 1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (h64))? };
        Ok(__cordl_ret.into())
    }
    pub fn Avx2Accumulate(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nbStripes: i64,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Avx2Accumulate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Avx2Accumulate", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (acc, input, dest, secret, nbStripes, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Avx2Accumulate512(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Avx2Accumulate512")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Avx2Accumulate512", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (acc, input, dest, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Avx2HashLongInternalLoop(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Avx2HashLongInternalLoop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Avx2HashLongInternalLoop", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (acc, input, dest, length, secret, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Avx2ScrambleAcc(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Avx2ScrambleAcc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Avx2ScrambleAcc", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (acc, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultAccumulate(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nbStripes: i64,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DefaultAccumulate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultAccumulate", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (acc, input, dest, secret, nbStripes, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultAccumulate512(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DefaultAccumulate512")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultAccumulate512", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (acc, input, dest, secret, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultHashLongInternalLoop(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DefaultHashLongInternalLoop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultHashLongInternalLoop", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (acc, input, dest, length, secret, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultScrambleAcc(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("DefaultScrambleAcc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultScrambleAcc", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (acc, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncodeSecretKey(
        dst: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EncodeSecretKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncodeSecretKey", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dst, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Internal(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Hash128Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Internal", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, dest, length, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len0To16(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len0To16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len0To16", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, length, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len129To240(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len129To240")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len129To240", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, length, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len17To128(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len17To128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len17To128", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, length, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len1To3(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len1To3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len1To3", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, length, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len4To8(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len4To8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len4To8", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (input, len, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Len9To16(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Len9To16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Len9To16", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (input, len, secret, seed, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Long(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Long")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Long", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, dest, length, secret, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128Long_BurstManaged(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Hash128Long$BurstManaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash128Long$BurstManaged", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, dest, length, secret, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128_ByRefMut1<T>(
        input: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        crate::Unity::Mathematics::uint4,
                        1usize,
                    >("Hash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash128",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128_Il2CppObject_Il2CppObject_i64_2(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                        ),
                        crate::Unity::Mathematics::uint4,
                        3usize,
                    >("Hash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash128",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, destination, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128_Il2CppObject_Il2CppObject_i64_u64_4(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            u64,
                        ),
                        crate::Unity::Mathematics::uint4,
                        4usize,
                    >("Hash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash128",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, destination, length, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128_Il2CppObject_i64_0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                        ),
                        crate::Unity::Mathematics::uint4,
                        2usize,
                    >("Hash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash128",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash128_Il2CppObject_i64_u64_3(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            u64,
                        ),
                        crate::Unity::Mathematics::uint4,
                        3usize,
                    >("Hash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash128",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Internal(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        5usize,
                    >("Hash64Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Internal", 5usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, dest, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len0To16(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len0To16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len0To16", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len129To240(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len129To240")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len129To240", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len17To128(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len17To128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len17To128", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len1To3(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len1To3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len1To3", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, len, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len4To8(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len4To8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len4To8", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Len9To16(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        4usize,
                    >("Hash64Len9To16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Len9To16", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Long(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u64,
                        4usize,
                    >("Hash64Long")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Long", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, dest, length, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64Long_BurstManaged(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u64,
                        4usize,
                    >("Hash64Long$BurstManaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Hash64Long$BurstManaged", 4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, dest, length, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64_ByRefMut1<T>(
        input: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        crate::Unity::Mathematics::uint2,
                        1usize,
                    >("Hash64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash64",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64_Il2CppObject_i64_0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                        ),
                        crate::Unity::Mathematics::uint2,
                        2usize,
                    >("Hash64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash64",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Hash64_Il2CppObject_i64_u64_2(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            u64,
                        ),
                        crate::Unity::Mathematics::uint2,
                        3usize,
                    >("Hash64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Hash64",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, length, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MergeAcc(
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        3usize,
                    >("MergeAcc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MergeAcc", 3usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (acc, secret, start))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mix16(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        u64,
                        3usize,
                    >("Mix16")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Mix16",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mix2Acc(
        acc0: u64,
        acc1: u64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u64,
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u64,
                        3usize,
                    >("Mix2Acc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Mix2Acc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (acc0, acc1, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mix32(
        acc: crate::Unity::Collections::xxHash3_ulong2,
        input_1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        input_2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::xxHash3_ulong2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::xxHash3_ulong2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        crate::Unity::Collections::xxHash3_ulong2,
                        5usize,
                    >("Mix32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Mix32",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::xxHash3_ulong2 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (acc, input_1, input_2, secret, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul128Fold64(lhs: u64, rhs: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64), u64, 2usize>("Mul128Fold64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Mul128Fold64", 2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul32To64(x: u32, y: u32) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, u32), u64, 2usize>("Mul32To64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Mul32To64", 2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read32LE_Il2CppObject0(
        addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        u32,
                        1usize,
                    >("Read32LE")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Read32LE", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (addr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read32LE_u32_1(
        addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Read32LE")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Read32LE", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (addr, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read64LE(
        addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        u64,
                        1usize,
                    >("Read64LE")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Read64LE", 1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (addr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotL32(x: u32, r: i32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, i32), u32, 2usize>("RotL32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RotL32",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotL64(x: u64, r: i32) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, i32), u64, 2usize>("RotL64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RotL64",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Swap32(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("Swap32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Swap32",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn Swap64(x: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), u64, 1usize>("Swap64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Swap64",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUint2(
        u: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64),
                        crate::Unity::Mathematics::uint2,
                        1usize,
                    >("ToUint2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToUint2",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked((), (u))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUint4(
        ul0: u64,
        ul1: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, u64),
                        crate::Unity::Mathematics::uint4,
                        2usize,
                    >("ToUint4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToUint4",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked((), (ul0, ul1))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write64LE(
        addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u64,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Write64LE")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Write64LE", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (addr, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn XorShift64(v64: u64, shift: i32) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, i32), u64, 2usize>("XorShift64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "XorShift64", 2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (v64, shift))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn rrmxmx(h64: u64, length: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64), u64, 2usize>("rrmxmx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rrmxmx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (h64, length))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::xxHash3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall"
)]
#[repr(C)]
#[derive(Debug)]
pub struct xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/Hash128Long_00000A7A$BurstDirectCall";
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
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall")]
impl std::ops::Deref
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall")]
impl std::ops::DerefMut
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall")]
impl crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    pub fn GetFunctionPointer() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::IntPtr,
                        0usize,
                    >("GetFunctionPointer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFunctionPointer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFunctionPointerDiscard(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetFunctionPointerDiscard")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFunctionPointerDiscard", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (input, dest, length, secret, result))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_BurstDirectCall"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_BurstDirectCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/Hash128Long_00000A7A$PostfixBurstDelegate";
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
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate")]
impl std::ops::Deref
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate")]
impl std::ops::DerefMut
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate")]
impl crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    pub fn BeginInvoke(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            crate::System::AsyncCallback,
        >,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
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
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        7usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        input,
                        dest,
                        length,
                        secret,
                        result,
                        _cordl_fixed_empty_name_whitespace,
                        _cordl_fixed_empty_name_whitespace,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            crate::System::IAsyncResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        result: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::uint4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::uint4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (input, dest, length, secret, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        _cordl_fixed_empty_name_whitespace: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_fixed_empty_name_whitespace, _cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        _cordl_fixed_empty_name_whitespace: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
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
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        _cordl_fixed_empty_name_whitespace,
                        _cordl_fixed_empty_name_whitespace,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash128Long_00000A7A_PostfixBurstDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::xxHash3_Hash128Long_00000A7A_PostfixBurstDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall"
)]
#[repr(C)]
#[derive(Debug)]
pub struct xxHash3_Hash64Long_00000A73_BurstDirectCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/Hash64Long_00000A73$BurstDirectCall";
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
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall")]
impl std::ops::Deref
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall")]
impl std::ops::DerefMut
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall")]
impl crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall {
    pub fn GetFunctionPointer() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::IntPtr,
                        0usize,
                    >("GetFunctionPointer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFunctionPointer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFunctionPointerDiscard(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetFunctionPointerDiscard")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFunctionPointerDiscard", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u64,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (input, dest, length, secret))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_BurstDirectCall"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_BurstDirectCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/Hash64Long_00000A73$PostfixBurstDelegate";
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
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate")]
impl std::ops::Deref
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate")]
impl std::ops::DerefMut
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate")]
impl crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    pub fn BeginInvoke(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            crate::System::AsyncCallback,
        >,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
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
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        6usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        input,
                        dest,
                        length,
                        secret,
                        _cordl_fixed_empty_name_whitespace,
                        _cordl_fixed_empty_name_whitespace,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            crate::System::IAsyncResult,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        u64,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
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
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        u64,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked(self, (input, dest, length, secret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        _cordl_fixed_empty_name_whitespace: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_fixed_empty_name_whitespace, _cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        _cordl_fixed_empty_name_whitespace: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
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
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        _cordl_fixed_empty_name_whitespace,
                        _cordl_fixed_empty_name_whitespace,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+xxHash3+Hash64Long_00000A73_PostfixBurstDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::xxHash3_Hash64Long_00000A73_PostfixBurstDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct xxHash3_StreamingState {
    pub State: crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData,
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::xxHash3_StreamingState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/StreamingState";
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
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Collections::xxHash3_StreamingState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Collections::xxHash3_StreamingState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Collections::xxHash3_StreamingState {
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
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Collections::xxHash3_StreamingState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+StreamingState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::xxHash3_StreamingState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+StreamingState")]
impl crate::Unity::Collections::xxHash3_StreamingState {
    #[cfg(feature = "Unity+Collections+xxHash3+StreamingState+StreamingStateData")]
    pub type StreamingStateData = crate::Unity::Collections::StreamingState_xxHash3_StreamingStateData;
    pub fn CheckKeySize(
        &mut self,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckKeySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckKeySize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConsumeStripes(
        &mut self,
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nbStripesSoFar: quest_hook::libil2cpp::ByRefMut<i32>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        totalStripes: i64,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("ConsumeStripes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConsumeStripes", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (acc, nbStripesSoFar, input, totalStripes, secret, isHash64),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DigestHash128(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::uint4,
                        0usize,
                    >("DigestHash128")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DigestHash128", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DigestHash64(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::uint2,
                        0usize,
                    >("DigestHash64")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DigestHash64", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DigestLong(
        &mut self,
        acc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isHash64: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DigestLong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DigestLong", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (acc, secret, isHash64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
        isHash64: bool,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (isHash64, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update_ByRefMut1<T>(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<T>,
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
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update_Il2CppObject_i32_0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (input, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        isHash64: bool,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, u64),
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
            cordl_method_info.invoke_unchecked(self, (isHash64, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Acc(
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
                    >("get_Acc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_Acc",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Buffer(
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
                    >("get_Buffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Buffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SecretKey(
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
                    >("get_SecretKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SecretKey", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct xxHash3_ulong2 {
    pub x: u64,
    pub y: u64,
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::xxHash3_ulong2 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "xxHash3/ulong2";
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
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Collections::xxHash3_ulong2 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Collections::xxHash3_ulong2 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Collections::xxHash3_ulong2 {
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
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::xxHash3_ulong2 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+xxHash3+ulong2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::xxHash3_ulong2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+xxHash3+ulong2")]
impl crate::Unity::Collections::xxHash3_ulong2 {
    pub fn _ctor(
        &mut self,
        x: u64,
        y: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u64),
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
            cordl_method_info.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
}
