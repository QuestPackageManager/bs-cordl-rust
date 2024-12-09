#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTilemap {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputInterface,
    pub settings: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemapSettings,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputInterfaceTilemap =>
    "HoudiniEngineUnity"."HEU_InputInterfaceTilemap"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap {
    type Target = crate::HoudiniEngineUnity::HEU_InputInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
    pub type HEU_InputDataTilemap = crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap;
    pub fn CreateInputNodeWithDataUpload(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        connectNodeID: i32,
        inputObject: *mut crate::UnityEngine::GameObject,
        inputNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateInputNodeWithDataUpload",
                (session, connectNodeID, inputObject, inputNodeID),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTilemapDataFromGameObject(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap = __cordl_object
            .invoke("GenerateTilemapDataFromGameObject", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        settings: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemapSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsThisInputObjectSupported", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UploadData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        inputNodeID: i32,
        inputData: *mut crate::HoudiniEngineUnity::HEU_InputData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadData", (session, inputNodeID, inputData))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTilemap_HEU_InputDataTilemap {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputData,
    pub _tilemap: *mut crate::UnityEngine::Tilemaps::Tilemap,
    pub _transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap =>
    "HoudiniEngineUnity"."HEU_InputInterfaceTilemap/HEU_InputDataTilemap"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap {
    type Target = crate::HoudiniEngineUnity::HEU_InputData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTilemap+HEU_InputDataTilemap")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTilemap_HEU_InputDataTilemap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
