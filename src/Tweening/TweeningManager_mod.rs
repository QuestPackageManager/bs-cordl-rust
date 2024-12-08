#[cfg(feature = "Tweening+TweeningManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TweeningManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeTweens: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Tweening::Tween,
    >,
    pub _activeTweensSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::Tweening::Tween,
    >,
    pub _tweensByOwner: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Object,
        *mut crate::System::Collections::Generic::HashSet_1<*mut crate::Tweening::Tween>,
    >,
    pub _ownerByTween: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::Tweening::Tween,
        *mut crate::System::Object,
    >,
    pub _reusableTweenHashSets: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::System::Collections::Generic::HashSet_1<*mut crate::Tweening::Tween>,
    >,
    pub _parityOfNextNewTween: crate::Tweening::FrameParity,
}
#[cfg(feature = "Tweening+TweeningManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tweening::TweeningManager => "Tweening"
    ."TweeningManager"
);
#[cfg(feature = "Tweening+TweeningManager")]
impl std::ops::Deref for crate::Tweening::TweeningManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+TweeningManager")]
impl std::ops::DerefMut for crate::Tweening::TweeningManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+TweeningManager")]
impl crate::Tweening::TweeningManager {
    pub fn AddTween(
        &mut self,
        tween: *mut crate::Tweening::Tween,
        owner: *mut crate::System::Object,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Tweening::Tween> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Tweening::Tween = __cordl_object
            .invoke("AddTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret)
    }
    pub fn AddTweenToDataStructures(
        &mut self,
        tween: *mut crate::Tweening::Tween,
        owner: *mut crate::System::Object,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTweenToDataStructures", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret)
    }
    pub fn AddTweenToOwnerDictionary(
        &mut self,
        tween: *mut crate::Tweening::Tween,
        owner: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTweenToOwnerDictionary", (tween, owner))?;
        Ok(__cordl_ret)
    }
    pub fn GetTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn KillAllTweens(
        &mut self,
        owner: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KillAllTweens", (owner))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RemoveTweenFromOwnerDictionary(
        &mut self,
        tween: *mut crate::Tweening::Tween,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveTweenFromOwnerDictionary", (tween))?;
        Ok(__cordl_ret)
    }
    pub fn RestartTween(
        &mut self,
        tween: *mut crate::Tweening::Tween,
        owner: *mut crate::System::Object,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Tweening::Tween> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Tweening::Tween = __cordl_object
            .invoke("RestartTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret)
    }
    pub fn ResumeTween(
        &mut self,
        tween: *mut crate::Tweening::Tween,
        owner: *mut crate::System::Object,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Tweening::Tween> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Tweening::Tween = __cordl_object
            .invoke("ResumeTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "Tweening+TweeningManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::TweeningManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
