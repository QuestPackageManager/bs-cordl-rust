#[cfg(feature = "ModestTree+LinqExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LinqExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+LinqExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::LinqExtensions => "ModestTree"
    ."LinqExtensions"
);
#[cfg(feature = "ModestTree+LinqExtensions")]
impl std::ops::Deref for crate::ModestTree::LinqExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl std::ops::DerefMut for crate::ModestTree::LinqExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl crate::ModestTree::LinqExtensions {
    #[cfg(feature = "ModestTree+LinqExtensions+_Yield_d__0_1")]
    pub type _Yield_d__0_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions__Yield_d__0_1<
        T,
    >;
    #[cfg(feature = "ModestTree+LinqExtensions+__c__8_1")]
    pub type __c__8_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions___c__8_1<
        T,
    >;
    #[cfg(feature = "ModestTree+LinqExtensions+__c__DisplayClass10_0_1")]
    pub type __c__DisplayClass10_0_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions___c__DisplayClass10_0_1<
        T,
    >;
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::LinqExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
