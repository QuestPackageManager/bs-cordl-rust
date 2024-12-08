#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryProviderWrapper_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _provider: *mut crate::Zenject::IProvider,
    pub _injectContext: *mut crate::Zenject::InjectContext,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryProviderWrapper_1 < TContract >
    => "Zenject"."FactoryProviderWrapper`1" < TContract >
);
#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactoryProviderWrapper_1<TContract> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactoryProviderWrapper_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryProviderWrapper_1<TContract> {
    pub fn _ctor(
        &mut self,
        provider: *mut crate::Zenject::IProvider,
        injectContext: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider, injectContext))?;
        Ok(__cordl_ret)
    }
    pub fn Create(&mut self) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object.invoke("Create", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        provider: *mut crate::Zenject::IProvider,
        injectContext: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider, injectContext))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+FactoryProviderWrapper_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryProviderWrapper_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
