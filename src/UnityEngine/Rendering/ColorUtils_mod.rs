#[cfg(feature = "cordl_class_UnityEngine+Rendering+ColorUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ColorUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::ColorUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ColorUtils";
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
#[cfg(feature = "UnityEngine+Rendering+ColorUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ColorUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ColorUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ColorUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ColorUtils")]
impl crate::UnityEngine::Rendering::ColorUtils {
    pub fn CIExyToLMS(
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("CIExyToLMS")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CIExyToLMS", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ColorBalanceToLMSCoeffs(
        temperature: f32,
        tint: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("ColorBalanceToLMSCoeffs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ColorBalanceToLMSCoeffs", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (temperature, tint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeEV100(
        aperture: f32,
        shutterSpeed: f32,
        ISO: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32, f32), f32, 3usize>("ComputeEV100")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeEV100", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (aperture, shutterSpeed, ISO))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeEV100FromAvgLuminance(
        avgLuminance: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        f32,
                        1usize,
                    >("ComputeEV100FromAvgLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeEV100FromAvgLuminance", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (avgLuminance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeISO(
        aperture: f32,
        shutterSpeed: f32,
        targetEV100: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32, f32), f32, 3usize>("ComputeISO")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeISO", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (aperture, shutterSpeed, targetEV100))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEV100ToExposure(EV100: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("ConvertEV100ToExposure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertEV100ToExposure", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (EV100))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertExposureToEV100(exposure: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("ConvertExposureToEV100")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertExposureToEV100", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (exposure))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Luminance(
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>),
                        f32,
                        1usize,
                    >("Luminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Luminance", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareLiftGammaGain(
        inLift: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        inGamma: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        inGain: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_3<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        ),
                        crate::System::ValueTuple_3<
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                        >,
                        3usize,
                    >("PrepareLiftGammaGain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareLiftGammaGain", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_3<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inLift, inGamma, inGain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareShadowsMidtonesHighlights(
        inShadows: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        inMidtones: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        inHighlights: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_3<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        ),
                        crate::System::ValueTuple_3<
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                        >,
                        3usize,
                    >("PrepareShadowsMidtonesHighlights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareShadowsMidtonesHighlights", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_3<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (inShadows, inMidtones, inHighlights))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareSplitToning(
        inShadows: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        inHighlights: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        balance: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            f32,
                        ),
                        crate::System::ValueTuple_2<
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                        >,
                        3usize,
                    >("PrepareSplitToning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareSplitToning", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::UnityEngine::Vector4,
            crate::UnityEngine::Vector4,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inShadows, inHighlights, balance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StandardIlluminantY(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("StandardIlluminantY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StandardIlluminantY", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToHex(c: crate::UnityEngine::Color) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color),
                        u32,
                        1usize,
                    >("ToHex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToHex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToRGBA(hex: u32) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32),
                        crate::UnityEngine::Color,
                        1usize,
                    >("ToRGBA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToRGBA",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (hex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lensImperfectionExposureScale() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        f32,
                        0usize,
                    >("get_lensImperfectionExposureScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_lensImperfectionExposureScale", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ColorUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::ColorUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
