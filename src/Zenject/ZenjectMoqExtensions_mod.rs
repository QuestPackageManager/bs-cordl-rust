#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectMoqExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectMoqExtensions => "Zenject"
    ."ZenjectMoqExtensions"
);
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl std::ops::Deref for crate::Zenject::ZenjectMoqExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl std::ops::DerefMut for crate::Zenject::ZenjectMoqExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl crate::Zenject::ZenjectMoqExtensions {
    pub fn FromMock_FactoryFromBinder_1_1<TContract>(
        binder: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryFromBinder_1<TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromMock", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMock_FromBinderGeneric_1_0<TContract>(
        binder: quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderGeneric_1<TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromMock", (binder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectMoqExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
