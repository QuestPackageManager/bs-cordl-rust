#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolExpandBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
    pub _MemoryPoolBindInfo_k__BackingField: *mut crate::Zenject::MemoryPoolBindInfo,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MemoryPoolExpandBinder_1 < TContract >
    => "Zenject"."MemoryPoolExpandBinder`1" < TContract >
);
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MemoryPoolExpandBinder_1<TContract> {
    type Target = crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MemoryPoolExpandBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::MemoryPoolExpandBinder_1<TContract> {
    pub fn get_MemoryPoolBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::MemoryPoolBindInfo>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolBindInfo = __cordl_object
            .invoke("get_MemoryPoolBindInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExpandByDoubling(
        &mut self,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryArgumentsToChoiceBinder_1<
            TContract,
        > = __cordl_object.invoke("ExpandByDoubling", (showExpandWarning))?;
        Ok(__cordl_ret)
    }
    pub fn set_MemoryPoolBindInfo(
        &mut self,
        value: *mut crate::Zenject::MemoryPoolBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MemoryPoolBindInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandByOneAtATime(
        &mut self,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryArgumentsToChoiceBinder_1<
            TContract,
        > = __cordl_object.invoke("ExpandByOneAtATime", (showExpandWarning))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        poolBindInfo: *mut crate::Zenject::MemoryPoolBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, factoryBindInfo, poolBindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        poolBindInfo: *mut crate::Zenject::MemoryPoolBindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, bindInfo, factoryBindInfo, poolBindInfo),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MemoryPoolExpandBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
