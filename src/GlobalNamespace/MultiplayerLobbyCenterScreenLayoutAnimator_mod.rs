#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyCenterScreenLayoutAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _nextLevelTransform: *mut crate::UnityEngine::RectTransform,
    pub _nextLevelBasePosition: *mut crate::UnityEngine::RectTransform,
    pub _nextLevelCountdownPosition: *mut crate::UnityEngine::RectTransform,
    pub _transitionDuration: f32,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
}
#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLobbyCenterScreenLayoutAnimator => ""
    ."MultiplayerLobbyCenterScreenLayoutAnimator"
);
#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
impl std::ops::Deref for MultiplayerLobbyCenterScreenLayoutAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
impl std::ops::DerefMut for MultiplayerLobbyCenterScreenLayoutAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
impl MultiplayerLobbyCenterScreenLayoutAnimator {
    pub fn Move(
        &mut self,
        from: crate::UnityEngine::Vector3,
        to: crate::UnityEngine::Vector3,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Move", (from, to, duration))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StartCountdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartCountdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopCountdown(
        &mut self,
        instant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCountdown", (instant))?;
        Ok(__cordl_ret)
    }
    pub fn _Move_b__7_0(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Move>b__7_0", (pos))?;
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
#[cfg(feature = "MultiplayerLobbyCenterScreenLayoutAnimator")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLobbyCenterScreenLayoutAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
