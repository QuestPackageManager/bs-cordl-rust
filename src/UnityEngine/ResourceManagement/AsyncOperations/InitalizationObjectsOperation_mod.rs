#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitalizationObjectsOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        bool,
    >,
    pub m_RtdOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::AddressableAssets::Initialization::ResourceManagerRuntimeData,
    >,
    pub m_Addressables: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AddressablesImpl,
    >,
    pub m_DepOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation =>
    "UnityEngine.ResourceManagement.AsyncOperations"."InitalizationObjectsOperation"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        bool,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
impl crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        rtdOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::Initialization::ResourceManagerRuntimeData,
        >,
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (rtdOp, addressables))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LogRuntimeWarnings(
        &mut self,
        pathToBuildLogs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LogRuntimeWarnings", (pathToBuildLogs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _Execute_b__8_0(
        &mut self,
        obj: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Execute>b__8_0", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DebugName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+InitalizationObjectsOperation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
