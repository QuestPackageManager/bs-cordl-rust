#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_MaterialData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
    pub fn GetMaterialAlpha(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
        defaultValue: f32,
        alpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMaterialAlpha",
                (session, nodeID, parameters, defaultValue, alpha),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSupportedFileFormat(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        imageInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ImageInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSupportedFileFormat", (session, imageInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureFileNameFromMaterialParam(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parmInfo: crate::HoudiniEngineUnity::HAPI_ParmInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTextureFileNameFromMaterialParam", (session, nodeID, parmInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsExistingMaterial(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsExistingMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTransparentMaterial(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTransparentMaterial", (session, nodeID, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateMaterialFromHoudini(
        &mut self,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaterialFromHoudini", (materialInfo, assetCacheFolderPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn UseLegacyShaders(
        &mut self,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeInfo: crate::HoudiniEngineUnity::HAPI_NodeInfo,
        parmInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_ParmInfo>,
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
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_MaterialData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
> for crate::HoudiniEngineUnity::HEU_MaterialData {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
> for crate::HoudiniEngineUnity::HEU_MaterialData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialData+Source")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_MaterialData_Source {
    #[default]
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
