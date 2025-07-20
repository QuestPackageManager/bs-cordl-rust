#[cfg(feature = "AudioHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::AudioHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AudioHelpers";
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
#[cfg(feature = "AudioHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::AudioHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl crate::GlobalNamespace::AudioHelpers {
    pub fn DBToNormalizedVolume(db: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("DBToNormalizedVolume")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DBToNormalizedVolume", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (db))? };
        Ok(__cordl_ret.into())
    }
    pub fn NormalizedVolumeToDB(
        normalizedVolume: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("NormalizedVolumeToDB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NormalizedVolumeToDB", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (normalizedVolume))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
