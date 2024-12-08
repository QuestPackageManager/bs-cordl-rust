#[cfg(feature = "ModestTree+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+TypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::TypeExtensions => "ModestTree"
    ."TypeExtensions"
);
#[cfg(feature = "ModestTree+TypeExtensions")]
impl std::ops::Deref for crate::ModestTree::TypeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl std::ops::DerefMut for crate::ModestTree::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl crate::ModestTree::TypeExtensions {
    #[cfg(feature = "ModestTree+TypeExtensions+_GetParentTypes_d__28")]
    pub type _GetParentTypes_d__28 = crate::ModestTree::TypeExtensions__GetParentTypes_d__28;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::ModestTree::TypeExtensions___c__DisplayClass35_0;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass35_1")]
    pub type __c__DisplayClass35_1 = crate::ModestTree::TypeExtensions___c__DisplayClass35_1;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass39_0")]
    pub type __c__DisplayClass39_0 = crate::ModestTree::TypeExtensions___c__DisplayClass39_0;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass39_1")]
    pub type __c__DisplayClass39_1 = crate::ModestTree::TypeExtensions___c__DisplayClass39_1;
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
