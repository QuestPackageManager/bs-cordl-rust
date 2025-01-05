#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
#[repr(C)]
#[derive(Debug)]
pub struct Sphere {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Shapes::Shape,
    >,
    pub m_Subdivisions: i32,
    pub m_BottomMostVertexIndex: i32,
    pub m_Smooth: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Shapes::Sphere =>
    "UnityEngine.ProBuilder.Shapes"."Sphere"
);
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Shapes::Shape,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl crate::UnityEngine::ProBuilder::Shapes::Sphere {
    pub fn CopyShape(
        &mut self,
        shape: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Shapes::Shape>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyShape", (shape))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RebuildMesh(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        _cordl_size: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("RebuildMesh", (mesh, _cordl_size, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubdivideIcosahedron(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubdivideIcosahedron", (vertices, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBounds(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        _cordl_size: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        bounds: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("UpdateBounds", (mesh, _cordl_size, rotation, bounds))?;
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
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
