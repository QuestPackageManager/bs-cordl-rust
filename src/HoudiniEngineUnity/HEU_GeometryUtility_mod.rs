#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GeometryUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_GeometryUtility =>
    "HoudiniEngineUnity"."HEU_GeometryUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_GeometryUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_GeometryUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
impl crate::HoudiniEngineUnity::HEU_GeometryUtility {
    pub fn CalculateMeshTangents(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateMeshTangents", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCubeMeshFromPoints(
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        pointsColor: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        _cordl_size: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCubeMeshFromPoints", (points, pointsColor, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePerTriangle(
        meshSrc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePerTriangle", (meshSrc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSecondaryUVSet(
        meshsrc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateSecondaryUVSet", (meshsrc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceOutputName(
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userPrefix: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstanceOutputName", (partName, userPrefix, index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeometryUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_GeometryUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
