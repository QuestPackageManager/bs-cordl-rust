#[cfg(feature = "TMPro+ShaderUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+ShaderUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::ShaderUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "ShaderUtilities";
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
#[cfg(feature = "TMPro+ShaderUtilities")]
impl std::ops::Deref for crate::TMPro::ShaderUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl std::ops::DerefMut for crate::TMPro::ShaderUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl crate::TMPro::ShaderUtilities {
    pub fn GetFontExtent(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFontExtent", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding_Il2CppArray1(
        materials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        enableExtraPadding: bool,
        isBold: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPadding", (materials, enableExtraPadding, isBold))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding_Material0(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        enableExtraPadding: bool,
        isBold: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPadding", (material, enableExtraPadding, isBold))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShaderPropertyIDs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShaderPropertyIDs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMaskingEnabled(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMaskingEnabled", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateShaderRatios(
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateShaderRatios", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShaderRef_MobileBitmap() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ShaderRef_MobileBitmap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShaderRef_MobileSDF() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ShaderRef_MobileSDF", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::ShaderUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
