#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
#[repr(C)]
#[derive(Debug)]
pub struct Lerp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Lerp")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Experimental::Lerp {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Experimental";
    const CLASS_NAME: &'static str = "Lerp";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::Experimental::Lerp as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Color, crate::UnityEngine::Color, f32),
                crate::UnityEngine::Color,
                3usize,
            >("Interpolate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::Experimental::Lerp as
                    quest_hook::libil2cpp::Type > ::class(), "Interpolate", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (start, end, ratio))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Interpolate_StyleValues_StyleValues2(
        start: crate::UnityEngine::UIElements::Experimental::StyleValues,
        end: crate::UnityEngine::UIElements::Experimental::StyleValues,
        ratio: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Experimental::StyleValues,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::Experimental::Lerp as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::UIElements::Experimental::StyleValues,
                    crate::UnityEngine::UIElements::Experimental::StyleValues,
                    f32,
                ),
                crate::UnityEngine::UIElements::Experimental::StyleValues,
                3usize,
            >("Interpolate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::Experimental::Lerp as
                    quest_hook::libil2cpp::Type > ::class(), "Interpolate", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Experimental::StyleValues = unsafe {
            method.invoke_unchecked((), (start, end, ratio))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Interpolate_f32_f32_0(
        start: f32,
        end: f32,
        ratio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::Experimental::Lerp as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), f32, 3usize>("Interpolate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::Experimental::Lerp as
                    quest_hook::libil2cpp::Type > ::class(), "Interpolate", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (start, end, ratio))?
        };
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
