#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_TestHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_TestHelpers";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TestHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TestHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl crate::HoudiniEngineUnity::HEU_TestHelpers {
    #[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
    pub type RequireClass_1<T: quest_hook::libil2cpp::Type> = crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<
        T,
    >;
    #[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
    pub type RequireStruct_1<T: quest_hook::libil2cpp::Type> = crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<
        T,
    >;
    pub fn AssertTrueLogEquivalent_GameObject_GameObject1(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_IEquivableWrapperClass_1_IEquivableWrapperClass_1_3<
        T,
    >(
        a: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_IEquivable_1_IEquivable_1_2<T>(
        a: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
        b: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivable_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivable_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Il2CppArray_Il2CppArray10(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Il2CppArray_Il2CppArray11<T>(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivable_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivable_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Il2CppArray_Il2CppArray12<T>(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                >,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                >,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Il2CppArray_Il2CppArray9<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Il2CppString_Il2CppString4(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_List_1_List_1_7<T>(
        a: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivable_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivable_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_List_1_List_1_8<T>(
        a: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                >,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                >,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        8usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (a, b, result, header, subject, optional1, optional2, optional3),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_List_1_List_1_HEU_TestHelpers_RequireClass_1_6<T>(
        a: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        b: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T>,
                            >,
                        ),
                        bool,
                        9usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        a,
                        b,
                        result,
                        header,
                        subject,
                        optional1,
                        optional2,
                        optional3,
                        _cordl__,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_List_1_List_1_HEU_TestHelpers_RequireStruct_1_5<T>(
        a: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        b: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<
                                    T,
                                >,
                            >,
                        ),
                        bool,
                        9usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        a,
                        b,
                        result,
                        header,
                        subject,
                        optional1,
                        optional2,
                        optional3,
                        _cordl__,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_T_T_HEU_TestHelpers_RequireStruct_1_0<T>(
        a: T,
        b: T,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            T,
                            T,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<
                                    T,
                                >,
                            >,
                        ),
                        bool,
                        9usize,
                    >("AssertTrueLogEquivalent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AssertTrueLogEquivalent", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        a,
                        b,
                        result,
                        header,
                        subject,
                        optional1,
                        optional2,
                        optional3,
                        _cordl__,
                    ),
                )?
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
    pub fn PrintTestLogAndSetResult(
        expression: bool,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("PrintTestLogAndSetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PrintTestLogAndSetResult", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        expression,
                        result,
                        header,
                        subject,
                        optional1,
                        optional2,
                        optional3,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_GameObject_GameObject1(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_IEquivableWrapperClass_1_IEquivableWrapperClass_1_3<T>(
        a: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
        >,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_IEquivable_1_IEquivable_1_2<T>(
        a: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
        b: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivable_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::IEquivable_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Il2CppArray_Il2CppArray5<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Il2CppString_Il2CppString6(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_List_1_List_1_4<T>(
        a: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        b: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_T_T0<T>(
        a: T,
        b: T,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                            T,
                            T,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldBeTested")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldBeTested", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, bResult, header, subject))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TestOutputObjectEquivalence(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        ),
                        bool,
                        2usize,
                    >("TestOutputObjectEquivalence")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TestOutputObjectEquivalence", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b))? };
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_TestHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers_RequireClass_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_TestHelpers/RequireClass`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "HoudiniEngineUnity",
                        "HEU_TestHelpers/RequireClass`1",
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers_RequireStruct_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_TestHelpers/RequireStruct`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "HoudiniEngineUnity",
                        "HEU_TestHelpers/RequireStruct`1",
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
