#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManagerDiagnostics {
    __cordl_parent: crate::System::Object,
    pub m_ResourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    pub m_cachedDiagnosticInfo: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics =>
    "UnityEngine.AddressableAssets.Utility"."ResourceManagerDiagnostics"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    pub const k_MaximumCompletedOpResultEntryLength: i32 = 30i32;
    pub const k_NumberOfCompletedOpResultEntriesToShow: i32 = 4i32;
    pub fn _ctor(
        &mut self,
        resourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourceManager))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateHashCode(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CalculateHashCode", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateCompletedOperationHashcode(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateCompletedOperationHashcode", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn SumDependencyNameHashCodes(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SumDependencyNameHashCodes", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn OnResourceManagerDiagnosticEvent(
        &mut self,
        eventContext: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnResourceManagerDiagnosticEvent", (eventContext))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCompletedOperationDisplayName(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GenerateCompletedOperationDisplayName", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        resourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourceManager))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
