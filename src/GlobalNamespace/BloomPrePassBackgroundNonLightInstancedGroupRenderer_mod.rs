#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    __cordl_parent: BloomPrePassNonLightPass,
    pub _renderers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BloomPrePassBackgroundNonLightRenderer,
    >,
    pub _supportedProperties: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty,
    >,
    pub _reusableFloatArrays: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _reusableVectorArrays: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub _reusableMatrixArrays: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub _reusableArraysSize: i32,
    pub _commandBuffer: *mut crate::UnityEngine::Rendering::CommandBuffer,
    pub _reusableSetMaterialPropertyBlock: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub _reusableGetMaterialPropertyBlock: *mut crate::UnityEngine::MaterialPropertyBlock,
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBackgroundNonLightInstancedGroupRenderer =>
    ""."BloomPrePassBackgroundNonLightInstancedGroupRenderer"
);
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl std::ops::Deref for BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    type Target = BloomPrePassNonLightPass;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl std::ops::DerefMut for BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    pub const kInternalMatricesCachingId: &'static str = "INTERNAL_MATRICES";
    #[cfg(
        feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
    )]
    pub type SupportedProperty = crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty;
    #[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+PropertyType")]
    pub type PropertyType = crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType;
    pub fn AutoFillRenderers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoFillRenderers", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCachedFloatArray(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("GetCachedFloatArray", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn GetCachedMatrixArray(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("GetCachedMatrixArray", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn GetCachedVectorArray(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector4,
        > = __cordl_object.invoke("GetCachedVectorArray", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn InitIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIfNeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Render(
        &mut self,
        dest: *mut crate::UnityEngine::RenderTexture,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (dest, viewMatrix, projectionMatrix))?;
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
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl quest_hook::libil2cpp::ObjectType
for BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+PropertyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType {
    Color = 2i32,
    Float = 0i32,
    Matrix4x4 = 3i32,
    Vector = 1i32,
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+PropertyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType
    => ""."BloomPrePassBackgroundNonLightInstancedGroupRenderer/PropertyType"
);
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty {
    __cordl_parent: crate::System::Object,
    pub propertyType: crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType,
    pub propertyName: *mut crate::System::String,
    pub propertyId: i32,
}
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty
    => ""."BloomPrePassBackgroundNonLightInstancedGroupRenderer/SupportedProperty"
);
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
impl crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty {
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
#[cfg(
    feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}