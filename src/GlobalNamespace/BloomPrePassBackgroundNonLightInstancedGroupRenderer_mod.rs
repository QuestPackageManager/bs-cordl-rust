#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassNonLightPass,
    >,
    pub _renderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BloomPrePassBackgroundNonLightRenderer,
            >,
        >,
    >,
    pub _supportedProperties: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty,
            >,
        >,
    >,
    pub _reusableFloatArrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    >,
    pub _reusableVectorArrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    >,
    pub _reusableMatrixArrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    >,
    pub _reusableArraysSize: i32,
    pub _commandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _reusableSetMaterialPropertyBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub _reusableGetMaterialPropertyBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer => ""
    ."BloomPrePassBackgroundNonLightInstancedGroupRenderer"
);
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassNonLightPass,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    pub const kInternalMatricesCachingId: &'static str = "INTERNAL_MATRICES";
    #[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+PropertyType")]
    pub type PropertyType = crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType;
    #[cfg(
        feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+SupportedProperty"
    )]
    pub type SupportedProperty = crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_SupportedProperty;
    pub fn AutoFillRenderers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoFillRenderers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedFloatArray(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("GetCachedFloatArray", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedMatrixArray(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = __cordl_object.invoke("GetCachedMatrixArray", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedVectorArray(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = __cordl_object.invoke("GetCachedVectorArray", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIfNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (dest, viewMatrix, projectionMatrix))?;
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
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassBackgroundNonLightInstancedGroupRenderer+PropertyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub propertyType: crate::GlobalNamespace::BloomPrePassBackgroundNonLightInstancedGroupRenderer_PropertyType,
    pub propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
