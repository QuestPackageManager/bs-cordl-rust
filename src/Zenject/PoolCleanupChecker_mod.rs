#[cfg(feature = "Zenject+PoolCleanupChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct PoolCleanupChecker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _poolFactories: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::IMemoryPool,
    >,
    pub _ignoredPools: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PoolCleanupChecker => "Zenject"
    ."PoolCleanupChecker"
);
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl std::ops::Deref for crate::Zenject::PoolCleanupChecker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl std::ops::DerefMut for crate::Zenject::PoolCleanupChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl crate::Zenject::PoolCleanupChecker {
    pub fn LateDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateDispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        poolFactories: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::Zenject::IMemoryPool>,
        >,
        ignoredPools: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poolFactories, ignoredPools))?;
        Ok(__cordl_object.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        poolFactories: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::Zenject::IMemoryPool>,
        >,
        ignoredPools: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (poolFactories, ignoredPools))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PoolCleanupChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl AsRef<crate::Zenject::ILateDisposable> for crate::Zenject::PoolCleanupChecker {
    fn as_ref(&self) -> &crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PoolCleanupChecker")]
impl AsMut<crate::Zenject::ILateDisposable> for crate::Zenject::PoolCleanupChecker {
    fn as_mut(&mut self) -> &mut crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
