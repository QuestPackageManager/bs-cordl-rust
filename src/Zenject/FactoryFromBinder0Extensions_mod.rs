#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder0Extensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder0Extensions =>
    "Zenject"."FactoryFromBinder0Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder0Extensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder0Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl crate::Zenject::FactoryFromBinder0Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__1_1")]
    pub type __c__1_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__1_1<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__3_1")]
    pub type __c__3_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__3_1<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__5_2")]
    pub type __c__5_2<
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder0Extensions___c__5_2<TContract, TMemoryPool>;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__DisplayClass0_0_2")]
    pub type __c__DisplayClass0_0_2<
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder0Extensions___c__DisplayClass0_0_2<
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__DisplayClass6_0_1")]
    pub type __c__DisplayClass6_0_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__DisplayClass6_0_1<
        TContract,
    >;
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder0Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
