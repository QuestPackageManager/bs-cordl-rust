#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatorOverrideController {
    __cordl_parent: crate::UnityEngine::RuntimeAnimatorController,
    pub OnOverrideControllerDirty: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback,
    >,
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AnimatorOverrideController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AnimatorOverrideController";
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
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl std::ops::Deref for crate::UnityEngine::AnimatorOverrideController {
    type Target = crate::UnityEngine::RuntimeAnimatorController;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl std::ops::DerefMut for crate::UnityEngine::AnimatorOverrideController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl crate::UnityEngine::AnimatorOverrideController {
    #[cfg(
        feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
    )]
    pub type OnOverrideControllerDirtyCallback = crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback;
    pub fn ApplyOverrides(
        &mut self,
        overrides: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                crate::System::Collections::Generic::KeyValuePair_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::AnimationClip,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyOverrides")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ApplyOverrides", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (overrides))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimatorOverrideController,
        >,
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::RuntimeAnimatorController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimatorOverrideController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::RuntimeAnimatorController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Internal_Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_Create", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_self, controller))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::RuntimeAnimatorController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (controller))?;
        Ok(__cordl_object.into())
    }
    pub fn OnInvalidateOverrideController(
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimatorOverrideController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::AnimatorOverrideController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnInvalidateOverrideController")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnInvalidateOverrideController", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (controller))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SendNotification")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SendNotification", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetClip(
        &mut self,
        originalClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        overrideClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        notify: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetClip")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetClip", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (originalClip, overrideClip, notify))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::RuntimeAnimatorController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::RuntimeAnimatorController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (controller))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AnimatorOverrideController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AnimatorOverrideController/OnOverrideControllerDirtyCallback";
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
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
