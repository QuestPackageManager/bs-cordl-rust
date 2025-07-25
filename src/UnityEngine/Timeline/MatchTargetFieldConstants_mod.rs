#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct MatchTargetFieldConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "MatchTargetFieldConstants";
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
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl std::ops::Deref for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    pub fn HasAny(
        me: crate::UnityEngine::Timeline::MatchTargetFields,
        fields: crate::UnityEngine::Timeline::MatchTargetFields,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Timeline::MatchTargetFields,
                            crate::UnityEngine::Timeline::MatchTargetFields,
                        ),
                        bool,
                        2usize,
                    >("HasAny")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "HasAny",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (me, fields))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Toggle(
        me: crate::UnityEngine::Timeline::MatchTargetFields,
        flag: crate::UnityEngine::Timeline::MatchTargetFields,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::MatchTargetFields> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Timeline::MatchTargetFields,
                            crate::UnityEngine::Timeline::MatchTargetFields,
                        ),
                        crate::UnityEngine::Timeline::MatchTargetFields,
                        2usize,
                    >("Toggle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Toggle",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Timeline::MatchTargetFields = unsafe {
            cordl_method_info.invoke_unchecked((), (me, flag))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
