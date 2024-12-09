#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolBindingFinalizer_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::ProviderBindingFinalizer,
    pub _poolBindInfo: *mut crate::Zenject::MemoryPoolBindInfo,
    pub _factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MemoryPoolBindingFinalizer_1 <
    TContract > => "Zenject"."MemoryPoolBindingFinalizer`1" < TContract >
);
#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MemoryPoolBindingFinalizer_1<TContract> {
    type Target = crate::Zenject::ProviderBindingFinalizer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MemoryPoolBindingFinalizer_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::MemoryPoolBindingFinalizer_1<TContract> {
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        poolBindInfo: *mut crate::Zenject::MemoryPoolBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, factoryBindInfo, poolBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFinalizeBinding", (container))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
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
            .invoke(".ctor", (bindInfo, factoryBindInfo, poolBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+MemoryPoolBindingFinalizer_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MemoryPoolBindingFinalizer_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
