#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarTweenController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _avatarTransform: *mut crate::UnityEngine::Transform,
    pub _headParent: *mut crate::UnityEngine::Transform,
    pub _leftHandTransform: *mut crate::UnityEngine::Transform,
    pub _rightHandTransform: *mut crate::UnityEngine::Transform,
    pub _bodyTransform: *mut crate::UnityEngine::Transform,
    pub _headInnerTransform: *mut crate::UnityEngine::Transform,
    pub _leftHandInnerTransform: *mut crate::UnityEngine::Transform,
    pub _rightHandInnerTransform: *mut crate::UnityEngine::Transform,
    pub _bodyInnerTransform: *mut crate::UnityEngine::Transform,
    pub _popDuration: f32,
    pub _popEaseType: EaseType,
    pub _headPopAmount: f32,
    pub _handsPopAmount: f32,
    pub _clothesPopAmount: f32,
    pub _allPopAmount: f32,
    pub _appearDuration: f32,
    pub _appearSpacing: f32,
    pub _appearHeight: f32,
    pub _squashFactor: crate::UnityEngine::Vector3,
    pub _disappearDuration: f32,
    pub _disappearHeight: f32,
    pub _disappearSquash: crate::UnityEngine::Vector3,
    pub _disappearScaleEase: EaseType,
    pub _disappearPositionEase: EaseType,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _sharedCoroutineStarter: *mut ICoroutineStarter,
    pub _popHeadTween: *mut crate::Tweening::Tween_1<f32>,
    pub _popLeftHandTween: *mut crate::Tweening::Tween_1<f32>,
    pub _popRightHandTween: *mut crate::Tweening::Tween_1<f32>,
    pub _popClothesTween: *mut crate::Tweening::Tween_1<f32>,
    pub _appearHeadPositionTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearHeadScaleTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearBodyPositionTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearBodyScaleTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearRightHandPositionTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearRightHandScaleTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearLeftHandPositionTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _appearLeftHandScaleTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _disappearScaleTween: *mut crate::Tweening::Tween_1<crate::UnityEngine::Vector3>,
    pub _disappearPositionTween: *mut crate::Tweening::Tween_1<
        crate::UnityEngine::Vector3,
    >,
    pub _avatarLocalPosition: crate::UnityEngine::Vector3,
    pub _avatarLocalScale: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController =>
    "BeatSaber.BeatAvatarAdapter.AvatarEditor"."AvatarTweenController"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController {
    #[cfg(
        feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController+__c__DisplayClass54_0"
    )]
    pub type __c__DisplayClass54_0 = crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController___c__DisplayClass54_0;
    #[cfg(
        feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController+_AppearAnimation_d__55"
    )]
    pub type _AppearAnimation_d__55 = crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController__AppearAnimation_d__55;
    #[cfg(
        feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController+_DisappearAnimation_d__60"
    )]
    pub type _DisappearAnimation_d__60 = crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController__DisappearAnimation_d__60;
    pub fn AppearAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("AppearAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppearBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearBody", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppearHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppearLeftHand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearLeftHand", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppearRightHand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearRightHand", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreatePopTween(
        &mut self,
        partTransform: *mut crate::UnityEngine::Transform,
        popAmount: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Tweening::Tween_1<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Tweening::Tween_1<f32> = __cordl_object
            .invoke("CreatePopTween", (partTransform, popAmount))?;
        Ok(__cordl_ret)
    }
    pub fn DisappearAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("DisappearAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopClothes_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopClothes", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopClothes_f32_1(
        &mut self,
        popAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopClothes", (popAmount))?;
        Ok(__cordl_ret)
    }
    pub fn PopHands_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHands", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopHands_f32_1(
        &mut self,
        popAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHands", (popAmount))?;
        Ok(__cordl_ret)
    }
    pub fn PopHead_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopHead_f32_1(
        &mut self,
        popAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHead", (popAmount))?;
        Ok(__cordl_ret)
    }
    pub fn PresentAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn _AppearBody_b__56_0(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearBody>b__56_0", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearBody_b__56_1(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearBody>b__56_1", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearHead_b__57_0(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearHead>b__57_0", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearHead_b__57_1(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearHead>b__57_1", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearLeftHand_b__58_0(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearLeftHand>b__58_0", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearLeftHand_b__58_1(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearLeftHand>b__58_1", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearRightHand_b__59_0(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearRightHand>b__59_0", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _AppearRightHand_b__59_1(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AppearRightHand>b__59_1", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _DisappearAnimation_b__60_0(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DisappearAnimation>b__60_0", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _DisappearAnimation_b__60_1(
        &mut self,
        val: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DisappearAnimation>b__60_1", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}