#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Bloom")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct Bloom {
    __cordl_parent: crate::UnityEngine::Rendering::VolumeComponent,
    pub skipIterations:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ClampedIntParameter>,
    pub threshold: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub intensity: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub scatter: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ClampedFloatParameter>,
    pub clamp: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub tint: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ColorParameter>,
    pub highQualityFiltering:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BoolParameter>,
    pub downscale:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DownscaleParameter>,
    pub maxIterations:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ClampedIntParameter>,
    pub dirtTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureParameter>,
    pub dirtIntensity: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Bloom")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Universal::Bloom {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "Bloom";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Bloom")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::Bloom {
    type Target = crate::UnityEngine::Rendering::VolumeComponent;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Bloom")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::Bloom {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Bloom")]
impl crate::UnityEngine::Rendering::Universal::Bloom {
    pub fn IsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsActive",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTileCompatible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsTileCompatible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsTileCompatible",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Bloom")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Universal::Bloom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Bloom")]
impl AsRef<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::Bloom
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Bloom")]
impl AsMut<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::Bloom
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
