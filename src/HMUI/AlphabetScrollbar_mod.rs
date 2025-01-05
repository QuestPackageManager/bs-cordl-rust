#[cfg(feature = "HMUI+AlphabetScrollbar")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollbar {
    __cordl_parent: crate::HMUI::Interactable,
    pub _tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
    pub _characterHeight: f32,
    pub _normalColor: crate::UnityEngine::Color,
    pub _textPrefab: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _prealocatedTexts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::TMPro::TextMeshProUGUI>,
    >,
    pub _highlightImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _characterScrollData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AlphabetScrollInfo_Data>,
        >,
    >,
    pub _texts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
        >,
    >,
    pub _highlightedCharacterIndex: i32,
    pub _pointerIsDown: bool,
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::AlphabetScrollbar => "HMUI"
    ."AlphabetScrollbar"
);
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl std::ops::Deref for crate::HMUI::AlphabetScrollbar {
    type Target = crate::HMUI::Interactable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl std::ops::DerefMut for crate::HMUI::AlphabetScrollbar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl crate::HMUI::AlphabetScrollbar {
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
    pub fn GetPointerCharacterIndex(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPointerCharacterIndex", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
        character: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitText", (text, character))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPointerDown(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointerMoveInsideCoroutine(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("PointerMoveInsideCoroutine", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareTransforms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshHighlight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshHighlight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        characterScrollData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (characterScrollData))?;
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
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::AlphabetScrollbar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+AlphabetScrollbar")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::HMUI::AlphabetScrollbar {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
