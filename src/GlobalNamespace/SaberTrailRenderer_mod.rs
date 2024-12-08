#[cfg(feature = "SaberTrailRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberTrailRenderer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _meshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _mesh: *mut crate::UnityEngine::Mesh,
    pub _vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub _indices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _uvs: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    pub _trailWidth: f32,
    pub _trailDuration: f32,
    pub _segmentDuration: f32,
    pub _granularity: i32,
    pub _whiteSectionMaxDuration: f32,
}
#[cfg(feature = "SaberTrailRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberTrailRenderer => ""
    ."SaberTrailRenderer"
);
#[cfg(feature = "SaberTrailRenderer")]
impl std::ops::Deref for crate::GlobalNamespace::SaberTrailRenderer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTrailRenderer")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberTrailRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTrailRenderer")]
impl crate::GlobalNamespace::SaberTrailRenderer {
    pub const kMinMotionBlurSpeed: f32 = 2.5f32;
    pub const kMotionBlurStrength: f32 = 0.8f32;
    pub fn Init(
        &mut self,
        trailWidth: f32,
        trailDuration: f32,
        granularity: i32,
        whiteSectionMaxDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (trailWidth, trailDuration, granularity, whiteSectionMaxDuration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTrailWidth(
        &mut self,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrailWidth", (width))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndices", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMesh(
        &mut self,
        trailElementCollection: *mut crate::GlobalNamespace::TrailElementCollection,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMesh", (trailElementCollection, color))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVertices(
        &mut self,
        trailElementCollection: *mut crate::GlobalNamespace::TrailElementCollection,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVertices", (trailElementCollection, color))?;
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
}
#[cfg(feature = "SaberTrailRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberTrailRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
