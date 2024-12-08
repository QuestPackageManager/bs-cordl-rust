#[cfg(feature = "System+Linq+IdentityFunction_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IdentityFunction_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+IdentityFunction_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::IdentityFunction_1 < TElement > =>
    "System.Linq"."IdentityFunction`1" < TElement >
);
#[cfg(feature = "System+Linq+IdentityFunction_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::IdentityFunction_1<TElement> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IdentityFunction_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::IdentityFunction_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IdentityFunction_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::IdentityFunction_1<TElement> {
    #[cfg(feature = "System+Linq+IdentityFunction_1+__c")]
    pub type __c = crate::System::Linq::IdentityFunction_1___c<TElement>;
}
#[cfg(feature = "System+Linq+IdentityFunction_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::IdentityFunction_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
