#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder1Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder1Extensions =>
    "Zenject"."FactoryFromBinder1Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder1Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder1Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl crate::Zenject::FactoryFromBinder1Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder1Extensions+__c__1_2")]
    pub type __c__1_2<
        TParam1: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder1Extensions___c__1_2<TParam1, TContract>;
    #[cfg(feature = "Zenject+FactoryFromBinder1Extensions+__c__3_2")]
    pub type __c__3_2<
        TParam1: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder1Extensions___c__3_2<TParam1, TContract>;
    #[cfg(feature = "Zenject+FactoryFromBinder1Extensions+__c__5_3")]
    pub type __c__5_3<
        TParam1: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder1Extensions___c__5_3<
        TParam1,
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder1Extensions+__c__DisplayClass0_0_2")]
    pub type __c__DisplayClass0_0_2<
        TParam1: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder1Extensions___c__DisplayClass0_0_2<
        TParam1,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder1Extensions+__c__DisplayClass6_0_3")]
    pub type __c__DisplayClass6_0_3<
        TParam1: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder1Extensions___c__DisplayClass6_0_3<
        TParam1,
        TContract,
        TMemoryPool,
    >;
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder1Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
