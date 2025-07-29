#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+GlobalIllumination+LightmapperUtils"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapperUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+GlobalIllumination+LightmapperUtils"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";
    const CLASS_NAME: &'static str = "LightmapperUtils";
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    pub fn ApplyColorTemperature(
        cct: crate::UnityEngine::Color,
        lightColor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ApplyColorTemperature")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyColorTemperature", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cct, lightColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractColorTemperature(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>),
                        crate::UnityEngine::Color,
                        1usize,
                    >("ExtractColorTemperature")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractColorTemperature", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (l))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractIndirect(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>),
                        crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
                        1usize,
                    >("ExtractIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractIndirect", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor = unsafe {
            cordl_method_info.invoke_unchecked((), (l))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractInnerCone(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>),
                        f32,
                        1usize,
                    >("ExtractInnerCone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractInnerCone", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (l))? };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut1(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        dir: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DirectionalLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::DirectionalLight,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, dir))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut2(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        point: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::PointLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::PointLight,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, point))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut3(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        spot: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::SpotLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::SpotLight,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, spot))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut4(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        rect: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::RectangleLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::RectangleLight,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, rect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut5(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        disc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DiscLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::DiscLight,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, disc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut6(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (l, cookie))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extract_LightmapBakeType0(
        baketype: crate::UnityEngine::LightmapBakeType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LightMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::LightmapBakeType),
                        crate::UnityEngine::Experimental::GlobalIllumination::LightMode,
                        1usize,
                    >("Extract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extract",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LightMode = unsafe {
            cordl_method_info.invoke_unchecked((), (baketype))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+GlobalIllumination+LightmapperUtils"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
