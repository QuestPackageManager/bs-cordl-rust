#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionPool_2<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TCollection: std::marker::PhantomData<TCollection>,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Pool::CollectionPool_2 <
    TCollection, TItem > => "UnityEngine.Pool"."CollectionPool`2" < TCollection, TItem >
);
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    pub fn Get_0() -> quest_hook::libil2cpp::Result<TCollection>
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TCollection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_ByRefMut1(
        value: quest_hook::libil2cpp::ByRefMut<TCollection>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Pool::PooledObject_1<TCollection>,
    >
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Pool::PooledObject_1<TCollection> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        toRelease: TCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (toRelease))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
