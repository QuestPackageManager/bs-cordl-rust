#[cfg(feature = "cordl_class_UnityEngine+UIElements+WorldSpaceDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct WorldSpaceDataStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+WorldSpaceDataStore")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::WorldSpaceDataStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "WorldSpaceDataStore";
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
#[cfg(feature = "UnityEngine+UIElements+WorldSpaceDataStore")]
impl std::ops::Deref for crate::UnityEngine::UIElements::WorldSpaceDataStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+WorldSpaceDataStore")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::WorldSpaceDataStore {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+WorldSpaceDataStore")]
impl crate::UnityEngine::UIElements::WorldSpaceDataStore {
    pub fn GetWorldSpaceData(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::WorldSpaceData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        crate::UnityEngine::UIElements::WorldSpaceData,
                        1usize,
                    >("GetWorldSpaceData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWorldSpaceData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::WorldSpaceData =
            unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorldSpaceData(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        data: crate::UnityEngine::UIElements::WorldSpaceData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        crate::UnityEngine::UIElements::WorldSpaceData,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetWorldSpaceData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetWorldSpaceData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ve, data))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+WorldSpaceDataStore")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::WorldSpaceDataStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
