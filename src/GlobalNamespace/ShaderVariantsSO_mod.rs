#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariant_ShaderVariantsSO_Variant {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _passType: crate::UnityEngine::Rendering::PassType,
    pub _keywords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant => ""
    ."ShaderVariantsSO/ShaderVariant/Variant"
);
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant {
    pub fn New(
        passType: crate::UnityEngine::Rendering::PassType,
        keywords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (passType, keywords))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        passType: crate::UnityEngine::Rendering::PassType,
        keywords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (passType, keywords))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_keywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_passType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::PassType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::PassType = __cordl_object
            .invoke("get_passType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant+Variant")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant {
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
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _shaderVariants: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
            >,
        >,
    >,
}
#[cfg(feature = "ShaderVariantsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderVariantsSO => ""
    ."ShaderVariantsSO"
);
#[cfg(feature = "ShaderVariantsSO")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariantsSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
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
        shaderVariants: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (shaderVariants))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_shaderVariants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant,
                >,
            >,
        > = __cordl_object.invoke("get_shaderVariants", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariantsSO_ShaderVariant {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _variants: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant,
            >,
        >,
    >,
    pub _shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
}
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderVariantsSO_ShaderVariant
    => ""."ShaderVariantsSO/ShaderVariant"
);
#[cfg(feature = "ShaderVariantsSO+ShaderVariant")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderVariantsSO_ShaderVariant {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub type Variant = crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant;
    pub fn New(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        variants: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (shader, variants))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        variants: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (shader, variants))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = __cordl_object
            .invoke("get_shader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ShaderVariant_ShaderVariantsSO_Variant,
                >,
            >,
        > = __cordl_object.invoke("get_variants", ())?;
        Ok(__cordl_ret.into())
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
