#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchRendererGroup {
    __cordl_parent: crate::System::Object,
    pub m_GroupHandle: crate::System::IntPtr,
    pub m_PerformCulling: *mut crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchRendererGroup =>
    "UnityEngine.Rendering"."BatchRendererGroup"
);
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl std::ops::Deref for crate::UnityEngine::Rendering::BatchRendererGroup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::BatchRendererGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl crate::UnityEngine::Rendering::BatchRendererGroup {
    #[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
    pub type OnPerformCulling = crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling;
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::BatchRendererGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchRendererGroup_OnPerformCulling {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling =>
    "UnityEngine.Rendering"."BatchRendererGroup/OnPerformCulling"
);
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    pub fn Invoke(
        &mut self,
        rendererGroup: *mut crate::UnityEngine::Rendering::BatchRendererGroup,
        cullingContext: crate::UnityEngine::Rendering::BatchCullingContext,
        cullingOutput: crate::UnityEngine::Rendering::BatchCullingOutput,
        userContext: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Jobs::JobHandle = __cordl_object
            .invoke(
                "Invoke",
                (rendererGroup, cullingContext, cullingOutput, userContext),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
