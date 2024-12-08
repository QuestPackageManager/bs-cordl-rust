#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CacheInitialization_CacheInitOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        bool,
    >,
    pub m_Callback: *mut crate::System::Func_1<bool>,
    pub m_UpdateRequired: bool,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp =>
    "UnityEngine.AddressableAssets.Initialization"."CacheInitialization/CacheInitOp"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        bool,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
impl crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        callback: *mut crate::System::Func_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
        unscaledDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (unscaledDeltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
#[repr(C)]
#[derive(Debug)]
pub struct CacheInitialization {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Initialization::CacheInitialization =>
    "UnityEngine.AddressableAssets.Initialization"."CacheInitialization"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
impl crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization {
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+__c__DisplayClass1_0"
    )]
    pub type __c__DisplayClass1_0 = crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization___c__DisplayClass1_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization+CacheInitOp"
    )]
    pub type CacheInitOp = crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization_CacheInitOp;
    pub fn Initialize(
        &mut self,
        id: *mut crate::System::String,
        dataStr: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Initialize", (id, dataStr))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync(
        &mut self,
        rm: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
        id: *mut crate::System::String,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object.invoke("InitializeAsync", (rm, id, data))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+CacheInitialization")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::CacheInitialization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
