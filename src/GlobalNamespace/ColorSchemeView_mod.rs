#[cfg(feature = "ColorSchemeView")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberAColorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _saberBColorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _environment0ColorImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _environment1ColorImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _environmentColor0BoostImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _environmentColor1BoostImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _obstacleColorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
}
#[cfg(feature = "ColorSchemeView")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ColorSchemeView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorSchemeView";
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
#[cfg(feature = "ColorSchemeView")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeView")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeView")]
impl crate::GlobalNamespace::ColorSchemeView {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "ColorSchemeView")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorSchemeView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
