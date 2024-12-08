#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DecoratorToChoiceFromBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _bindContainer: *mut crate::Zenject::DiContainer,
    pub _bindInfo: *mut crate::Zenject::BindInfo,
    pub _factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DecoratorToChoiceFromBinder_1 <
    TContract > => "Zenject"."DecoratorToChoiceFromBinder`1" < TContract >
);
#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::DecoratorToChoiceFromBinder_1<TContract> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::DecoratorToChoiceFromBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::DecoratorToChoiceFromBinder_1<TContract> {
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, factoryBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn With<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryFromBinder_2<TContract, TConcrete>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryFromBinder_2<
            TContract,
            TConcrete,
        > = __cordl_object.invoke("With", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, factoryBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DecoratorToChoiceFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::DecoratorToChoiceFromBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}