#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarTweenController")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarTweenController {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _avatarTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _headParent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _leftHandTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _rightHandTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _bodyTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _headInnerTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _leftHandInnerTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _rightHandInnerTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _bodyInnerTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _popDuration: f32,
    pub _popEaseType: crate::GlobalNamespace::EaseType,
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
    pub _disappearScaleEase: crate::GlobalNamespace::EaseType,
    pub _disappearPositionEase: crate::GlobalNamespace::EaseType,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::TimeTweeningManager,
    >,
    pub _sharedCoroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _popHeadTween: quest_hook::libil2cpp::Gc<f32>,
    pub _popLeftHandTween: quest_hook::libil2cpp::Gc<f32>,
    pub _popRightHandTween: quest_hook::libil2cpp::Gc<f32>,
    pub _popClothesTween: quest_hook::libil2cpp::Gc<f32>,
    pub _appearHeadPositionTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _appearHeadScaleTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _appearBodyPositionTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _appearBodyScaleTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _appearRightHandPositionTween: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
    >,
    pub _appearRightHandScaleTween: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
    >,
    pub _appearLeftHandPositionTween: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
    >,
    pub _appearLeftHandScaleTween: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
    >,
    pub _disappearScaleTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _disappearPositionTween: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
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
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
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
    pub fn AppearAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("AppearAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AppearBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearBody", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AppearHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AppearLeftHand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearLeftHand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AppearRightHand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppearRightHand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePopTween(
        &mut self,
        partTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        popAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<f32> = __cordl_object
            .invoke("CreatePopTween", (partTransform, popAmount))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisappearAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("DisappearAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideAvatar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PopAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PopClothes_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopClothes", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn PopHands_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHands", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn PopHead_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopHead", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn PresentAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentAvatar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAll", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
