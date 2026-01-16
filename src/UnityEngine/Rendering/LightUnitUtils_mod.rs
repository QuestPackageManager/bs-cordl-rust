#[cfg(feature = "cordl_class_UnityEngine+Rendering+LightUnitUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct LightUnitUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LightUnitUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::LightUnitUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LightUnitUtils";
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
#[cfg(feature = "UnityEngine+Rendering+LightUnitUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LightUnitUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LightUnitUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LightUnitUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LightUnitUtils")]
impl crate::UnityEngine::Rendering::LightUnitUtils {
    pub const SphereSolidAngle: f32 = 12.566371f32;
    pub fn CandelaToEv100(candela: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("CandelaToEv100")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CandelaToEv100",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (candela))? };
        Ok(__cordl_ret.into())
    }
    pub fn CandelaToLumen(candela: f32, solidAngle: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("CandelaToLumen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CandelaToLumen",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (candela, solidAngle))? };
        Ok(__cordl_ret.into())
    }
    pub fn CandelaToLux(candela: f32, distance: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("CandelaToLux")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CandelaToLux",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (candela, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertIntensity(
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        intensity: f32,
        fromUnit: crate::UnityEngine::Rendering::LightUnit,
        toUnit: crate::UnityEngine::Rendering::LightUnit,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                        f32,
                        crate::UnityEngine::Rendering::LightUnit,
                        crate::UnityEngine::Rendering::LightUnit,
                    ), f32, 4usize>("ConvertIntensity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertIntensity",
                            4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (light, intensity, fromUnit, toUnit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertIntensityInternal(
        intensity: f32,
        fromUnit: crate::UnityEngine::Rendering::LightUnit,
        toUnit: crate::UnityEngine::Rendering::LightUnit,
        lightType: crate::UnityEngine::LightType,
        area: f32,
        luxAtDistance: f32,
        solidAngle: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        f32,
                        crate::UnityEngine::Rendering::LightUnit,
                        crate::UnityEngine::Rendering::LightUnit,
                        crate::UnityEngine::LightType,
                        f32,
                        f32,
                        f32,
                    ), f32, 7usize>("ConvertIntensityInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertIntensityInternal",
                            7usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    intensity,
                    fromUnit,
                    toUnit,
                    lightType,
                    area,
                    luxAtDistance,
                    solidAngle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Ev100ToCandela(ev100: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("Ev100ToCandela")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Ev100ToCandela",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (ev100))? };
        Ok(__cordl_ret.into())
    }
    pub fn Ev100ToNits(ev100: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("Ev100ToNits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Ev100ToNits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (ev100))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAreaFromDiscLight(discRadius: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("GetAreaFromDiscLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAreaFromDiscLight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (discRadius))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAreaFromRectangleLight_Vector2_1(
        rectSize: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Vector2), f32, 1usize>(
                        "GetAreaFromRectangleLight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAreaFromRectangleLight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (rectSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAreaFromRectangleLight_f32_f32_0(
        rectSizeX: f32,
        rectSizeY: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("GetAreaFromRectangleLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAreaFromRectangleLight",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (rectSizeX, rectSizeY))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAreaFromTubeLight(tubeLength: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("GetAreaFromTubeLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAreaFromTubeLight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (tubeLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeLightUnit(
        lightType: crate::UnityEngine::LightType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::LightUnit> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::LightType),
                        crate::UnityEngine::Rendering::LightUnit,
                        1usize,
                    >("GetNativeLightUnit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNativeLightUnit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::LightUnit =
            unsafe { cordl_method_info.invoke_unchecked((), (lightType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSolidAngle(
        lightType: crate::UnityEngine::LightType,
        spotReflector: bool,
        spotAngle: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::LightType, bool, f32, f32),
                        f32,
                        4usize,
                    >("GetSolidAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSolidAngle", 4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (lightType, spotReflector, spotAngle, aspectRatio))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSolidAngleFromPointLight() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("GetSolidAngleFromPointLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSolidAngleFromPointLight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSolidAngleFromPyramidLight(
        spotAngle: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("GetSolidAngleFromPyramidLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSolidAngleFromPyramidLight",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (spotAngle, aspectRatio))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSolidAngleFromSpotLight(spotAngle: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("GetSolidAngleFromSpotLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSolidAngleFromSpotLight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (spotAngle))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightUnitSupported(
        lightType: crate::UnityEngine::LightType,
        lightUnit: crate::UnityEngine::Rendering::LightUnit,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::LightType,
                        crate::UnityEngine::Rendering::LightUnit,
                    ), bool, 2usize>("IsLightUnitSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsLightUnitSupported",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (lightType, lightUnit))? };
        Ok(__cordl_ret.into())
    }
    pub fn LumenToCandela(lumen: f32, solidAngle: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("LumenToCandela")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LumenToCandela",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (lumen, solidAngle))? };
        Ok(__cordl_ret.into())
    }
    pub fn LumenToNits(lumen: f32, area: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("LumenToNits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LumenToNits",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (lumen, area))? };
        Ok(__cordl_ret.into())
    }
    pub fn LuxToCandela(lux: f32, distance: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("LuxToCandela")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LuxToCandela",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (lux, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn NitsToEv100(nits: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("NitsToEv100")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NitsToEv100",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (nits))? };
        Ok(__cordl_ret.into())
    }
    pub fn NitsToLumen(nits: f32, area: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("NitsToLumen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NitsToLumen",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (nits, area))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_k_EvToLuminanceFactor() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_k_EvToLuminanceFactor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_k_EvToLuminanceFactor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_k_LuminanceToEvFactor() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_k_LuminanceToEvFactor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_k_LuminanceToEvFactor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LightUnitUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::LightUnitUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
