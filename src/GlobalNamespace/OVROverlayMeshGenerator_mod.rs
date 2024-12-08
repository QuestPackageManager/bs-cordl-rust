#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVROverlayMeshGenerator_CubeFace {
    Back = 5i32,
    Bottom = 3i32,
    COUNT = 6i32,
    Front = 4i32,
    Left = 1i32,
    Right = 0i32,
    Top = 2i32,
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVROverlayMeshGenerator_CubeFace => ""
    ."OVROverlayMeshGenerator/CubeFace"
);
#[cfg(feature = "OVROverlayMeshGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OVROverlayMeshGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Mesh: *mut crate::UnityEngine::Mesh,
    pub _Verts: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub _UV: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector2,
    >,
    pub _Tris: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _Overlay: *mut crate::GlobalNamespace::OVROverlay,
    pub _MeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _MeshCollider: *mut crate::UnityEngine::MeshCollider,
    pub _MeshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _CameraRoot: *mut crate::UnityEngine::Transform,
    pub _Transform: *mut crate::UnityEngine::Transform,
    pub _LastShape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    pub _LastPosition: crate::UnityEngine::Vector3,
    pub _LastRotation: crate::UnityEngine::Quaternion,
    pub _LastScale: crate::UnityEngine::Vector3,
    pub _LastDestRectLeft: crate::UnityEngine::Rect,
    pub _LastDestRectRight: crate::UnityEngine::Rect,
    pub _LastSrcRectLeft: crate::UnityEngine::Rect,
    pub _LastTexture: *mut crate::UnityEngine::Texture,
    pub _Awake: bool,
}
#[cfg(feature = "OVROverlayMeshGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVROverlayMeshGenerator => ""
    ."OVROverlayMeshGenerator"
);
#[cfg(feature = "OVROverlayMeshGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::OVROverlayMeshGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlayMeshGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVROverlayMeshGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlayMeshGenerator")]
impl crate::GlobalNamespace::OVROverlayMeshGenerator {
    #[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
    pub type CubeFace = crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBoundingRect(
        &mut self,
        a: crate::UnityEngine::Rect,
        b: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetBoundingRect", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn SetOverlay(
        &mut self,
        overlay: *mut crate::GlobalNamespace::OVROverlay,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOverlay", (overlay))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMesh(
        &mut self,
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMesh", (shape, position, rotation, scale, rect))?;
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
#[cfg(feature = "OVROverlayMeshGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVROverlayMeshGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
