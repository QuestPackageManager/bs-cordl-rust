#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTrackedKeyboardSampleControls {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub trackedKeyboard: *mut OVRTrackedKeyboard,
    pub StartingFocusField: *mut crate::UnityEngine::UI::InputField,
    pub NameValue: *mut crate::UnityEngine::UI::Text,
    pub ConnectedValue: *mut crate::UnityEngine::UI::Text,
    pub StateValue: *mut crate::UnityEngine::UI::Text,
    pub SelectKeyboardValue: *mut crate::UnityEngine::UI::Text,
    pub TypeValue: *mut crate::UnityEngine::UI::Text,
    pub GoodStateColor: crate::UnityEngine::Color,
    pub BadStateColor: crate::UnityEngine::Color,
    pub TrackingToggle: *mut crate::UnityEngine::UI::Toggle,
    pub ConnectionToggle: *mut crate::UnityEngine::UI::Toggle,
    pub RemoteKeyboardToggle: *mut crate::UnityEngine::UI::Toggle,
    pub ShaderButtons: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UI::Button,
    >,
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRTrackedKeyboardSampleControls => ""
    ."OVRTrackedKeyboardSampleControls"
);
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl std::ops::Deref for OVRTrackedKeyboardSampleControls {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl std::ops::DerefMut for OVRTrackedKeyboardSampleControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl OVRTrackedKeyboardSampleControls {
    #[cfg(feature = "OVRTrackedKeyboardSampleControls+_SetShaderCoroutine_d__19")]
    pub type _SetShaderCoroutine_d__19 = crate::GlobalNamespace::OVRTrackedKeyboardSampleControls__SetShaderCoroutine_d__19;
    pub fn SetPresentationKeyLabels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresentationKeyLabels", ())?;
        Ok(__cordl_ret)
    }
    pub fn _SetShaderCoroutine_b__19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetShaderCoroutine>b__19_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPresentationOpaque(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresentationOpaque", ())?;
        Ok(__cordl_ret)
    }
    pub fn LaunchKeyboardSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchKeyboardSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetShaderCoroutine(
        &mut self,
        shaderName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("SetShaderCoroutine", (shaderName))?;
        Ok(__cordl_ret)
    }
    pub fn SetDiffuseShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDiffuseShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTrackingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrackingEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn SetUnlitShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnlitShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl quest_hook::libil2cpp::ObjectType for OVRTrackedKeyboardSampleControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
