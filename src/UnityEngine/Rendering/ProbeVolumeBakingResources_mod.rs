#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeBakingResources")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeVolumeBakingResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Version: i32,
    pub dilationShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub subdivideSceneCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub voxelizeSceneShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub traceVirtualOffsetCS: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ComputeShader,
    >,
    pub traceVirtualOffsetRT: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RayTracingShader,
    >,
    pub skyOcclusionCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub skyOcclusionRT: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RayTracingShader,
    >,
    pub renderingLayerCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub renderingLayerRT: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RayTracingShader,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeBakingResources")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeBakingResources";
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
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_version")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_version", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl AsRef<crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings>
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl AsMut<crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings>
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl AsRef<crate::UnityEngine::Rendering::IRenderPipelineResources>
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRenderPipelineResources {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeBakingResources")]
impl AsMut<crate::UnityEngine::Rendering::IRenderPipelineResources>
for crate::UnityEngine::Rendering::ProbeVolumeBakingResources {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::IRenderPipelineResources {
        unsafe { std::mem::transmute(self) }
    }
}
