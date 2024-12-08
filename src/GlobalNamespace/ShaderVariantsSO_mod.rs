#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariantsSO_ShaderVariant {
    __cordl_parent: crate::System::Object,
    pub _variants: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::ShaderVariant_Variant,
    >,
    pub _shader: *mut crate::UnityEngine::Shader,
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderVariantsSO_ShaderVariant
    => ""."ShaderVariantsSO/ShaderVariant"
);
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
impl crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant {
    #[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
    pub type Variant = crate::GlobalNamespace::ShaderVariant_Variant;
    pub fn New(
        shader: *mut crate::UnityEngine::Shader,
        variants: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariant_Variant,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (shader, variants))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        shader: *mut crate::UnityEngine::Shader,
        variants: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariant_Variant,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (shader, variants))?;
        Ok(__cordl_ret)
    }
    pub fn get_shader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_shader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariant_Variant,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariant_Variant,
        > = __cordl_object.invoke("get_variants", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ShaderVariantsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariantsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _shaderVariants: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
    >,
}
#[cfg(feature = "ShaderVariantsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderVariantsSO => ""
    ."ShaderVariantsSO"
);
#[cfg(feature = "ShaderVariantsSO")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariantsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShaderVariantsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO")]
impl crate::GlobalNamespace::ShaderVariantsSO {
    #[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
    pub type ShaderVariant = crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant;
    pub fn Init(
        &mut self,
        shaderVariants: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (shaderVariants))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_shaderVariants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
        > = __cordl_object.invoke("get_shaderVariants", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ShaderVariantsSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ShaderVariantsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariant_Variant {
    __cordl_parent: crate::System::Object,
    pub _passType: crate::UnityEngine::Rendering::PassType,
    pub _keywords: *mut crate::System::String,
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderVariant_Variant => ""
    ."ShaderVariantsSO/ShaderVariant/Variant"
);
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariant_Variant {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShaderVariant_Variant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl crate::GlobalNamespace::ShaderVariant_Variant {
    pub fn New(
        passType: crate::UnityEngine::Rendering::PassType,
        keywords: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (passType, keywords))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        passType: crate::UnityEngine::Rendering::PassType,
        keywords: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (passType, keywords))?;
        Ok(__cordl_ret)
    }
    pub fn get_keywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_keywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_passType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::PassType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::PassType = __cordl_object
            .invoke("get_passType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShaderVariant_Variant {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
