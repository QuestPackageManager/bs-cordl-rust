#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatorOverrideController {
    __cordl_parent: crate::UnityEngine::RuntimeAnimatorController,
    pub OnOverrideControllerDirty: *mut crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback,
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorOverrideController =>
    "UnityEngine"."AnimatorOverrideController"
);
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl std::ops::Deref for crate::UnityEngine::AnimatorOverrideController {
    type Target = crate::UnityEngine::RuntimeAnimatorController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl std::ops::DerefMut for crate::UnityEngine::AnimatorOverrideController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimatorOverrideController")]
impl crate::UnityEngine::AnimatorOverrideController {
    #[cfg(
        feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
    )]
    pub type OnOverrideControllerDirtyCallback = crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback;
    pub fn _ctor(
        &mut self,
        controller: *mut crate::UnityEngine::RuntimeAnimatorController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (controller))?;
        Ok(__cordl_ret)
    }
    pub fn SendNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNotification", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetClip(
        &mut self,
        originalClip: *mut crate::UnityEngine::AnimationClip,
        overrideClip: *mut crate::UnityEngine::AnimationClip,
        notify: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClip", (originalClip, overrideClip, notify))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyOverrides(
        &mut self,
        overrides: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::UnityEngine::AnimationClip,
                *mut crate::UnityEngine::AnimationClip,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyOverrides", (overrides))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        controller: *mut crate::UnityEngine::RuntimeAnimatorController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (controller))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback =>
    "UnityEngine"."AnimatorOverrideController/OnOverrideControllerDirtyCallback"
);
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AnimatorOverrideController+OnOverrideControllerDirtyCallback"
)]
impl crate::UnityEngine::AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
