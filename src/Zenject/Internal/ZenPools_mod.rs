#[cfg(feature = "Zenject+Internal+ZenPools")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenPools {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ZenPools =>
    "Zenject.Internal"."ZenPools"
);
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl std::ops::Deref for crate::Zenject::Internal::ZenPools {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl std::ops::DerefMut for crate::Zenject::Internal::ZenPools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl crate::Zenject::Internal::ZenPools {
    pub fn DespawnArray<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnArray", (arr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnBindInfo(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnBindInfo", (bindInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnDictionary<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnDictionary", (dictionary))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnHashSet<T>(
        set: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnHashSet", (set))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnInjectContext(
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnInjectContext", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnList<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnList", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnLookupId(
        lookupId: quest_hook::libil2cpp::Gc<crate::Zenject::Internal::LookupId>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnLookupId", (lookupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnStatement(
        statement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DespawnStatement", (statement))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnArray<T>(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnArray", (length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnBindInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnBindInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnDictionary<TKey, TValue>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnHashSet<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("SpawnHashSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnInjectContext_InjectableInfo_InjectContext_Il2CppObject_Type_Il2CppObject1(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
        currentContext: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        targetInstance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SpawnInjectContext",
                (
                    container,
                    injectableInfo,
                    currentContext,
                    targetInstance,
                    targetType,
                    concreteIdentifier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnInjectContext_Type0(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnInjectContext", (container, memberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnList<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("SpawnList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnLookupId(
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        bindingId: crate::Zenject::BindingId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::Internal::LookupId>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::Internal::LookupId> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnLookupId", (provider, bindingId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnStatement() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnStatement", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ZenPools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
