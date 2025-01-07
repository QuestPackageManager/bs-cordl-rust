#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManagerDiagnostics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ResourceManager: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceManager,
    >,
    pub m_cachedDiagnosticInfo: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Utility";
    const CLASS_NAME: &'static str = "ResourceManagerDiagnostics";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CalculateCompletedOperationHashcode(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateCompletedOperationHashcode", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateHashCode(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CalculateHashCode", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCompletedOperationDisplayName(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GenerateCompletedOperationDisplayName", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourceManager))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourceManager))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+ResourceManagerDiagnostics")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
