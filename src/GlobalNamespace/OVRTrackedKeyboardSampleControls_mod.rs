#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTrackedKeyboardSampleControls {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub trackedKeyboard: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRTrackedKeyboard,
    >,
    pub StartingFocusField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::InputField,
    >,
    pub NameValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub ConnectedValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub StateValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub SelectKeyboardValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub TypeValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub GoodStateColor: crate::UnityEngine::Color,
    pub BadStateColor: crate::UnityEngine::Color,
    pub TrackingToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub ConnectionToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub RemoteKeyboardToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub ShaderButtons: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
        >,
    >,
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTrackedKeyboardSampleControls {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTrackedKeyboardSampleControls";
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
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTrackedKeyboardSampleControls {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTrackedKeyboardSampleControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl crate::GlobalNamespace::OVRTrackedKeyboardSampleControls {
    pub fn LaunchKeyboardSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LaunchKeyboardSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetDiffuseShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDiffuseShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPresentationKeyLabels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresentationKeyLabels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPresentationOpaque(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresentationOpaque", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetShaderCoroutine(
        &mut self,
        shaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("SetShaderCoroutine", (shaderName))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetUnlitShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnlitShader", ())?;
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _SetShaderCoroutine_b__19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetShaderCoroutine>b__19_0", ())?;
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
#[cfg(feature = "OVRTrackedKeyboardSampleControls")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTrackedKeyboardSampleControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
