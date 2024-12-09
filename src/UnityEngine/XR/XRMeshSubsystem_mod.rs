#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
#[repr(C)]
#[derive(Debug)]
pub struct XRMeshSubsystem {
    __cordl_parent: crate::UnityEngine::IntegratedSubsystem_1<Blacklisted>,
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRMeshSubsystem =>
    "UnityEngine.XR"."XRMeshSubsystem"
);
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
impl std::ops::Deref for crate::UnityEngine::XR::XRMeshSubsystem {
    type Target = crate::UnityEngine::IntegratedSubsystem_1<Blacklisted>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
impl std::ops::DerefMut for crate::UnityEngine::XR::XRMeshSubsystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
impl crate::UnityEngine::XR::XRMeshSubsystem {
    #[cfg(feature = "UnityEngine+XR+XRMeshSubsystem+MeshTransformList")]
    pub type MeshTransformList = crate::UnityEngine::XR::XRMeshSubsystem_MeshTransformList;
    pub fn InvokeMeshReadyDelegate(
        &mut self,
        result: crate::UnityEngine::XR::MeshGenerationResult,
        onMeshGenerationComplete: *mut crate::System::Action_1<
            crate::UnityEngine::XR::MeshGenerationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeMeshReadyDelegate", (result, onMeshGenerationComplete))?;
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
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::XRMeshSubsystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem+MeshTransformList")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XRMeshSubsystem_MeshTransformList {
    pub m_Self: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem+MeshTransformList")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::XRMeshSubsystem_MeshTransformList => "UnityEngine.XR"
    ."XRMeshSubsystem/MeshTransformList"
);
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem+MeshTransformList")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::XRMeshSubsystem_MeshTransformList {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRMeshSubsystem+MeshTransformList")]
impl crate::UnityEngine::XR::XRMeshSubsystem_MeshTransformList {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
