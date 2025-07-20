#[cfg(feature = "Easing")]
#[repr(C)]
#[derive(Debug)]
pub struct Easing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Easing")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Easing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Easing";
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
#[cfg(feature = "Easing")]
impl std::ops::Deref for crate::GlobalNamespace::Easing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Easing")]
impl std::ops::DerefMut for crate::GlobalNamespace::Easing {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Easing")]
impl crate::GlobalNamespace::Easing {
    pub fn BeatSaberInOutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("BeatSaberInOutBack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatSaberInOutBack", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeatSaberInOutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("BeatSaberInOutBounce")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatSaberInOutBounce", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeatSaberInOutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("BeatSaberInOutElastic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatSaberInOutElastic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InBack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InBack", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InBounce")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InBounce", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InCirc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InCirc", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InCubic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InCubic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InElastic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InElastic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InExpo(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InExpo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InExpo", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutBack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutBack", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutBounce")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutBounce", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutCirc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutCirc", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutCubic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutCubic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutElastic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutElastic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutExpo(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutExpo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutExpo", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutQuad", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutQuart(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutQuart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutQuart", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutQuint(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutQuint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutQuint", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InOutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InOutSine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InOutSine", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InQuad", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InQuart(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InQuart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InQuart", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InQuint(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InQuint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InQuint", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn InSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("InSine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InSine", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linear(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("Linear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Linear", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutBack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutBack", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutBounce")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutBounce", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutCirc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutCirc", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutCubic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutCubic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutElastic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutElastic", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutExpo(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutExpo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutExpo", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutQuad", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutQuart(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutQuart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutQuart", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutQuint(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutQuint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutQuint", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn OutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("OutSine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OutSine", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Easing")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Easing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
