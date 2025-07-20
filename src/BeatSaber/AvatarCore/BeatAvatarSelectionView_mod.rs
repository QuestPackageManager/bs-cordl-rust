#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarSelectionView {
    __cordl_parent: crate::BeatSaber::AvatarCore::AvatarSelectionView,
    pub _avatarContainer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _beatAvatarVisualController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController,
    >,
    pub _avatarTweenController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController,
    >,
    pub _avatarDataModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::BeatAvatarSelectionView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "BeatAvatarSelectionView";
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
#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::BeatAvatarSelectionView {
    type Target = crate::BeatSaber::AvatarCore::AvatarSelectionView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::BeatAvatarSelectionView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
impl crate::BeatSaber::AvatarCore::BeatAvatarSelectionView {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Activate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "Activate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Deactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "Deactivate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleAvatarDataModelDidChangeAvatarData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleAvatarDataModelDidChangeAvatarData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleAvatarDataModelDidChangeAvatarData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCreated(
        &mut self,
        isCreated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetCreated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "SetCreated", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isCreated))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateAvatarVisuals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateAvatarVisuals",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::BeatAvatarSelectionView as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::BeatAvatarSelectionView as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+BeatAvatarSelectionView")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::BeatAvatarSelectionView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
