#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryAssetProvider_1<TAdapter: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider,
    __cordl_phantom_TAdapter: std::marker::PhantomData<TAdapter>,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::BinaryAssetProvider_1 <
    TAdapter > => "UnityEngine.ResourceManagement.ResourceProviders"
    ."BinaryAssetProvider`1" < TAdapter >
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
impl<TAdapter: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryAssetProvider_1<
    TAdapter,
> {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
impl<TAdapter: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryAssetProvider_1<
    TAdapter,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
impl<
    TAdapter: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryAssetProvider_1<
    TAdapter,
> {
    pub fn Convert(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TAdapter: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Convert", (_cordl_type, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TAdapter: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAdapter: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryAssetProvider_1"
)]
impl<TAdapter: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryAssetProvider_1<
    TAdapter,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
