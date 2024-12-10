#[cfg(feature = "HMUI+HoverHintController")]
#[repr(C)]
#[derive(Debug)]
pub struct HoverHintController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _hoverHintPanelPrefab: *mut crate::HMUI::HoverHintPanel,
    pub _hoverHintPanel: *mut crate::HMUI::HoverHintPanel,
    pub _isHiding: bool,
}
#[cfg(feature = "HMUI+HoverHintController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::HoverHintController => "HMUI"
    ."HoverHintController"
);
#[cfg(feature = "HMUI+HoverHintController")]
impl std::ops::Deref for crate::HMUI::HoverHintController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+HoverHintController")]
impl std::ops::DerefMut for crate::HMUI::HoverHintController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+HoverHintController")]
impl crate::HMUI::HoverHintController {
    pub const kHideHintDelay: f32 = 0.3f32;
    pub const kShowHintDelay: f32 = 0.6f32;
    #[cfg(feature = "HMUI+HoverHintController+_HideHintAfterDelay_d__11")]
    pub type _HideHintAfterDelay_d__11 = crate::HMUI::HoverHintController__HideHintAfterDelay_d__11;
    #[cfg(feature = "HMUI+HoverHintController+_ShowHintAfterDelay_d__10")]
    pub type _ShowHintAfterDelay_d__10 = crate::HMUI::HoverHintController__ShowHintAfterDelay_d__10;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideHint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideHint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideHintAfterDelay(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("HideHintAfterDelay", (delay))?;
        Ok(__cordl_ret.into())
    }
    pub fn HideHintInstant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideHintInstant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupAndShowHintPanel(
        &mut self,
        hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAndShowHintPanel", (hoverHint))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowHint(
        &mut self,
        hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowHint", (hoverHint))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowHintAfterDelay(
        &mut self,
        hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("ShowHintAfterDelay", (hoverHint, delay))?;
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
#[cfg(feature = "HMUI+HoverHintController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::HoverHintController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
