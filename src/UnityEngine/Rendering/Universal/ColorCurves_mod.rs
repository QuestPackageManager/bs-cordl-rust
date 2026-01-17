#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ColorCurves")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ColorCurves {
    __cordl_parent: crate::UnityEngine::Rendering::VolumeComponent,
    pub master: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub red: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub green: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub blue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub hueVsHue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub hueVsSat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub satVsSat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
    pub lumVsSat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::TextureCurveParameter>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ColorCurves")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Universal::ColorCurves {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ColorCurves";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ColorCurves")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ColorCurves {
    type Target = crate::UnityEngine::Rendering::VolumeComponent;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ColorCurves")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ColorCurves {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ColorCurves")]
impl crate::UnityEngine::Rendering::Universal::ColorCurves {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ColorCurves")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Universal::ColorCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ColorCurves")]
impl AsRef<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::ColorCurves
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ColorCurves")]
impl AsMut<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::ColorCurves
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
