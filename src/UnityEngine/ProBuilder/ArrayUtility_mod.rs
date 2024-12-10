#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ArrayUtility =>
    "UnityEngine.ProBuilder"."ArrayUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ArrayUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl crate::UnityEngine::ProBuilder::ArrayUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
    pub type SearchRange = crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange;
    #[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+__c__23_1")]
    pub type __c__23_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::ProBuilder::ArrayUtility___c__23_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+__c__DisplayClass21_0_2")]
    pub type __c__DisplayClass21_0_2<
        TSource: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
    > = crate::UnityEngine::ProBuilder::ArrayUtility___c__DisplayClass21_0_2<
        TSource,
        TKey,
    >;
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ArrayUtility_SearchRange {
    pub begin: i32,
    pub end: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::ArrayUtility_SearchRange => "UnityEngine.ProBuilder"
    ."ArrayUtility/SearchRange"
);
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
impl crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    pub fn Center(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Center",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        begin: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (begin, end),
        )?;
        Ok(__cordl_ret.into())
    }
}
