#[cfg(feature = "ColorSchemeView")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberAColorImage: *mut crate::UnityEngine::UI::Image,
    pub _saberBColorImage: *mut crate::UnityEngine::UI::Image,
    pub _environment0ColorImage: *mut crate::UnityEngine::UI::Image,
    pub _environment1ColorImage: *mut crate::UnityEngine::UI::Image,
    pub _environmentColor0BoostImage: *mut crate::UnityEngine::UI::Image,
    pub _environmentColor1BoostImage: *mut crate::UnityEngine::UI::Image,
    pub _obstacleColorImage: *mut crate::UnityEngine::UI::Image,
}
#[cfg(feature = "ColorSchemeView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ColorSchemeView => ""."ColorSchemeView"
);
#[cfg(feature = "ColorSchemeView")]
impl std::ops::Deref for ColorSchemeView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeView")]
impl std::ops::DerefMut for ColorSchemeView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeView")]
impl ColorSchemeView {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetColors(
        &mut self,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environment0Color: crate::UnityEngine::Color,
        environment1Color: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        obstacleColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetColors",
                (
                    saberAColor,
                    saberBColor,
                    environment0Color,
                    environment1Color,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    obstacleColor,
                ),
            )?;
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
#[cfg(feature = "ColorSchemeView")]
impl quest_hook::libil2cpp::ObjectType for ColorSchemeView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
