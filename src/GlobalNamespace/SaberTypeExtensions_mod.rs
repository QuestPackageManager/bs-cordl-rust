#[cfg(feature = "SaberTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SaberTypeExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SaberTypeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SaberTypeExtensions";
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
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SaberTypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberTypeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl crate::GlobalNamespace::SaberTypeExtensions {
    pub fn MainSaber(
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        crate::GlobalNamespace::SaberType,
                        1usize,
                    >("MainSaber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MainSaber", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::SaberType = unsafe {
            cordl_method_info.invoke_unchecked((), (leftHanded))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesColorType(
        saberType: crate::GlobalNamespace::SaberType,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::SaberType,
                            crate::GlobalNamespace::ColorType,
                        ),
                        bool,
                        2usize,
                    >("MatchesColorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchesColorType", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (saberType, colorType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Node(
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::XRNode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::SaberType),
                        crate::UnityEngine::XR::XRNode,
                        1usize,
                    >("Node")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Node",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::XR::XRNode = unsafe {
            cordl_method_info.invoke_unchecked((), (saberType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToColorType(
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::SaberType),
                        crate::GlobalNamespace::ColorType,
                        1usize,
                    >("ToColorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToColorType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ColorType = unsafe {
            cordl_method_info.invoke_unchecked((), (saberType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSaberType(
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::ColorType),
                        crate::GlobalNamespace::SaberType,
                        1usize,
                    >("ToSaberType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToSaberType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::SaberType = unsafe {
            cordl_method_info.invoke_unchecked((), (colorType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
