#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
#[repr(C)]
#[derive(Debug)]
pub struct CallSiteOps {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSiteOps")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::CallSiteOps
    => "System.Runtime.CompilerServices"."CallSiteOps"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRule", (site, rule))?;
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
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Bind", (binder, site, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearMatch(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearMatch", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMatchmaker", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedRules", (cache))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMatch(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMatch", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::RuleCache_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRuleCache", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetRules", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveRule", (cache, rule, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNotMatched(
        site: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetNotMatched", (site))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateRules", (_cordl_this, matched))?;
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
