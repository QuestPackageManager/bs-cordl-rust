#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder2Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder2Extensions =>
    "Zenject"."FactoryFromBinder2Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder2Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder2Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl crate::Zenject::FactoryFromBinder2Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder2Extensions+__c__1_3")]
    pub type __c__1_3<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder2Extensions___c__1_3<
        TParam1,
        TParam2,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder2Extensions+__c__3_3")]
    pub type __c__3_3<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder2Extensions___c__3_3<
        TParam1,
        TParam2,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder2Extensions+__c__5_4")]
    pub type __c__5_4<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder2Extensions___c__5_4<
        TParam1,
        TParam2,
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder2Extensions+__c__DisplayClass0_0_3")]
    pub type __c__DisplayClass0_0_3<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder2Extensions___c__DisplayClass0_0_3<
        TParam1,
        TParam2,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder2Extensions+__c__DisplayClass6_0_4")]
    pub type __c__DisplayClass6_0_4<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder2Extensions___c__DisplayClass6_0_4<
        TParam1,
        TParam2,
        TContract,
        TMemoryPool,
    >;
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder2Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
