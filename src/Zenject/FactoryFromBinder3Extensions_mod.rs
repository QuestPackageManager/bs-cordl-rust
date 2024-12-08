#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder3Extensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder3Extensions =>
    "Zenject"."FactoryFromBinder3Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder3Extensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder3Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
impl crate::Zenject::FactoryFromBinder3Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder3Extensions+__c__1_4")]
    pub type __c__1_4<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder3Extensions___c__1_4<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder3Extensions+__c__3_4")]
    pub type __c__3_4<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder3Extensions___c__3_4<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder3Extensions+__c__5_5")]
    pub type __c__5_5<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder3Extensions___c__5_5<
        TParam1,
        TParam2,
        TParam3,
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder3Extensions+__c__DisplayClass0_0_4")]
    pub type __c__DisplayClass0_0_4<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder3Extensions___c__DisplayClass0_0_4<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder3Extensions+__c__DisplayClass6_0_5")]
    pub type __c__DisplayClass6_0_5<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder3Extensions___c__DisplayClass6_0_5<
        TParam1,
        TParam2,
        TParam3,
        TContract,
        TMemoryPool,
    >;
}
#[cfg(feature = "Zenject+FactoryFromBinder3Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder3Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
