#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerSpectatingSpotPickerViewController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _stepValuePicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StepValuePicker,
    >,
    pub _spectatorController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSpectatorController,
    >,
}
#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerSpectatingSpotPickerViewController";
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
#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
impl crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController {
    pub fn HandleDecButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleDecButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), "HandleDecButtonWasPressed",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleIncButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleIncButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), "HandleIncButtonWasPressed",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSpectatingSpotDidChangeEvent(
        &mut self,
        spectatingSpot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSpectatingSpot,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleSpectatingSpotDidChangeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSpectatingSpotDidChangeEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (spectatingSpot))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshSpectatingSpotName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RefreshSpectatingSpotName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), "RefreshSpectatingSpotName",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), "Start", 0usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotPickerViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerSpectatingSpotPickerViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
