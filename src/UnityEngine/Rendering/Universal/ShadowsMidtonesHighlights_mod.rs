#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
#[repr(C)]
#[derive(Debug)]
pub struct ShadowsMidtonesHighlights {
    __cordl_parent: crate::UnityEngine::Rendering::VolumeComponent,
    pub shadows: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Vector4Parameter>,
    pub midtones: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Vector4Parameter>,
    pub highlights: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Vector4Parameter>,
    pub shadowsStart: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub shadowsEnd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub highlightsStart:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
    pub highlightsEnd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::MinFloatParameter>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShadowsMidtonesHighlights";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights {
    type Target = crate::UnityEngine::Rendering::VolumeComponent;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl AsRef<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowsMidtonesHighlights")]
impl AsMut<crate::UnityEngine::Rendering::IPostProcessComponent>
    for crate::UnityEngine::Rendering::Universal::ShadowsMidtonesHighlights
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IPostProcessComponent {
        unsafe { std::mem::transmute(self) }
    }
}
