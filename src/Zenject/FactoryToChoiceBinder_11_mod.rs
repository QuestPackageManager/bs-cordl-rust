#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryToChoiceBinder_11<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TParam8: quest_hook::libil2cpp::Type,
    TParam9: quest_hook::libil2cpp::Type,
    TParam10: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactoryFromBinder_11<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
    >,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TParam4: std::marker::PhantomData<TParam4>,
    __cordl_phantom_TParam5: std::marker::PhantomData<TParam5>,
    __cordl_phantom_TParam6: std::marker::PhantomData<TParam6>,
    __cordl_phantom_TParam7: std::marker::PhantomData<TParam7>,
    __cordl_phantom_TParam8: std::marker::PhantomData<TParam8>,
    __cordl_phantom_TParam9: std::marker::PhantomData<TParam9>,
    __cordl_phantom_TParam10: std::marker::PhantomData<TParam10>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryToChoiceBinder_11 < TParam1,
    TParam2, TParam3, TParam4, TParam5, TParam6, TParam7, TParam8, TParam9, TParam10,
    TContract > => "Zenject"."FactoryToChoiceBinder`11" < TParam1, TParam2, TParam3,
    TParam4, TParam5, TParam6, TParam7, TParam8, TParam9, TParam10, TContract >
);
#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TParam8: quest_hook::libil2cpp::Type,
    TParam9: quest_hook::libil2cpp::Type,
    TParam10: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::FactoryToChoiceBinder_11<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
    TParam7,
    TParam8,
    TParam9,
    TParam10,
    TContract,
> {
    type Target = crate::Zenject::FactoryFromBinder_11<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TParam8: quest_hook::libil2cpp::Type,
    TParam9: quest_hook::libil2cpp::Type,
    TParam10: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::FactoryToChoiceBinder_11<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
    TParam7,
    TParam8,
    TParam9,
    TParam10,
    TContract,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TParam8: quest_hook::libil2cpp::Type,
    TParam9: quest_hook::libil2cpp::Type,
    TParam10: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryToChoiceBinder_11<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
    TParam7,
    TParam8,
    TParam9,
    TParam10,
    TContract,
> {
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
    pub fn To<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryFromBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TConcrete,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryFromBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TConcrete,
        > = __cordl_object.invoke("To", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryFromBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryFromBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        > = __cordl_object.invoke("ToSelf", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
#[cfg(feature = "Zenject+FactoryToChoiceBinder_11")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TParam8: quest_hook::libil2cpp::Type,
    TParam9: quest_hook::libil2cpp::Type,
    TParam10: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryToChoiceBinder_11<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
    TParam7,
    TParam8,
    TParam9,
    TParam10,
    TContract,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}