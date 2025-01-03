#[cfg(feature = "AutoSetupController")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoSetupController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _closeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _selectBasedOnSuggestionsToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _autoStartWhenAllReadyToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _forceAutoStartAfterSongSelectionToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _randomSongIfNoneSuggestedToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::GlobalNamespace::AutoSetupData>,
    >,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _toggleBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleBinder>,
    pub _autoSetupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AutoSetupData>,
}
#[cfg(feature = "AutoSetupController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutoSetupController => ""
    ."AutoSetupController"
);
#[cfg(feature = "AutoSetupController")]
impl std::ops::Deref for crate::GlobalNamespace::AutoSetupController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AutoSetupController")]
impl std::ops::DerefMut for crate::GlobalNamespace::AutoSetupController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AutoSetupController")]
impl crate::GlobalNamespace::AutoSetupController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        autoSetupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AutoSetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (autoSetupData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__12_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__12_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__12_1(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__12_1", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__12_2(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__12_2", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__12_3(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__12_3", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__12_4(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__12_4", (isOn))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::AutoSetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::AutoSetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AutoSetupController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AutoSetupController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
