#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenSpaceLensFlare {
    __cordl_parent: crate::UnityEngine::Rendering::VolumeComponent,
    pub intensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::MinFloatParameter,
    >,
    pub tintColor: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ColorParameter,
    >,
    pub bloomMip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedIntParameter,
    >,
    pub firstFlareIntensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::MinFloatParameter,
    >,
    pub secondaryFlareIntensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::MinFloatParameter,
    >,
    pub warpedFlareIntensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::MinFloatParameter,
    >,
    pub warpedFlareScale: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Vector2Parameter,
    >,
    pub samples: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedIntParameter,
    >,
    pub sampleDimmer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub vignetteEffect: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub startingPosition: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub scale: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub streaksIntensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::MinFloatParameter,
    >,
    pub streaksLength: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub streaksOrientation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::FloatParameter,
    >,
    pub streaksThreshold: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
    pub resolution: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlareResolutionParameter,
    >,
    pub chromaticAbberationIntensity: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ClampedFloatParameter,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceLensFlare";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    type Target = crate::UnityEngine::Rendering::VolumeComponent;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    pub fn IsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsActive", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsStreaksActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsStreaksActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsStreaksActive", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTileCompatible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsTileCompatible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsTileCompatible", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl AsRef<crate::UnityEngine::Rendering::IPostProcessComponent>
for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceLensFlare")]
impl AsMut<crate::UnityEngine::Rendering::IPostProcessComponent>
for crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
