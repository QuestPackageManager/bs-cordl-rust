#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityXRDisplay {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::OpenXR::API::UnityXRDisplay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.API";
    const CLASS_NAME: &'static str = "UnityXRDisplay";
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
#[cfg(feature = "UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
impl std::ops::Deref for crate::UnityEngine::XR::OpenXR::API::UnityXRDisplay {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
impl std::ops::DerefMut for crate::UnityEngine::XR::OpenXR::API::UnityXRDisplay {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
impl crate::UnityEngine::XR::OpenXR::API::UnityXRDisplay {
    pub const kUnityXRRenderTextureIdDontCare: u32 = 0u32;
    pub const k_UnityOpenXRLib: &'static str = "UnityOpenXR";
    pub fn CreateTexture(
        desc: crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureDesc,
        id: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureDesc,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                    ), bool, 2usize>("CreateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (desc, id))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRDisplay")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::OpenXR::API::UnityXRDisplay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
