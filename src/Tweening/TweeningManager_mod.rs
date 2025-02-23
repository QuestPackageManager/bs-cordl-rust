#[cfg(feature = "Tweening+TweeningManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TweeningManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeTweens: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        >,
    >,
    pub _activeTweensSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        >,
    >,
    pub _tweensByOwner: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::HashSet_1<
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                >,
            >,
        >,
    >,
    pub _ownerByTween: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub _reusableTweenHashSets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::HashSet_1<
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                >,
            >,
        >,
    >,
    pub _parityOfNextNewTween: crate::Tweening::FrameParity,
}
#[cfg(feature = "Tweening+TweeningManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::Tweening::TweeningManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tweening";
    const CLASS_NAME: &'static str = "TweeningManager";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                3usize,
            >("AddTween")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTween", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = unsafe {
            method.invoke_unchecked(self, (tween, owner, updateEveryOtherFrame))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTweenToDataStructures(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        updateEveryOtherFrame: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                bool,
                3usize,
            >("AddTweenToDataStructures")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTweenToDataStructures", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (tween, owner, updateEveryOtherFrame))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTweenToOwnerDictionary(
        &mut self,
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddTweenToOwnerDictionary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTweenToOwnerDictionary", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tween, owner))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("GetTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTime", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn KillAllTweens(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("KillAllTweens")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "KillAllTweens", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (owner))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LateUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Tweening::Tween>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveTweenFromOwnerDictionary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveTweenFromOwnerDictionary", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tween))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                3usize,
            >("RestartTween")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RestartTween", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = unsafe {
            method.invoke_unchecked(self, (tween, owner, updateEveryOtherFrame))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::Tweening::Tween>,
                3usize,
            >("ResumeTween")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResumeTween", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Tweening::Tween> = unsafe {
            method.invoke_unchecked(self, (tween, owner, updateEveryOtherFrame))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
