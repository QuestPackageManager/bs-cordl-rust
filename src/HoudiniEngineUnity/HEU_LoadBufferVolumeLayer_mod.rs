#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_LoadBufferVolumeLayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _partID: i32,
    pub _heightMapWidth: i32,
    pub _heightMapHeight: i32,
    pub _strength: f32,
    pub _diffuseTexturePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _maskTexturePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _metallic: f32,
    pub _normalTexturePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _normalScale: f32,
    pub _smoothness: f32,
    pub _specularColor: crate::UnityEngine::Color,
    pub _tileSize: crate::UnityEngine::Vector2,
    pub _tileOffset: crate::UnityEngine::Vector2,
    pub _uiExpanded: bool,
    pub _tile: i32,
    pub _normalizedHeights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _minHeight: f32,
    pub _maxHeight: f32,
    pub _heightRange: f32,
    pub _terrainSizeX: f32,
    pub _terrainSizeY: f32,
    pub _position: crate::UnityEngine::Vector3,
    pub _minBounds: crate::UnityEngine::Vector3,
    pub _maxBounds: crate::UnityEngine::Vector3,
    pub _center: crate::UnityEngine::Vector3,
    pub _layerPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _hasLayerAttributes: bool,
    pub _layerType: crate::HoudiniEngineUnity::HFLayerType,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_LoadBufferVolumeLayer";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
