#[cfg(feature = "cordl_class_UnityEngine+ColorGamutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorGamutUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ColorGamutUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ColorGamutUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ColorGamutUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ColorGamutUtility")]
impl std::ops::Deref for crate::UnityEngine::ColorGamutUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorGamutUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ColorGamutUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorGamutUtility")]
impl crate::UnityEngine::ColorGamutUtility {
    pub fn GetColorPrimaries(
        gamut: crate::UnityEngine::ColorGamut,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ColorPrimaries> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ColorGamut),
                        crate::UnityEngine::ColorPrimaries,
                        1usize,
                    >("GetColorPrimaries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorPrimaries", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ColorPrimaries =
            unsafe { cordl_method_info.invoke_unchecked((), (gamut))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransferFunction(
        gamut: crate::UnityEngine::ColorGamut,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TransferFunction> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ColorGamut),
                        crate::UnityEngine::TransferFunction,
                        1usize,
                    >("GetTransferFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransferFunction", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TransferFunction =
            unsafe { cordl_method_info.invoke_unchecked((), (gamut))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetWhitePoint(
        gamut: crate::UnityEngine::ColorGamut,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::WhitePoint> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ColorGamut),
                        crate::UnityEngine::WhitePoint,
                        1usize,
                    >("GetWhitePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWhitePoint", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::WhitePoint =
            unsafe { cordl_method_info.invoke_unchecked((), (gamut))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ColorGamutUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ColorGamutUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
