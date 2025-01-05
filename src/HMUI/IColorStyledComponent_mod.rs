#[cfg(feature = "HMUI+IColorStyledComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct IColorStyledComponent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+IColorStyledComponent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IColorStyledComponent => "HMUI"
    ."IColorStyledComponent"
);
#[cfg(feature = "HMUI+IColorStyledComponent")]
impl std::ops::Deref for crate::HMUI::IColorStyledComponent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IColorStyledComponent")]
impl std::ops::DerefMut for crate::HMUI::IColorStyledComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IColorStyledComponent")]
impl crate::HMUI::IColorStyledComponent {
    pub fn UpdateColorStyle_Color_f32__cordl_bool_GradientDirection_Color_Color1(
        &mut self,
        color: crate::UnityEngine::Color,
        globalLightTintIntensity: f32,
        gradient: bool,
        gradientDirection: crate::GlobalNamespace::GradientDirection,
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateColorStyle",
                (
                    color,
                    globalLightTintIntensity,
                    gradient,
                    gradientDirection,
                    color0,
                    color1,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateColorStyle_Gc0(
        &mut self,
        colorStyle: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadOnlyColorStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateColorStyle", (colorStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_colorStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadOnlyColorStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadOnlyColorStyle,
        > = __cordl_object.invoke("get_colorStyle", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+IColorStyledComponent")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IColorStyledComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
