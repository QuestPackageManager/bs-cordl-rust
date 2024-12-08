#[cfg(feature = "TMPro+TMP_ColorGradient")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_ColorGradient {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub colorMode: crate::TMPro::ColorMode,
    pub topLeft: crate::UnityEngine::Color,
    pub topRight: crate::UnityEngine::Color,
    pub bottomLeft: crate::UnityEngine::Color,
    pub bottomRight: crate::UnityEngine::Color,
}
#[cfg(feature = "TMPro+TMP_ColorGradient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_ColorGradient => "TMPro"
    ."TMP_ColorGradient"
);
#[cfg(feature = "TMPro+TMP_ColorGradient")]
impl std::ops::Deref for crate::TMPro::TMP_ColorGradient {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ColorGradient")]
impl std::ops::DerefMut for crate::TMPro::TMP_ColorGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ColorGradient")]
impl crate::TMPro::TMP_ColorGradient {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Color1(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (color))?;
        Ok(__cordl_object)
    }
    pub fn New_Color_Color_Color_Color2(
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
        color2: crate::UnityEngine::Color,
        color3: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (color0, color1, color2, color3))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Color1(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (color))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Color_Color_Color_Color2(
        &mut self,
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
        color2: crate::UnityEngine::Color,
        color3: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (color0, color1, color2, color3))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_ColorGradient")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_ColorGradient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
