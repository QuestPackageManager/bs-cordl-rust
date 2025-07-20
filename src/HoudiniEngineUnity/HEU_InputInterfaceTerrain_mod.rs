#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTerrain {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputInterface,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_InputInterfaceTerrain";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    type Target = crate::HoudiniEngineUnity::HEU_InputInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
    pub type HEU_InputDataTerrain = crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain;
    pub fn CreateHeightFieldInputNode(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        idt: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
                            >,
                        ),
                        bool,
                        2usize,
                    >("CreateHeightFieldInputNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateHeightFieldInputNode", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, idt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputNodeWithDataUpload(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        connectNodeID: i32,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        4usize,
                    >("CreateInputNodeWithDataUpload")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateInputNodeWithDataUpload", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (session, connectNodeID, inputObject, inputNodeID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTerrainDataFromGameObject(
        &mut self,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
                        >,
                        1usize,
                    >("GenerateTerrainDataFromGameObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GenerateTerrainDataFromGameObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        > = unsafe { method.invoke_unchecked(self, (inputObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        bool,
                        1usize,
                    >("IsThisInputObjectSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsThisInputObjectSupported", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (inputObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetHeightFieldData(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        volumeNodeID: i32,
        partID: i32,
        heightValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        heightFieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::HoudiniEngineUnity::HAPI_VolumeInfo,
                            >,
                        ),
                        bool,
                        6usize,
                    >("SetHeightFieldData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetHeightFieldData", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        session,
                        volumeNodeID,
                        partID,
                        heightValues,
                        heightFieldName,
                        baseVolumeInfo,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMaskLayer(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        idt: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        >,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::HoudiniEngineUnity::HAPI_VolumeInfo,
                            >,
                        ),
                        bool,
                        3usize,
                    >("SetMaskLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetMaskLayer", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, idt, baseVolumeInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTerrainDataAttributesToHeightField(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoNodeID: i32,
        partID: i32,
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
                        ),
                        bool,
                        4usize,
                    >("SetTerrainDataAttributesToHeightField")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTerrainDataAttributesToHeightField",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, geoNodeID, partID, terrainData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTerrainLayerAttributesToHeightField(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoNodeID: i32,
        partID: i32,
        terrainLayer: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainLayer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainLayer>,
                        ),
                        bool,
                        4usize,
                    >("SetTerrainLayerAttributesToHeightField")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTerrainLayerAttributesToHeightField",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, geoNodeID, partID, terrainLayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTreeInstances(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoNodeID: i32,
        partID: i32,
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetTreeInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTreeInstances", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, geoNodeID, partID, terrainData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTreePrototypes(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoNodeID: i32,
        partID: i32,
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetTreePrototypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTreePrototypes", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, geoNodeID, partID, terrainData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UploadAlphaMaps(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        idt: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        >,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
        bMaskSet: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::HoudiniEngineUnity::HAPI_VolumeInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        bool,
                        4usize,
                    >("UploadAlphaMaps")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UploadAlphaMaps", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, idt, baseVolumeInfo, bMaskSet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UploadHeightValuesWithTransform(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        idt: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        >,
        volumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::HoudiniEngineUnity::HAPI_VolumeInfo,
                            >,
                        ),
                        bool,
                        3usize,
                    >("UploadHeightValuesWithTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UploadHeightValuesWithTransform", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (session, idt, volumeInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputData,
    pub _heightFieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _parentNodeID: i32,
    pub _voxelSize: f32,
    pub _terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    pub _terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    pub _numPointsX: i32,
    pub _numPointsY: i32,
    pub _transform: crate::HoudiniEngineUnity::HAPI_Transform,
    pub _heightScale: f32,
    pub _heightfieldNodeID: i32,
    pub _heightNodeID: i32,
    pub _maskNodeID: i32,
    pub _mergeNodeID: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_InputInterfaceTerrain/HEU_InputDataTerrain";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    type Target = crate::HoudiniEngineUnity::HEU_InputData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
