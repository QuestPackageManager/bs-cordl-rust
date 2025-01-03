#[cfg(feature = "Tweening+TweeningManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TweeningManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeTweens: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::Tweening::Tween>,
    >,
    pub _activeTweensSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<*mut crate::Tweening::Tween>,
    >,
    pub _tweensByOwner: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut crate::System::Collections::Generic::HashSet_1<
                *mut crate::Tweening::Tween,
            >,
        >,
    >,
    pub _ownerByTween: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::Tweening::Tween,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub _reusableTweenHashSets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            *mut crate::System::Collections::Generic::HashSet_1<
                *mut crate::Tweening::Tween,
            >,
        >,
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
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = __cordl_object
            .invoke("AddTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTweenToDataStructures(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTweenToDataStructures", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTweenToOwnerDictionary(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTweenToOwnerDictionary", (tween, owner))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KillAllTweens(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KillAllTweens", (owner))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveTweenFromOwnerDictionary(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveTweenFromOwnerDictionary", (tween))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestartTween(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = __cordl_object
            .invoke("RestartTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResumeTween(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = __cordl_object
            .invoke("ResumeTween", (tween, owner, updateEveryOtherFrame))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "Tweening+TweeningManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::TweeningManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
