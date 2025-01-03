#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputMeshUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputMeshUtility =>
    "HoudiniEngineUnity"."HEU_InputMeshUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    pub fn SetMeshDetailAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: crate::UnityEngine::Vector3,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshDetailAttribute",
                (session, geoID, partID, attrName, tupleSize, data, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshPointAttribute_Il2CppArray_ByRefMut3(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshPointAttribute",
                (session, geoID, partID, attrName, data, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshPointAttribute_i32_Il2CppArray_ByRefMut1(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshPointAttribute",
                (session, geoID, partID, attrName, tupleSize, data, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshPointAttribute_i32_Il2CppArray_ByRefMut2(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3Int>,
        >,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshPointAttribute",
                (session, geoID, partID, attrName, tupleSize, data, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshPointAttribute_i32_Il2CppArray_ByRefMut__cordl_bool0(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
        bConvertToHoudiniCoordinateSystem: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshPointAttribute",
                (
                    session,
                    geoID,
                    partID,
                    attrName,
                    tupleSize,
                    data,
                    partInfo,
                    bConvertToHoudiniCoordinateSystem,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshVertexAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
        bConvertToHoudiniCoordinateSystem: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshVertexAttribute",
                (
                    session,
                    geoID,
                    partID,
                    attrName,
                    tupleSize,
                    data,
                    indices,
                    partInfo,
                    bConvertToHoudiniCoordinateSystem,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshVertexFloatAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tupleSize: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMeshVertexFloatAttribute",
                (session, geoID, partID, attrName, tupleSize, data, indices, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadMeshIntoHoudiniNode(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetNodeID: i32,
        objectID: i32,
        geoID: i32,
        mesh: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UploadMeshIntoHoudiniNode",
                (session, assetNodeID, objectID, geoID, mesh),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
