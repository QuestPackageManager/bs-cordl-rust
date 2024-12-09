#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryArgumentsToChoiceBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryArgumentsToChoiceBinder_1 <
    TContract > => "Zenject"."FactoryArgumentsToChoiceBinder`1" < TContract >
);
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract> {
    type Target = crate::Zenject::FactoryToChoiceBinder_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract> {
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, factoryBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithFactoryArgumentsExplicit(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArgumentsExplicit", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_Il2CppArray6(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (args))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_T0<T>(
        &mut self,
        param: T,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (param))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_TParam1_TParam2_1<TParam1, TParam2>(
        &mut self,
        param1: TParam1,
        param2: TParam2,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_TParam1_TParam2_TParam3_2<TParam1, TParam2, TParam3>(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2, param3))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_TParam1_TParam2_TParam3_TParam4_3<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
    >(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2, param3, param4))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_TParam1_TParam2_TParam3_TParam4_TParam5_4<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
    >(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
        param5: TParam5,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke("WithFactoryArguments", (param1, param2, param3, param4, param5))?;
        Ok(__cordl_ret)
    }
    pub fn WithFactoryArguments_TParam1_TParam2_TParam3_TParam4_TParam5_TParam6_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
    >(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
        param5: TParam5,
        param6: TParam6,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceBinder_1<TContract> = __cordl_object
            .invoke(
                "WithFactoryArguments",
                (param1, param2, param3, param4, param5, param6),
            )?;
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
#[cfg(feature = "Zenject+FactoryArgumentsToChoiceBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
