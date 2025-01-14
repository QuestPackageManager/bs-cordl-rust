#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
#[repr(C)]
#[derive(Debug)]
pub struct Easing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Experimental::Easing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Experimental";
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
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Experimental::Easing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Experimental::Easing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl crate::UnityEngine::UIElements::Experimental::Easing {
    pub fn InBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InBack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InBack", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InBounce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InBounce", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InCirc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InCirc", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InCubic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InCubic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InElastic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InElastic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutBack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutBack", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutBounce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutBounce", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutCirc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutCirc", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutCubic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutCubic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutElastic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutElastic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, i32), f32, 2usize>("InOutPower")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutPower", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t, power)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutQuad")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutQuad", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InOutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InOutSine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InOutSine", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, i32), f32, 2usize>("InPower")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InPower", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t, power)) };
        Ok(__cordl_ret.into())
    }
    pub fn InQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InQuad")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InQuad", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn InSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("InSine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InSine", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn Linear(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("Linear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Linear", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutBack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutBack", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutBounce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutBounce", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutCirc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutCirc", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutCubic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutCubic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutElastic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutElastic", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, i32), f32, 2usize>("OutPower")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutPower", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t, power)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutQuad")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutQuad", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
    pub fn OutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("OutSine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OutSine", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (t)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::Easing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
