#[cfg(feature = "UnityEngine+ResourceManagement+Util+ObjectInitializationData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ObjectInitializationData {
    pub m_Id: *mut crate::System::String,
    pub m_ObjectType: crate::UnityEngine::ResourceManagement::Util::SerializedType,
    pub m_Data: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ObjectInitializationData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::ObjectInitializationData =>
    "UnityEngine.ResourceManagement.Util"."ObjectInitializationData"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ObjectInitializationData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ObjectInitializationData")]
impl crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData {
    pub fn CreateInstance<TObject>(
        &mut self,
        idOverride: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateInstance",
            (idOverride),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetAsyncInitHandle(
        &mut self,
        rm: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
        idOverride: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAsyncInitHandle",
            (rm, idOverride),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Id",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::Util::SerializedType,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::Util::SerializedType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ObjectType",
            (),
        )?;
        Ok(__cordl_ret)
    }
}