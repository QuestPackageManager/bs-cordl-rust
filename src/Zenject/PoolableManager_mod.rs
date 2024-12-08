#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PoolableManager_PoolableInfo {
    pub Poolable: *mut crate::Zenject::IPoolable,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PoolableManager_PoolableInfo =>
    "Zenject"."PoolableManager/PoolableInfo"
);
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Zenject::PoolableManager_PoolableInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
impl crate::Zenject::PoolableManager_PoolableInfo {
    pub fn _ctor(
        &mut self,
        poolable: *mut crate::Zenject::IPoolable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (poolable, priority),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct PoolableManager {
    __cordl_parent: crate::System::Object,
    pub _poolables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::IPoolable,
    >,
    pub _isSpawned: bool,
}
#[cfg(feature = "Zenject+PoolableManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PoolableManager => "Zenject"
    ."PoolableManager"
);
#[cfg(feature = "Zenject+PoolableManager")]
impl std::ops::Deref for crate::Zenject::PoolableManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl std::ops::DerefMut for crate::Zenject::PoolableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl crate::Zenject::PoolableManager {
    #[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
    pub type PoolableInfo = crate::Zenject::PoolableManager_PoolableInfo;
    #[cfg(feature = "Zenject+PoolableManager+__c")]
    pub type __c = crate::Zenject::PoolableManager___c;
    #[cfg(feature = "Zenject+PoolableManager+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::PoolableManager___c__DisplayClass2_0;
    #[cfg(feature = "Zenject+PoolableManager+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::Zenject::PoolableManager___c__DisplayClass3_0;
    pub fn CreatePoolableInfo(
        &mut self,
        poolable: *mut crate::Zenject::IPoolable,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::PoolableManager_PoolableInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::PoolableManager_PoolableInfo = __cordl_object
            .invoke("CreatePoolableInfo", (poolable, priorities))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        poolables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::IPoolable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poolables, priorities))?;
        Ok(__cordl_object)
    }
    pub fn TriggerOnDespawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerOnDespawned", ())?;
        Ok(__cordl_ret)
    }
    pub fn TriggerOnSpawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerOnSpawned", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        poolables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::IPoolable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (poolables, priorities))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PoolableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
