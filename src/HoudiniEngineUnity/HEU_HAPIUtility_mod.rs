#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HAPIUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HAPIUtility =>
    "HoudiniEngineUnity"."HEU_HAPIUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl crate::HoudiniEngineUnity::HEU_HAPIUtility {
    pub fn ApplyLocalTransfromFromHoudiniToUnity(
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        unityTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyLocalTransfromFromHoudiniToUnity",
                (hapiTransform, unityTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyLocalTransfromFromHoudiniToUnityForInstance(
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        unityTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyLocalTransfromFromHoudiniToUnityForInstance",
                (hapiTransform, unityTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyMatrixToLocalTransform(
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyMatrixToLocalTransform", (matrix, transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyWorldTransfromFromHoudiniToUnity(
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        unityTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyWorldTransfromFromHoudiniToUnity",
                (hapiTransform, unityTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CookNodeInHoudini(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        bCookTemplatedGeos: bool,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CookNodeInHoudini",
                (session, nodeID, bCookTemplatedGeos, assetName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CookNodeInHoudiniWithOptions(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        options: crate::HoudiniEngineUnity::HAPI_CookOptions,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CookNodeInHoudiniWithOptions",
                (session, nodeID, options, assetName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndCookAssetNode(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCookTemplatedGeos: bool,
        newAssetID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateAndCookAssetNode",
                (session, assetName, bCookTemplatedGeos, newAssetID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndCookCurveAsset(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCookTemplatedGeos: bool,
        newAssetID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateAndCookCurveAsset",
                (session, assetName, bCookTemplatedGeos, newAssetID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndCookInputAsset(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCookTemplatedGeos: bool,
        newAssetID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateAndCookInputAsset",
                (session, assetName, bCookTemplatedGeos, newAssetID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewAsset(
        assetType: crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType,
        rootName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bBuildAsync: bool,
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateNewAsset",
                (assetType, rootName, parentTransform, session, bBuildAsync, rootGO),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewCurveAsset(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bBuildAsync: bool,
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateNewCurveAsset",
                (name, parentTransform, session, bBuildAsync, rootGO),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewInputAsset(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bBuildAsync: bool,
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateNewInputAsset",
                (name, parentTransform, session, bBuildAsync, rootGO),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyChildren(
        inTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyChildren", (inTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyChildrenWithComponent<T>(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyChildrenWithComponent", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGameObject(
        gameObect: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bRegisterUndo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyGameObject", (gameObect, bRegisterUndo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoesGeoPartHaveAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoesGeoPartHaveAttribute",
                (session, geoID, partID, attrName, owner, attributeInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoesMappedPathExist(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoesMappedPathExist", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindHoudiniAssetFileInPathWithExt(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindHoudiniAssetFileInPathWithExt", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetInScene(
        assetID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetInScene", (assetID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultCookOptions(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_CookOptions> {
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_CookOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultCookOptions", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHAPITransform(
        p: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        r: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        s: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_TransformEuler> {
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_TransformEuler = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHAPITransform", (p, r, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHAPITransformFromMatrix(
        mat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_TransformEuler> {
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_TransformEuler = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHAPITransformFromMatrix", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHAPITransformQuatFromMatrix(
        mat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_Transform> {
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_Transform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHAPITransformQuatFromMatrix", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHoudiniEngineInstallationInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHoudiniEngineInstallationInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMatrix4x4(
        p: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        r: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        s: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMatrix4x4", (p, r, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMatrixFromHAPITransform(
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        bConvertToUnity: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMatrixFromHAPITransform", (hapiTransform, bConvertToUnity))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInfos(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        nodeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_NodeInfo,
        >,
        objectInfos: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::HoudiniEngineUnity::HAPI_ObjectInfo,
            >,
        >,
        objectTransforms: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::HoudiniEngineUnity::HAPI_Transform,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetObjectInfos",
                (session, assetID, nodeInfo, objectInfos, objectTransforms),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentNodeID(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentNodeID", (session, nodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPosition(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPosition", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQuaternion(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQuaternion", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRealPathFromHFSPath(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRealPathFromHFSPath", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScale(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetScale", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateHDA(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialPosition: crate::UnityEngine::Vector3,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bBuildAsync: bool,
        bLoadFromMemory: bool,
        bAlwaysOverwriteOnLoad: bool,
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateHDA",
                (
                    filePath,
                    initialPosition,
                    session,
                    bBuildAsync,
                    bLoadFromMemory,
                    bAlwaysOverwriteOnLoad,
                    rootGO,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEqualTol(a: f32, b: f32, t: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEqualTol", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHoudiniAssetFile(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHoudiniAssetFile", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNodeValidInHoudini(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNodeValidInHoudini", (session, nodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSameTransform(
        transformMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        p: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        r: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        s: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSameTransform", (transformMatrix, p, r, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSessionSyncEqual(
        syncA: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_SessionSyncInfo,
        >,
        syncB: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_SessionSyncInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSessionSyncEqual", (syncA, syncB))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSupportedPolygonType(
        partType: crate::HoudiniEngineUnity::HAPI_PartType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSupportedPolygonType", (partType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTransformEqual(
        transA: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        transB: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTransformEqual", (transA, transB))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsViewportEqual(
        viewA: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_Viewport>,
        viewB: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_Viewport>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsViewportEqual", (viewA, viewB))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadGeoWithNewGeoSync(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadGeoWithNewGeoSync", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadHDAFile(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetLibraryID: quest_hook::libil2cpp::ByRefMut<i32>,
        assetNames: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadHDAFile", (session, assetPath, assetLibraryID, assetNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateValidFilePath_Il2CppString_Il2CppString1(
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateValidFilePath", (assetName, inFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateValidFilePath_Object0(
        inObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateValidFilePath", (inObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarning", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessHoudiniCookStatus(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessHoudiniCookStatus", (session, assetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAnimationCurveTangentModes(
        animCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
        tangentValues: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAnimationCurveTangentModes", (animCurve, tangentValues))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMatrixPosition(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMatrixPosition", (m, position))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
