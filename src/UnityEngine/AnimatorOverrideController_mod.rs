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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyOverrides", (overrides))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Create", (_cordl_self, controller))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnInvalidateOverrideController", (controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNotification", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClip(
        &mut self,
        originalClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        overrideClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        notify: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClip", (originalClip, overrideClip, notify))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::RuntimeAnimatorController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (controller))?;
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
    const CLASS_NAME: &'static str = "OnOverrideControllerDirtyCallback";
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
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
