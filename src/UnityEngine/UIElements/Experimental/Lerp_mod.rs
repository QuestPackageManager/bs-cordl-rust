#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
#[repr(C)]
#[derive(Debug)]
pub struct Lerp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Experimental::Lerp =>
    "UnityEngine.UIElements.Experimental"."Lerp"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Experimental::Lerp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Experimental::Lerp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
impl crate::UnityEngine::UIElements::Experimental::Lerp {
    pub fn Interpolate_Color_Color1(
        start: crate::UnityEngine::Color,
        end: crate::UnityEngine::Color,
        ratio: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interpolate", (start, end, ratio))?;
        Ok(__cordl_ret.into())
    }
    pub fn Interpolate_StyleValues_StyleValues2(
        start: crate::UnityEngine::UIElements::Experimental::StyleValues,
        end: crate::UnityEngine::UIElements::Experimental::StyleValues,
        ratio: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Experimental::StyleValues,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Experimental::StyleValues = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interpolate", (start, end, ratio))?;
        Ok(__cordl_ret.into())
    }
    pub fn Interpolate_f32_f32_0(
        start: f32,
        end: f32,
        ratio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interpolate", (start, end, ratio))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::Lerp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
