#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_MaterialData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _material: *mut crate::UnityEngine::Material,
    pub _materialSource: crate::HoudiniEngineUnity::HEU_MaterialData_Source,
    pub _materialKey: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_MaterialData =>
    "HoudiniEngineUnity"."HEU_MaterialData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_MaterialData {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_MaterialData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl crate::HoudiniEngineUnity::HEU_MaterialData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData+Source")]
    pub type Source = crate::HoudiniEngineUnity::HEU_MaterialData_Source;
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn IsExistingMaterial(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsExistingMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdateMaterialFromHoudini(
        &mut self,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        assetCacheFolderPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaterialFromHoudini", (materialInfo, assetCacheFolderPath))?;
        Ok(__cordl_ret)
    }
    pub fn UseLegacyShaders(
        &mut self,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        assetCacheFolderPath: *mut crate::System::String,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        nodeInfo: crate::HoudiniEngineUnity::HAPI_NodeInfo,
        parmInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::HoudiniEngineUnity::HAPI_ParmInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UseLegacyShaders",
                (materialInfo, assetCacheFolderPath, session, nodeInfo, parmInfos),
            )?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_MaterialData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData+Source")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_MaterialData_Source {
    DEFAULT = 0i32,
    HOUDINI = 1i32,
    SUBSTANCE = 3i32,
    UNITY = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData+Source")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_MaterialData_Source =>
    "HoudiniEngineUnity"."HEU_MaterialData/Source"
);