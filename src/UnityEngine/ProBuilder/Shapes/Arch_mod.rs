#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
#[repr(C)]
#[derive(Debug)]
pub struct Arch {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Shapes::Shape,
    >,
    pub m_Thickness: f32,
    pub m_NumberOfSides: i32,
    pub m_ArchDegrees: f32,
    pub m_EndCaps: bool,
    pub m_Smooth: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Shapes::Arch =>
    "UnityEngine.ProBuilder.Shapes"."Arch"
);
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Shapes::Arch {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Shapes::Shape,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Shapes::Arch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
impl crate::UnityEngine::ProBuilder::Shapes::Arch {
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
    pub fn GetFace(
        &mut self,
        vertex1: crate::UnityEngine::Vector2,
        vertex2: crate::UnityEngine::Vector2,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = __cordl_object.invoke("GetFace", (vertex1, vertex2, depth))?;
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
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Arch")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Shapes::Arch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
