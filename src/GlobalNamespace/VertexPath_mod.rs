#[cfg(feature = "VertexPath")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexPath {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub vertexCount: i32,
    pub _localVertices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::VertexPath_Vertex,
    >,
    pub _length: f32,
    pub _cumulativeLengthAtEachVertex: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _anchorVertexMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "VertexPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VertexPath => ""."VertexPath"
);
#[cfg(feature = "VertexPath")]
impl std::ops::Deref for crate::GlobalNamespace::VertexPath {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VertexPath")]
impl std::ops::DerefMut for crate::GlobalNamespace::VertexPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VertexPath")]
impl crate::GlobalNamespace::VertexPath {
    #[cfg(feature = "VertexPath+Vertex")]
    pub type Vertex = crate::GlobalNamespace::VertexPath_Vertex;
    pub fn AddVertex(
        &mut self,
        p0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p3: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
        lastRotationAxis: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        currentPathLength: quest_hook::libil2cpp::ByRefMut<f32>,
        lastVertex: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::VertexPath_Vertex,
        >,
        vertCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddVertex",
                (
                    p0,
                    p1,
                    p2,
                    p3,
                    t,
                    lastRotationAxis,
                    currentPathLength,
                    lastVertex,
                    vertCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPoint(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetPoint", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertex(
        &mut self,
        index: i32,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        normal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertex", (index, position, tangent, normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        numberOfPathSegments: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfPathSegments))?;
        Ok(__cordl_object.into())
    }
    pub fn SplitBezierPathIntoFixNumberOfSegments(
        &mut self,
        bezierPath: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierPath>,
        numberOfVertexSegments: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SplitBezierPathIntoFixNumberOfSegments",
                (bezierPath, numberOfVertexSegments),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TimeAtPoint(
        &mut self,
        pointIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("TimeAtPoint", (pointIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateByBezierPath(
        &mut self,
        bezierPath: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierPath>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateByBezierPath", (bezierPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        numberOfPathSegments: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numberOfPathSegments))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VertexPath")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::VertexPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VertexPath+Vertex")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VertexPath_Vertex {
    pub position: crate::UnityEngine::Vector3,
    pub tangent: crate::UnityEngine::Vector3,
    pub normal: crate::UnityEngine::Vector3,
}
#[cfg(feature = "VertexPath+Vertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VertexPath_Vertex => ""
    ."VertexPath/Vertex"
);
#[cfg(feature = "VertexPath+Vertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::VertexPath_Vertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "VertexPath+Vertex")]
impl crate::GlobalNamespace::VertexPath_Vertex {}
