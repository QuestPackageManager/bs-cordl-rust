#[cfg(feature = "cordl_class_OVROverlayMeshGenerator")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVROverlayMeshGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Tris: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub _UV: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
    >,
    pub _CubeUV: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
    >,
    pub _Verts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
    >,
    pub _CameraRoot: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _LastDestRectLeft: crate::UnityEngine::Rect,
    pub _LastPosition: crate::UnityEngine::Vector3,
    pub _LastRotation: crate::UnityEngine::Quaternion,
    pub _LastScale: crate::UnityEngine::Vector3,
    pub _LastTextureDimension: crate::UnityEngine::Rendering::TextureDimension,
    pub _LastShape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    pub _LastSrcRectLeft: crate::UnityEngine::Rect,
    pub _Mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _MeshCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider>,
    pub _MeshFilter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    pub _MeshRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    pub _Overlay: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    pub _Transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _PreviewMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVROverlayMeshGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVROverlayMeshGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlayMeshGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVROverlayMeshGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlayMeshGenerator")]
impl crate::GlobalNamespace::OVROverlayMeshGenerator {
    #[cfg(feature = "OVROverlayMeshGenerator+CubeFace")]
    pub type CubeFace = crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace;
    pub fn BuildCube(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        cubeUV: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        triangles: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        worldScale: f32,
        subQuads: i32,
        expandCoefficient: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 10usize>("BuildCube")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildCube",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    verts,
                    uv,
                    cubeUV,
                    triangles,
                    position,
                    rotation,
                    scale,
                    worldScale,
                    subQuads,
                    expandCoefficient,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildHemicylinder(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
        longitudes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Rect,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("BuildHemicylinder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildHemicylinder",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (verts, uv, triangles, scale, rect, longitudes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildQuad(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
                        crate::UnityEngine::Rect,
                    ), quest_hook::libil2cpp::Void, 4usize>("BuildQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildQuad",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (verts, uv, triangles, rect))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildSphere(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
        worldScale: f32,
        latitudes: i32,
        longitudes: i32,
        expandCoefficient: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Rect,
                        f32,
                        i32,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 11usize>("BuildSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildSphere",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
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
                    expandCoefficient,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMesh(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        cubeUVs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        tris: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
                        crate::GlobalNamespace::OVROverlay_OverlayShape,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Rect,
                    ), quest_hook::libil2cpp::Void, 9usize>("GenerateMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateMesh",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    verts, uvs, cubeUVs, tris, shape, position, rotation, scale, rect,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCubeUV(
        face: crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
        sideU: f32,
        sideV: f32,
        expandScale: f32,
        expandOffset: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
                        f32,
                        f32,
                        f32,
                        f32,
                    ), crate::UnityEngine::Vector2, 5usize>("GetCubeUV")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCubeUV",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (face, sideU, sideV, expandScale, expandOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCubeVert(
        face: crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
        sideU: f32,
        sideV: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace,
                        f32,
                        f32,
                    ), crate::UnityEngine::Vector3, 3usize>("GetCubeVert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCubeVert",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), (face, sideU, sideV))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereUV(
        theta: f32,
        phi: f32,
        expandScale: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32, f32), crate::UnityEngine::Vector2, 3usize>(
                        "GetSphereUV",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSphereUV",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked((), (theta, phi, expandScale))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereVert(
        theta: f32,
        phi: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), crate::UnityEngine::Vector3, 2usize>(
                        "GetSphereVert",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSphereVert",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), (theta, phi))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InverseTransformVert(
        vert: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        worldScale: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                    ), crate::UnityEngine::Vector3, 4usize>(
                        "InverseTransformVert"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InverseTransformVert",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), (vert, position, scale, worldScale))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnDestroy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEnable",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlay(
        &mut self,
        overlay: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVROverlay>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (overlay))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryUpdateMesh(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("TryUpdateMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryUpdateMesh",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMesh(
        &mut self,
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
        rect: crate::UnityEngine::Rect,
        cubemap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVROverlay_OverlayShape,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Rect,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("UpdateMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateMesh",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (shape, position, rotation, scale, rect, cubemap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVROverlayMeshGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum OVROverlayMeshGenerator_CubeFace {
    #[cfg_attr(feature = "derive_Default", default)]
    Back = 2i32,
    Bottom = 0i32,
    COUNT = 6i32,
    Front = 1i32,
    Left = 4i32,
    Right = 3i32,
    Top = 5i32,
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVROverlayMeshGenerator/CubeFace";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVROverlayMeshGenerator+CubeFace")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVROverlayMeshGenerator_CubeFace
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
