#[cfg(feature = "ColorScheme")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorScheme {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colorSchemeId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _colorSchemeNameLocalizationKey: *mut quest_hook::libil2cpp::Il2CppString,
    pub _useNonLocalizedName: bool,
    pub _nonLocalizedName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _isEditable: bool,
    pub _saberAColor: crate::UnityEngine::Color,
    pub _saberBColor: crate::UnityEngine::Color,
    pub _obstaclesColor: crate::UnityEngine::Color,
    pub _environmentColor0: crate::UnityEngine::Color,
    pub _environmentColor1: crate::UnityEngine::Color,
    pub _environmentColorW: crate::UnityEngine::Color,
    pub _supportsEnvironmentColorBoost: bool,
    pub _environmentColor0Boost: crate::UnityEngine::Color,
    pub _environmentColor1Boost: crate::UnityEngine::Color,
    pub _environmentColorWBoost: crate::UnityEngine::Color,
}
#[cfg(feature = "ColorScheme")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorScheme => ""."ColorScheme"
);
#[cfg(feature = "ColorScheme")]
impl std::ops::Deref for crate::GlobalNamespace::ColorScheme {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorScheme")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorScheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorScheme")]
impl crate::GlobalNamespace::ColorScheme {
    pub fn New_ColorSchemeSO3(
        colorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorScheme))?;
        Ok(__cordl_object)
    }
    pub fn New_ColorScheme_ColorSchemeSO2(
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        environmentColorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorScheme, environmentColorScheme))?;
        Ok(__cordl_object)
    }
    pub fn New_ColorScheme_Color_Color4(
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        environmentColorW: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (colorScheme, environmentColorW, environmentColorWBoost),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ColorScheme_Color_Color_Color_Color_Color__cordl_bool_Color_Color_Color_Color1(
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColorW: crate::UnityEngine::Color,
        supportsEnvironmentColorBoost: bool,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorScheme,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColorW,
                    supportsEnvironmentColorBoost,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    environmentColorWBoost,
                    obstaclesColor,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppString__cordl_bool_Il2CppString__cordl_bool_Color_Color_Color_Color_Color__cordl_bool_Color_Color_Color_Color0(
        colorSchemeId: *mut quest_hook::libil2cpp::Il2CppString,
        colorSchemeNameLocalizationKey: *mut quest_hook::libil2cpp::Il2CppString,
        useNonLocalizedName: bool,
        nonLocalizedName: *mut quest_hook::libil2cpp::Il2CppString,
        isEditable: bool,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColorW: crate::UnityEngine::Color,
        supportsEnvironmentColorBoost: bool,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorSchemeId,
                    colorSchemeNameLocalizationKey,
                    useNonLocalizedName,
                    nonLocalizedName,
                    isEditable,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColorW,
                    supportsEnvironmentColorBoost,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    environmentColorWBoost,
                    obstaclesColor,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ColorSchemeSO3(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ColorScheme_ColorSchemeSO2(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        environmentColorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorScheme, environmentColorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ColorScheme_Color_Color4(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        environmentColorW: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorScheme, environmentColorW, environmentColorWBoost))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ColorScheme_Color_Color_Color_Color_Color__cordl_bool_Color_Color_Color_Color1(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColorW: crate::UnityEngine::Color,
        supportsEnvironmentColorBoost: bool,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    colorScheme,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColorW,
                    supportsEnvironmentColorBoost,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    environmentColorWBoost,
                    obstaclesColor,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppString__cordl_bool_Il2CppString__cordl_bool_Color_Color_Color_Color_Color__cordl_bool_Color_Color_Color_Color0(
        &mut self,
        colorSchemeId: *mut quest_hook::libil2cpp::Il2CppString,
        colorSchemeNameLocalizationKey: *mut quest_hook::libil2cpp::Il2CppString,
        useNonLocalizedName: bool,
        nonLocalizedName: *mut quest_hook::libil2cpp::Il2CppString,
        isEditable: bool,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColorW: crate::UnityEngine::Color,
        supportsEnvironmentColorBoost: bool,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        environmentColorWBoost: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    colorSchemeId,
                    colorSchemeNameLocalizationKey,
                    useNonLocalizedName,
                    nonLocalizedName,
                    isEditable,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColorW,
                    supportsEnvironmentColorBoost,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    environmentColorWBoost,
                    obstaclesColor,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_colorSchemeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_colorSchemeId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorSchemeNameLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_colorSchemeNameLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColor0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor0", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColor0Boost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor0Boost", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColor1Boost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor1Boost", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColorW(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColorW", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentColorWBoost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColorWBoost", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isEditable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEditable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_nonLocalizedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_nonLocalizedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_obstaclesColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_obstaclesColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberAColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberAColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberBColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberBColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_supportsEnvironmentColorBoost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_supportsEnvironmentColorBoost", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useNonLocalizedName(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useNonLocalizedName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ColorScheme")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorScheme {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
