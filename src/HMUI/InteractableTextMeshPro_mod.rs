#[cfg(feature = "HMUI+InteractableTextMeshPro")]
#[repr(C)]
#[derive(Debug)]
pub struct InteractableTextMeshPro {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub _interactionAlpha: f32,
    pub _noInteractionAlpha: f32,
    pub _text: *mut crate::TMPro::TextMeshProUGUI,
    pub _canvasGroupCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::CanvasGroup,
    >,
}
#[cfg(feature = "HMUI+InteractableTextMeshPro")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InteractableTextMeshPro => "HMUI"
    ."InteractableTextMeshPro"
);
#[cfg(feature = "HMUI+InteractableTextMeshPro")]
impl std::ops::Deref for crate::HMUI::InteractableTextMeshPro {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InteractableTextMeshPro")]
impl std::ops::DerefMut for crate::HMUI::InteractableTextMeshPro {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InteractableTextMeshPro")]
impl crate::HMUI::InteractableTextMeshPro {
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
    pub fn OnCanvasGroupChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCanvasGroupChanged", ())?;
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
#[cfg(feature = "HMUI+InteractableTextMeshPro")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::InteractableTextMeshPro {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
