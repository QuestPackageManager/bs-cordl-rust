#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryDataProvider {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase,
    pub _IgnoreFailures_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider =>
    "UnityEngine.ResourceManagement.ResourceProviders"."BinaryDataProvider"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
    )]
    pub type InternalOp = crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp;
    pub fn get_IgnoreFailures(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreFailures", ())?;
        Ok(__cordl_ret)
    }
    pub fn Provide(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Provide", (provideHandle))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Convert", (_cordl_type, data))?;
        Ok(__cordl_ret)
    }
    pub fn set_IgnoreFailures(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreFailures", (value))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryDataProvider_InternalOp {
    __cordl_parent: crate::System::Object,
    pub m_Provider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider,
    pub m_RequestOperation: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    pub m_RequestQueueOperation: *mut crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    pub m_PI: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub m_IgnoreFailures: bool,
    pub m_Complete: bool,
    pub m_Timeout: i32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp
    => "UnityEngine.ResourceManagement.ResourceProviders"."BinaryDataProvider/InternalOp"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp {
    pub fn CompleteOperation(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteOperation", (data, exception))?;
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
    pub fn GetPercentComplete(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPercentComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
        rawProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (provideHandle, rawProvider))?;
        Ok(__cordl_ret)
    }
    pub fn SendWebRequest(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendWebRequest", (path))?;
        Ok(__cordl_ret)
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn RequestOperation_completed(
        &mut self,
        op: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestOperation_completed", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _SendWebRequest_b__13_0(
        &mut self,
        asyncOperation: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<SendWebRequest>b__13_0", (asyncOperation))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertBytes(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertBytes", (data))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BinaryDataProvider+InternalOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BinaryDataProvider_InternalOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
