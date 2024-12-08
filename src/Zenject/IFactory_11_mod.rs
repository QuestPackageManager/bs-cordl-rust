#[cfg(feature = "Zenject+IFactory_11")]
#[repr(C)]
#[derive(Debug)]
pub struct IFactory_11<
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
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+IFactory_11")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IFactory_11 < TParam1, TParam2,
    TParam3, TParam4, TParam5, TParam6, TParam7, TParam8, TParam9, TParam10, TValue > =>
    "Zenject"."IFactory`11" < TParam1, TParam2, TParam3, TParam4, TParam5, TParam6,
    TParam7, TParam8, TParam9, TParam10, TValue >
);
#[cfg(feature = "Zenject+IFactory_11")]
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
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::IFactory_11<
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
    TValue,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IFactory_11")]
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
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::IFactory_11<
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
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IFactory_11")]
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
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::IFactory_11<
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
    TValue,
> {
    pub fn Create(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
        param5: TParam5,
        param6: TParam6,
        param7: TParam7,
        param8: TParam8,
        param9: TParam9,
        param10: TParam10,
    ) -> quest_hook::libil2cpp::Result<TValue>
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
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke(
                "Create",
                (
                    param1,
                    param2,
                    param3,
                    param4,
                    param5,
                    param6,
                    param7,
                    param8,
                    param9,
                    param10,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IFactory_11")]
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
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::IFactory_11<
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
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
