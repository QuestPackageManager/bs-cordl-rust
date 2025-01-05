#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundPropertyHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BackgroundPropertyHelper => "UnityEngine.UIElements"
    ."BackgroundPropertyHelper"
);
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    pub fn ConvertScaleModeToBackgroundPosition(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertScaleModeToBackgroundPosition", (scaleMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertScaleModeToBackgroundRepeat(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertScaleModeToBackgroundRepeat", (scaleMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertScaleModeToBackgroundSize(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertScaleModeToBackgroundSize", (scaleMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveUnityBackgroundScaleMode(
        backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
        backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
        backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
        backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
        valid: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ScaleMode> {
        let __cordl_ret: crate::UnityEngine::ScaleMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveUnityBackgroundScaleMode",
                (
                    backgroundPositionX,
                    backgroundPositionY,
                    backgroundRepeat,
                    backgroundSize,
                    valid,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
