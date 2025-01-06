#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ProvideHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProvideHandle {
    pub m_Version: i32,
    pub m_InternalOp: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
    >,
    pub m_ResourceManager: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceManager,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ProvideHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle =>
    "UnityEngine.ResourceManagement.ResourceProviders"."ProvideHandle"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ProvideHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ProvideHandle")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle {
    pub fn Complete<T>(
        &mut self,
        result: T,
        status: bool,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Complete",
            (result, status, exception),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDependencies",
            (list),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependency<TDepObject>(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<TDepObject>
    where
        TDepObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDepObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDependency",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDownloadProgressCallbacks(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDownloadProgressCallbacks",
            (callback),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProgressCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Func_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetProgressCallback",
            (callback),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWaitForCompletionCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetWaitForCompletionCallback",
            (callback),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        op: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rm, op),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DependencyCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DependencyCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalOp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_InternalOp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Location", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ResourceManager",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
