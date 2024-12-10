#[cfg(feature = "OVRVignette")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVignette {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub VignetteShader: *mut crate::UnityEngine::Shader,
    pub MeshComplexity: crate::GlobalNamespace::OVRVignette_MeshComplexityLevel,
    pub Falloff: crate::GlobalNamespace::OVRVignette_FalloffType,
    pub VignetteFieldOfView: f32,
    pub VignetteAspectRatio: f32,
    pub VignetteFalloffDegrees: f32,
    pub VignetteColor: crate::UnityEngine::Color,
    pub _Camera: *mut crate::UnityEngine::Camera,
    pub _OpaqueMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _TransparentMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _OpaqueMeshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _TransparentMeshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _OpaqueMesh: *mut crate::UnityEngine::Mesh,
    pub _TransparentMesh: *mut crate::UnityEngine::Mesh,
    pub _OpaqueMaterial: *mut crate::UnityEngine::Material,
    pub _TransparentMaterial: *mut crate::UnityEngine::Material,
    pub _ShaderScaleAndOffset0Property: i32,
    pub _ShaderScaleAndOffset1Property: i32,
    pub _TransparentScaleAndOffset0: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector4,
    >,
    pub _TransparentScaleAndOffset1: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector4,
    >,
    pub _OpaqueScaleAndOffset0: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector4,
    >,
    pub _OpaqueScaleAndOffset1: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector4,
    >,
    pub _OpaqueVignetteVisible: bool,
    pub _TransparentVignetteVisible: bool,
}
#[cfg(feature = "OVRVignette")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVignette => ""."OVRVignette"
);
#[cfg(feature = "OVRVignette")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVignette {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVignette")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVignette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVignette")]
impl crate::GlobalNamespace::OVRVignette {
    #[cfg(feature = "OVRVignette+FalloffType")]
    pub type FalloffType = crate::GlobalNamespace::OVRVignette_FalloffType;
    #[cfg(feature = "OVRVignette+MeshComplexityLevel")]
    pub type MeshComplexityLevel = crate::GlobalNamespace::OVRVignette_MeshComplexityLevel;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildMaterials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildMeshes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildMeshes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableRenderers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableRenderers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableRenderers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableRenderers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTanFovAndOffsetForMonoEye(
        &mut self,
        tanFovX: quest_hook::libil2cpp::ByRefMut<f32>,
        tanFovY: quest_hook::libil2cpp::ByRefMut<f32>,
        offsetX: quest_hook::libil2cpp::ByRefMut<f32>,
        offsetY: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTanFovAndOffsetForMonoEye",
                (tanFovX, tanFovY, offsetX, offsetY),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTanFovAndOffsetForStereoEye(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        tanFovX: quest_hook::libil2cpp::ByRefMut<f32>,
        tanFovY: quest_hook::libil2cpp::ByRefMut<f32>,
        offsetX: quest_hook::libil2cpp::ByRefMut<f32>,
        offsetY: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTanFovAndOffsetForStereoEye",
                (eye, tanFovX, tanFovY, offsetX, offsetY),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTriangleCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginCameraRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeginCameraRendering", (context, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPostRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPostRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPreCull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPreCull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VisibilityTest(
        &mut self,
        scaleX: f32,
        scaleY: f32,
        offsetX: f32,
        offsetY: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VisibilityTest", (scaleX, scaleY, offsetX, offsetY))?;
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
}
#[cfg(feature = "OVRVignette")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRVignette {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVignette+FalloffType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRVignette_FalloffType {
    Linear = 0i32,
    Quadratic = 1i32,
}
#[cfg(feature = "OVRVignette+FalloffType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVignette_FalloffType => ""
    ."OVRVignette/FalloffType"
);
#[cfg(feature = "OVRVignette+MeshComplexityLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRVignette_MeshComplexityLevel {
    Detailed = 3i32,
    Normal = 2i32,
    Simple = 1i32,
    VeryDetailed = 4i32,
    VerySimple = 0i32,
}
#[cfg(feature = "OVRVignette+MeshComplexityLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVignette_MeshComplexityLevel
    => ""."OVRVignette/MeshComplexityLevel"
);
