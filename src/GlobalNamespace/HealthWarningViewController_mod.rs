#[cfg(feature = "HealthWarningViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _continueButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _safeAreaRectChecker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SafeAreaRectChecker,
    >,
    pub _duration: f32,
    pub _healthAndSafetyTextMesh: quest_hook::libil2cpp::Gc<
        crate::TMPro::TextMeshProUGUI,
    >,
    pub _healthAndSafetyLocalizationKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _healthAndSafetyPSLocalizationKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _requiresInteraction: bool,
    pub _dismissCoroutine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
}
#[cfg(feature = "HealthWarningViewController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::HealthWarningViewController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "HealthWarningViewController";
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
#[cfg(feature = "HealthWarningViewController")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::HealthWarningViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningViewController")]
impl crate::GlobalNamespace::HealthWarningViewController {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidActivate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidDeactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DismissHealthAndSafety(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("DismissHealthAndSafety")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DismissHealthAndSafety", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        requiresInteraction: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (requiresInteraction))
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
    pub fn _DidActivate_b__12_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<DidActivate>b__12_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<DidActivate>b__12_0", 0usize
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HealthWarningViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
