#[cfg(feature = "OVROverlayMeshGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OVROverlayMeshGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _Verts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
    >,
    pub _UV: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
    >,
    pub _Tris: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub _Overlay: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    pub _MeshFilter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    pub _MeshCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider>,
    pub _MeshRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    pub _CameraRoot: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _Transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _LastShape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    pub _LastPosition: crate::UnityEngine::Vector3,
    pub _LastRotation: crate::UnityEngine::Quaternion,
    pub _LastScale: crate::UnityEngine::Vector3,
    pub _LastDestRectLeft: crate::UnityEngine::Rect,
    pub _LastDestRectRight: crate::UnityEngine::Rect,
    pub _LastSrcRectLeft: crate::UnityEngine::Rect,
    pub _LastTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub _Awake: bool,
}
#[cfg(feature = "OVROverlayMeshGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVROverlayMeshGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVROverlayMeshGenerator";
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
        Ok(__cordl_ret.into())
    }
    pub fn BuildCube(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        worldScale: f32,
        subQuads: i32,
        expand_coef: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BuildCube",
                (
                    verts,
                    uv,
                    triangles,
                    position,
                    rotation,
                    scale,
                    worldScale,
                    subQuads,
                    expand_coef,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildHemicylinder(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
        longitudes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BuildHemicylinder",
                (verts, uv, triangles, scale, rect, longitudes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildQuad(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildQuad", (verts, uv, triangles, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSphere(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
        worldScale: f32,
        latitudes: i32,
        longitudes: i32,
        expand_coef: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BuildSphere",
                (
                    verts,
                    uv,
                    triangles,
                    position,
                    rotation,
                    scale,
                    rect,
                    worldScale,
                    latitudes,
                    longitudes,
                    expand_coef,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMesh(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        tris: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateMesh",
                (verts, uvs, tris, shape, position, rotation, scale, rect),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetCubeUV(
        face: crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
        sideUV: crate::UnityEngine::Vector2,
        expand_coef: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCubeUV", (face, sideUV, expand_coef))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCubeVert(
        face: crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
        sideUV: crate::UnityEngine::Vector2,
        expand_coef: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCubeVert", (face, sideUV, expand_coef))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereUV(
        theta: f32,
        phi: f32,
        expand_coef: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSphereUV", (theta, phi, expand_coef))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereVert(
        theta: f32,
        phi: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSphereVert", (theta, phi))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn SetOverlay(
        &mut self,
        overlay: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOverlay", (overlay))?;
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
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVROverlayMeshGenerator_CubeFace {
    #[default]
    Back = 5i32,
    Bottom = 3i32,
    COUNT = 6i32,
    Front = 4i32,
    Left = 1i32,
    Right = 0i32,
    Top = 2i32,
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CubeFace";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
