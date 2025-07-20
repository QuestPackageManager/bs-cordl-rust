#[cfg(feature = "Tweening+FrameParityExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FrameParityExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Tweening::FrameParityExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tweening";
    const CLASS_NAME: &'static str = "FrameParityExtensions";
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
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl std::ops::Deref for crate::Tweening::FrameParityExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl std::ops::DerefMut for crate::Tweening::FrameParityExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl crate::Tweening::FrameParityExtensions {
    pub fn GetSwitchedParity(
        frameParity: crate::Tweening::FrameParity,
    ) -> quest_hook::libil2cpp::Result<crate::Tweening::FrameParity> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Tweening::FrameParity),
                        crate::Tweening::FrameParity,
                        1usize,
                    >("GetSwitchedParity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSwitchedParity", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Tweening::FrameParity = unsafe {
            method.invoke_unchecked((), (frameParity))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::FrameParityExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
