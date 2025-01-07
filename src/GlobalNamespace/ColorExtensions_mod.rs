#[cfg(feature = "ColorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ColorExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ColorExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorExtensions";
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
#[cfg(feature = "ColorExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorExtensions")]
impl crate::GlobalNamespace::ColorExtensions {
    pub fn ColorWithAlpha(
        color: crate::UnityEngine::Color,
        alpha: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithAlpha", (color, alpha))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWithB(
        color: crate::UnityEngine::Color,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithB", (color, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWithG(
        color: crate::UnityEngine::Color,
        g: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithG", (color, g))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWithMultipliedAlpha(
        color: crate::UnityEngine::Color,
        alphaMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithMultipliedAlpha", (color, alphaMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWithR(
        color: crate::UnityEngine::Color,
        r: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithR", (color, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWithValue(
        color: crate::UnityEngine::Color,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorWithValue", (color, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHtmlStringRGBA(
        htmlColor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromHtmlStringRGBA", (htmlColor, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorFromHtmlString(
        colorHtmlString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetColorFromHtmlString", (colorHtmlString))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEqualTo(
        a: crate::UnityEngine::Color32,
        b: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEqualTo", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpRGBUnclamped(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LerpRGBUnclamped", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyRGB(
        c: crate::UnityEngine::Color,
        m: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyRGB", (c, m))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaturatedColor(
        color: crate::UnityEngine::Color,
        saturation: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaturatedColor", (color, saturation))?;
        Ok(__cordl_ret.into())
    }
    pub fn _FromHtmlStringRGBA_g__HtmlStringToFloat_0_0(
        htmlColor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<FromHtmlStringRGBA>g__HtmlStringToFloat|0_0", (htmlColor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
