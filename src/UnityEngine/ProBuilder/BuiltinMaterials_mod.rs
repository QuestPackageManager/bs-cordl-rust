#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
#[repr(C)]
#[derive(Debug)]
pub struct BuiltinMaterials {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::BuiltinMaterials =>
    "UnityEngine.ProBuilder"."BuiltinMaterials"
);
#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::BuiltinMaterials {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::BuiltinMaterials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
impl crate::UnityEngine::ProBuilder::BuiltinMaterials {
    pub const dotShader: &'static str = "Hidden/ProBuilder/VertexShader";
    pub const faceShader: &'static str = "Hidden/ProBuilder/FaceHighlight";
    pub const lineShader: &'static str = "Hidden/ProBuilder/LineBillboard";
    pub const lineShaderMetal: &'static str = "Hidden/ProBuilder/LineBillboardMetal";
    pub const pointShader: &'static str = "Hidden/ProBuilder/PointBillboard";
    pub const wireShader: &'static str = "Hidden/ProBuilder/FaceHighlight";
    pub fn GetDefaultMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLegacyDiffuse() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLegacyDiffuse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShapePreviewMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ShapePreviewMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colliderMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_colliderMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_edgePickerMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_edgePickerMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_facePickerMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_facePickerMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_geometryShadersSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_geometryShadersSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noDrawMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_noDrawMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionPickerShader() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_selectionPickerShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_triggerMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unlitVertexColor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unlitVertexColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexPickerMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_vertexPickerMaterial", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+BuiltinMaterials")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::BuiltinMaterials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
