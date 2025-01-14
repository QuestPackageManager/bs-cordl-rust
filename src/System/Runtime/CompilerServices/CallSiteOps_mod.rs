#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
#[repr(C)]
#[derive(Debug)]
pub struct CallSiteOps {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::CallSiteOps {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "CallSiteOps";
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
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::CallSiteOps {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::CallSiteOps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
impl crate::System::Runtime::CompilerServices::CallSiteOps {
    pub fn AddRule<T>(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
        rule: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::CallSite_1<T>,
                    >,
                    T,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddRule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddRule", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (site, rule))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Bind<T>(
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::CallSiteBinder,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::CallSite_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                T,
                3usize,
            >("Bind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Bind", 3usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (binder, site, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearMatch(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClearMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearMatch", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (site))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMatchmaker<T>(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite_1<T>,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite_1<T>,
                >,
                1usize,
            >("CreateMatchmaker")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateMatchmaker", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        > = unsafe { method.invoke_unchecked((), (site)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedRules<T>(
        cache: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::RuleCache_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::RuleCache_1<T>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                1usize,
            >("GetCachedRules")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCachedRules", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { method.invoke_unchecked((), (cache)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMatch(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite,
                >),
                bool,
                1usize,
            >("GetMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMatch", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (site)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRuleCache<T>(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::RuleCache_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite_1<T>,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::RuleCache_1<T>,
                >,
                1usize,
            >("GetRuleCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRuleCache", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::RuleCache_1<T>,
        > = unsafe { method.invoke_unchecked((), (site)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRules<T>(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite_1<T>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                1usize,
            >("GetRules")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRules", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = unsafe { method.invoke_unchecked((), (site)) };
        Ok(__cordl_ret.into())
    }
    pub fn MoveRule<T>(
        cache: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::RuleCache_1<T>,
        >,
        rule: T,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::RuleCache_1<T>,
                    >,
                    T,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("MoveRule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MoveRule", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cache, rule, i))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNotMatched(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::CallSite,
                >),
                bool,
                1usize,
            >("SetNotMatched")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetNotMatched", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (site)) };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRules<T>(
        _cordl_this: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        >,
        matched: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::CallSite_1<T>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UpdateRules")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateRules", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_this, matched))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CallSiteOps {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
