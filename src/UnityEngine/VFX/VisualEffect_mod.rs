#[cfg(feature = "cordl_class_UnityEngine+VFX+VisualEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualEffect {
    __cordl_parent: crate::UnityEngine::Behaviour,
    pub m_cachedEventAttribute:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXEventAttribute>,
    pub outputEventReceived: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::VFX::VFXOutputEventArgs>,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VisualEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::VFX::VisualEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.VFX";
    const CLASS_NAME: &'static str = "VisualEffect";
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
#[cfg(feature = "UnityEngine+VFX+VisualEffect")]
impl std::ops::Deref for crate::UnityEngine::VFX::VisualEffect {
    type Target = crate::UnityEngine::Behaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VisualEffect")]
impl std::ops::DerefMut for crate::UnityEngine::VFX::VisualEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VisualEffect")]
impl crate::UnityEngine::VFX::VisualEffect {
    pub fn CreateVFXEventAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXEventAttribute>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::VFX::VFXEventAttribute,
                        >,
                        0usize,
                    >("CreateVFXEventAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVFXEventAttribute", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXEventAttribute> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGetCachedEventAttributeForOutputEvent_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXEventAttribute>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::VFX::VisualEffect,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::VFX::VFXEventAttribute,
                        >,
                        1usize,
                    >("InvokeGetCachedEventAttributeForOutputEvent_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeGetCachedEventAttributeForOutputEvent_Internal",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXEventAttribute> =
            unsafe { cordl_method_info.invoke_unchecked((), (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOutputEventReceived_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
        eventNameId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InvokeOutputEventReceived_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeOutputEventReceived_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (source, eventNameId))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visualEffectAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffectAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::VFX::VisualEffectAsset,
                        >,
                        0usize,
                    >("get_visualEffectAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_visualEffectAsset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffectAsset> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visualEffectAsset_Injected(
        _unity_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::IntPtr), crate::System::IntPtr, 1usize>(
                        "get_visualEffectAsset_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_visualEffectAsset_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VisualEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::VFX::VisualEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
