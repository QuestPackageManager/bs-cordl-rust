#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryArgumentsToChoiceBinder_3<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryArgumentsToChoiceBinder_3 <
    TParam1, TParam2, TContract > => "Zenject"."FactoryArgumentsToChoiceBinder`3" <
    TParam1, TParam2, TContract >
);
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::FactoryArgumentsToChoiceBinder_3<TParam1, TParam2, TContract> {
    type Target = crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::FactoryArgumentsToChoiceBinder_3<TParam1, TParam2, TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryArgumentsToChoiceBinder_3<TParam1, TParam2, TContract> {
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, factoryBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn WithFactoryArgumentsExplicit(
        &mut self,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object.invoke("WithFactoryArgumentsExplicit", (extraArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_Il2CppArray6(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object.invoke("WithFactoryArguments", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_T0<T>(
        &mut self,
        param: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object.invoke("WithFactoryArguments", (param))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_TFactoryParam1_TFactoryParam2_1<
        TFactoryParam1,
        TFactoryParam2,
    >(
        &mut self,
        param1: TFactoryParam1,
        param2: TFactoryParam2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFactoryParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object.invoke("WithFactoryArguments", (param1, param2))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_TFactoryParam1_TFactoryParam2_TFactoryParam3_2<
        TFactoryParam1,
        TFactoryParam2,
        TFactoryParam3,
    >(
        &mut self,
        param1: TFactoryParam1,
        param2: TFactoryParam2,
        param3: TFactoryParam3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFactoryParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object.invoke("WithFactoryArguments", (param1, param2, param3))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_TFactoryParam1_TFactoryParam2_TFactoryParam3_TFactoryParam4_3<
        TFactoryParam1,
        TFactoryParam2,
        TFactoryParam3,
        TFactoryParam4,
    >(
        &mut self,
        param1: TFactoryParam1,
        param2: TFactoryParam2,
        param3: TFactoryParam3,
        param4: TFactoryParam4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFactoryParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2, param3, param4))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_TFactoryParam1_TFactoryParam2_TFactoryParam3_TFactoryParam4_TFactoryParam5_4<
        TFactoryParam1,
        TFactoryParam2,
        TFactoryParam3,
        TFactoryParam4,
        TFactoryParam5,
    >(
        &mut self,
        param1: TFactoryParam1,
        param2: TFactoryParam2,
        param3: TFactoryParam3,
        param4: TFactoryParam4,
        param5: TFactoryParam5,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFactoryParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2, param3, param4, param5))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFactoryArguments_TFactoryParam1_TFactoryParam2_TFactoryParam3_TFactoryParam4_TFactoryParam5_TFactoryParam6_5<
        TFactoryParam1,
        TFactoryParam2,
        TFactoryParam3,
        TFactoryParam4,
        TFactoryParam5,
        TFactoryParam6,
    >(
        &mut self,
        param1: TFactoryParam1,
        param2: TFactoryParam2,
        param3: TFactoryParam3,
        param4: TFactoryParam4,
        param5: TFactoryParam5,
        param6: TFactoryParam6,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFactoryParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryToChoiceBinder_3<TParam1, TParam2, TContract>,
        > = __cordl_object
            .invoke(
                "WithFactoryArguments",
                (param1, param2, param3, param4, param5, param6),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, factoryBindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryArgumentsToChoiceBinder_3<TParam1, TParam2, TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
