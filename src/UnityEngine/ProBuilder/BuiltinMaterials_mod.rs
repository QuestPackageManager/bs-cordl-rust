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
