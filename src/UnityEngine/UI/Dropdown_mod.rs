#[cfg(feature = "UnityEngine+UI+Dropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct Dropdown {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub m_Template: *mut crate::UnityEngine::RectTransform,
    pub m_CaptionText: *mut crate::UnityEngine::UI::Text,
    pub m_CaptionImage: *mut crate::UnityEngine::UI::Image,
    pub m_ItemText: *mut crate::UnityEngine::UI::Text,
    pub m_ItemImage: *mut crate::UnityEngine::UI::Image,
    pub m_Value: i32,
    pub m_Options: *mut crate::UnityEngine::UI::Dropdown_OptionDataList,
    pub m_OnValueChanged: *mut crate::UnityEngine::UI::Dropdown_DropdownEvent,
    pub m_AlphaFadeSpeed: f32,
    pub m_Dropdown: *mut crate::UnityEngine::GameObject,
    pub m_Blocker: *mut crate::UnityEngine::GameObject,
    pub m_Items: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
    >,
    pub m_AlphaTweenRunner: *mut crate::UnityEngine::UI::CoroutineTween::TweenRunner_1<
        crate::UnityEngine::UI::CoroutineTween::FloatTween,
    >,
    pub validTemplate: bool,
}
#[cfg(feature = "UnityEngine+UI+Dropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Dropdown => "UnityEngine.UI"
    ."Dropdown"
);
#[cfg(feature = "UnityEngine+UI+Dropdown")]
impl std::ops::Deref for crate::UnityEngine::UI::Dropdown {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Dropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown")]
impl crate::UnityEngine::UI::Dropdown {
    pub const kHighSortingLayer: i32 = 30000i32;
    #[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
    pub type DropdownEvent = crate::UnityEngine::UI::Dropdown_DropdownEvent;
    #[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
    pub type DropdownItem = crate::UnityEngine::UI::Dropdown_DropdownItem;
    #[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
    pub type OptionData = crate::UnityEngine::UI::Dropdown_OptionData;
    #[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
    pub type OptionDataList = crate::UnityEngine::UI::Dropdown_OptionDataList;
    #[cfg(feature = "UnityEngine+UI+Dropdown+_DelayedDestroyDropdownList_d__75")]
    pub type _DelayedDestroyDropdownList_d__75 = crate::UnityEngine::UI::Dropdown__DelayedDestroyDropdownList_d__75;
    #[cfg(feature = "UnityEngine+UI+Dropdown+__c__DisplayClass63_0")]
    pub type __c__DisplayClass63_0 = crate::UnityEngine::UI::Dropdown___c__DisplayClass63_0;
    pub fn AddItem(
        &mut self,
        data: *mut crate::UnityEngine::UI::Dropdown_OptionData,
        selected: bool,
        itemTemplate: *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
        items: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Dropdown_DropdownItem = __cordl_object
            .invoke("AddItem", (data, selected, itemTemplate, items))?;
        Ok(__cordl_ret)
    }
    pub fn AddOptions_List_1_0(
        &mut self,
        options: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptions", (options))?;
        Ok(__cordl_ret)
    }
    pub fn AddOptions_List_1_1(
        &mut self,
        options: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptions", (options))?;
        Ok(__cordl_ret)
    }
    pub fn AddOptions_List_1_2(
        &mut self,
        options: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Sprite,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptions", (options))?;
        Ok(__cordl_ret)
    }
    pub fn AlphaFadeList_f32_1(
        &mut self,
        duration: f32,
        start: f32,
        end: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AlphaFadeList", (duration, start, end))?;
        Ok(__cordl_ret)
    }
    pub fn AlphaFadeList_f32_f32_0(
        &mut self,
        duration: f32,
        alpha: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AlphaFadeList", (duration, alpha))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateBlocker(
        &mut self,
        rootCanvas: *mut crate::UnityEngine::Canvas,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateBlocker", (rootCanvas))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDropdownList(
        &mut self,
        _cordl_template: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateDropdownList", (_cordl_template))?;
        Ok(__cordl_ret)
    }
    pub fn CreateItem(
        &mut self,
        itemTemplate: *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Dropdown_DropdownItem = __cordl_object
            .invoke("CreateItem", (itemTemplate))?;
        Ok(__cordl_ret)
    }
    pub fn DelayedDestroyDropdownList(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("DelayedDestroyDropdownList", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyBlocker(
        &mut self,
        blocker: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyBlocker", (blocker))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyDropdownList(
        &mut self,
        dropdownList: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyDropdownList", (dropdownList))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyItem(
        &mut self,
        item: *mut crate::UnityEngine::UI::Dropdown_DropdownItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyItem", (item))?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImmediateDestroyDropdownList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImmediateDestroyDropdownList", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnCancel(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnSelectItem(
        &mut self,
        toggle: *mut crate::UnityEngine::UI::Toggle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSelectItem", (toggle))?;
        Ok(__cordl_ret)
    }
    pub fn OnSubmit(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshShownValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshShownValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        value: i32,
        sendCallback: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (value, sendCallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetAlpha(
        &mut self,
        alpha: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAlpha", (alpha))?;
        Ok(__cordl_ret)
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        input: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (input))?;
        Ok(__cordl_ret)
    }
    pub fn SetupTemplate(
        &mut self,
        rootCanvas: *mut crate::UnityEngine::Canvas,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupTemplate", (rootCanvas))?;
        Ok(__cordl_ret)
    }
    pub fn Show(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", ())?;
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
    pub fn get_alphaFadeSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_alphaFadeSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_captionImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Image> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Image = __cordl_object
            .invoke("get_captionImage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_captionText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Text = __cordl_object
            .invoke("get_captionText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_itemImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Image> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Image = __cordl_object
            .invoke("get_itemImage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_itemText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Text = __cordl_object
            .invoke("get_itemText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::Dropdown_DropdownEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Dropdown_DropdownEvent = __cordl_object
            .invoke("get_onValueChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        > = __cordl_object.invoke("get_options", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_template(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_template", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_alphaFadeSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alphaFadeSpeed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_captionImage(
        &mut self,
        value: *mut crate::UnityEngine::UI::Image,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_captionImage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_captionText(
        &mut self,
        value: *mut crate::UnityEngine::UI::Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_captionText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_itemImage(
        &mut self,
        value: *mut crate::UnityEngine::UI::Image,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_itemImage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_itemText(
        &mut self,
        value: *mut crate::UnityEngine::UI::Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_itemText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onValueChanged(
        &mut self,
        value: *mut crate::UnityEngine::UI::Dropdown_DropdownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_options(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_options", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_template(
        &mut self,
        value: *mut crate::UnityEngine::RectTransform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_template", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_value(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Dropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct Dropdown_DropdownEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<i32>,
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Dropdown_DropdownEvent =>
    "UnityEngine.UI"."Dropdown/DropdownEvent"
);
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::Dropdown_DropdownEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Dropdown_DropdownEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
impl crate::UnityEngine::UI::Dropdown_DropdownEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::Dropdown_DropdownEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
#[repr(C)]
#[derive(Debug)]
pub struct Dropdown_DropdownItem {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Text: *mut crate::UnityEngine::UI::Text,
    pub m_Image: *mut crate::UnityEngine::UI::Image,
    pub m_RectTransform: *mut crate::UnityEngine::RectTransform,
    pub m_Toggle: *mut crate::UnityEngine::UI::Toggle,
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Dropdown_DropdownItem =>
    "UnityEngine.UI"."Dropdown/DropdownItem"
);
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
impl std::ops::Deref for crate::UnityEngine::UI::Dropdown_DropdownItem {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Dropdown_DropdownItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
impl crate::UnityEngine::UI::Dropdown_DropdownItem {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnCancel(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
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
    pub fn get_image(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Image> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Image = __cordl_object
            .invoke("get_image", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Text = __cordl_object
            .invoke("get_text", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_toggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Toggle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Toggle = __cordl_object
            .invoke("get_toggle", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_image(
        &mut self,
        value: *mut crate::UnityEngine::UI::Image,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_image", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rectTransform(
        &mut self,
        value: *mut crate::UnityEngine::RectTransform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rectTransform", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut crate::UnityEngine::UI::Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_toggle(
        &mut self,
        value: *mut crate::UnityEngine::UI::Toggle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_toggle", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+DropdownItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::Dropdown_DropdownItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
#[repr(C)]
#[derive(Debug)]
pub struct Dropdown_OptionData {
    __cordl_parent: crate::System::Object,
    pub m_Text: *mut crate::System::String,
    pub m_Image: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Dropdown_OptionData =>
    "UnityEngine.UI"."Dropdown/OptionData"
);
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
impl std::ops::Deref for crate::UnityEngine::UI::Dropdown_OptionData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Dropdown_OptionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
impl crate::UnityEngine::UI::Dropdown_OptionData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Sprite2(
        image: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (image))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Sprite3(
        text: *mut crate::System::String,
        image: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text, image))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Sprite2(
        &mut self,
        image: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (image))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Sprite3(
        &mut self,
        text: *mut crate::System::String,
        image: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text, image))?;
        Ok(__cordl_ret)
    }
    pub fn get_image(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_image", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_text", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_image(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_image", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Dropdown_OptionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
#[repr(C)]
#[derive(Debug)]
pub struct Dropdown_OptionDataList {
    __cordl_parent: crate::System::Object,
    pub m_Options: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UI::Dropdown_OptionData,
    >,
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Dropdown_OptionDataList =>
    "UnityEngine.UI"."Dropdown/OptionDataList"
);
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
impl std::ops::Deref for crate::UnityEngine::UI::Dropdown_OptionDataList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Dropdown_OptionDataList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
impl crate::UnityEngine::UI::Dropdown_OptionDataList {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        > = __cordl_object.invoke("get_options", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_options(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UI::Dropdown_OptionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_options", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Dropdown+OptionDataList")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::Dropdown_OptionDataList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
