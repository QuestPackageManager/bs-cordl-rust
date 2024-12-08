#[cfg(feature = "CloudsMeshGenerator+Cloud")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CloudsMeshGenerator_Cloud {
    pub precisionOpaqueMesh: *mut crate::UnityEngine::Mesh,
    pub lowPolyMesh: *mut crate::UnityEngine::Mesh,
    pub sizeModifier: f32,
    pub bottomThreshold: f32,
    pub weight: i32,
    pub precisionVertexCount: i32,
    pub lowPolyVertexCount: i32,
    pub generatedCount: i32,
}
#[cfg(feature = "CloudsMeshGenerator+Cloud")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CloudsMeshGenerator_Cloud => ""
    ."CloudsMeshGenerator/Cloud"
);
#[cfg(feature = "CloudsMeshGenerator+Cloud")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CloudsMeshGenerator_Cloud {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CloudsMeshGenerator+Cloud")]
impl crate::GlobalNamespace::CloudsMeshGenerator_Cloud {}
#[cfg(feature = "CloudsMeshGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CloudsMeshGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _meshName: *mut crate::System::String,
    pub _pauseGenerator: bool,
    pub _bottomPushEnabled: bool,
    pub _bottomPushDistance: f32,
    pub _bottomScaleTopBottom: crate::UnityEngine::Vector2,
    pub _bottomHorizontalScale: f32,
    pub _drawRingGizmos: bool,
    pub _prohibitedRadii: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius,
    >,
    pub _meshSize: crate::UnityEngine::Vector2,
    pub _sizeRandomness: f32,
    pub _ratioRandomness: f32,
    pub _perMeshRadiusOffset: f32,
    pub _possibleColors: *mut crate::UnityEngine::Gradient,
    pub _randomSeed: i32,
    pub _heightRandomness: f32,
    pub _ringRotationRandomness: f32,
    pub _ringCount: i32,
    pub _meshesPerRadius: f32,
    pub _radiusCloseFar: crate::UnityEngine::Vector2,
    pub _sizeCloseFar: crate::UnityEngine::Vector2,
    pub _heightCloseFar: *mut crate::UnityEngine::AnimationCurve,
    pub _lowPolyThreshold: f32,
    pub _flipNormals: bool,
    pub _curveMesh: bool,
    pub _clouds: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CloudsMeshGenerator_Cloud,
    >,
    pub _meshCount: i32,
    pub _vertexCount: i32,
    pub _generatedMesh: *mut crate::UnityEngine::Mesh,
    pub _meshBounds: crate::UnityEngine::Bounds,
    pub _radiusChunks: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CloudsMeshGenerator_RadiusChunk,
    >,
    pub _rings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CloudsMeshGenerator_Ring,
    >,
    pub _sortedProhibitedRadii: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius,
    >,
}
#[cfg(feature = "CloudsMeshGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CloudsMeshGenerator => ""."CloudsMeshGenerator"
);
#[cfg(feature = "CloudsMeshGenerator")]
impl std::ops::Deref for CloudsMeshGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CloudsMeshGenerator")]
impl std::ops::DerefMut for CloudsMeshGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CloudsMeshGenerator")]
impl CloudsMeshGenerator {
    #[cfg(feature = "CloudsMeshGenerator+RadiusChunk")]
    pub type RadiusChunk = crate::GlobalNamespace::CloudsMeshGenerator_RadiusChunk;
    #[cfg(feature = "CloudsMeshGenerator+Cloud")]
    pub type Cloud = crate::GlobalNamespace::CloudsMeshGenerator_Cloud;
    #[cfg(feature = "CloudsMeshGenerator+Ring")]
    pub type Ring = crate::GlobalNamespace::CloudsMeshGenerator_Ring;
    #[cfg(feature = "CloudsMeshGenerator+ProhibitedRadius")]
    pub type ProhibitedRadius = crate::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius;
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
#[cfg(feature = "CloudsMeshGenerator")]
impl quest_hook::libil2cpp::ObjectType for CloudsMeshGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CloudsMeshGenerator+ProhibitedRadius")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CloudsMeshGenerator_ProhibitedRadius {
    pub transform: *mut crate::UnityEngine::Transform,
    pub distance: f32,
    pub radius: f32,
}
#[cfg(feature = "CloudsMeshGenerator+ProhibitedRadius")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius => ""
    ."CloudsMeshGenerator/ProhibitedRadius"
);
#[cfg(feature = "CloudsMeshGenerator+ProhibitedRadius")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CloudsMeshGenerator+ProhibitedRadius")]
impl crate::GlobalNamespace::CloudsMeshGenerator_ProhibitedRadius {}
#[cfg(feature = "CloudsMeshGenerator+RadiusChunk")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CloudsMeshGenerator_RadiusChunk {
    pub normalizedStart: f32,
    pub normalizedEnd: f32,
    pub absoluteStart: f32,
    pub absoluteEnd: f32,
}
#[cfg(feature = "CloudsMeshGenerator+RadiusChunk")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CloudsMeshGenerator_RadiusChunk
    => ""."CloudsMeshGenerator/RadiusChunk"
);
#[cfg(feature = "CloudsMeshGenerator+RadiusChunk")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CloudsMeshGenerator_RadiusChunk {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CloudsMeshGenerator+RadiusChunk")]
impl crate::GlobalNamespace::CloudsMeshGenerator_RadiusChunk {}
#[cfg(feature = "CloudsMeshGenerator+Ring")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CloudsMeshGenerator_Ring {
    pub radius: f32,
    pub normalizedRadius: f32,
    pub meshCount: i32,
    pub cloudIDs: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub sizeMultiplier: f32,
}
#[cfg(feature = "CloudsMeshGenerator+Ring")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CloudsMeshGenerator_Ring => ""
    ."CloudsMeshGenerator/Ring"
);
#[cfg(feature = "CloudsMeshGenerator+Ring")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CloudsMeshGenerator_Ring {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CloudsMeshGenerator+Ring")]
impl crate::GlobalNamespace::CloudsMeshGenerator_Ring {}
