#[cfg(feature = "System+Threading+LazyInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyInitializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+LazyInitializer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::LazyInitializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "LazyInitializer";
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
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::Deref for crate::System::Threading::LazyInitializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::DerefMut for crate::System::Threading::LazyInitializer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl crate::System::Threading::LazyInitializer {
    pub fn EnsureInitializedCore_ByRefMut0<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("EnsureInitializedCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitializedCore", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (target))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_ByRefMut_ByRefMut_Func_1_2<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        initialized: quest_hook::libil2cpp::ByRefMut<bool>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        4usize,
                    >("EnsureInitializedCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitializedCore", 4usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, initialized, syncLock, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_ByRefMut_Func_1_3<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        3usize,
                    >("EnsureInitializedCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitializedCore", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, syncLock, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_Func_1_1<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        2usize,
                    >("EnsureInitializedCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitializedCore", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut0<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("EnsureInitialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitialized", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (target))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut_ByRefMut_Func_1_2<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        initialized: quest_hook::libil2cpp::ByRefMut<bool>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        4usize,
                    >("EnsureInitialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitialized", 4usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, initialized, syncLock, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut_Func_1_3<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        3usize,
                    >("EnsureInitialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitialized", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, syncLock, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_Func_1_1<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        valueFactory: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                        ),
                        T,
                        2usize,
                    >("EnsureInitialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureInitialized", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (target, valueFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLockInitialized(
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("EnsureLockInitialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureLockInitialized", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (syncLock))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::LazyInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
