#[cfg(feature = "HMUI+CircleSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct CircleSlider {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub _handleRect: *mut crate::UnityEngine::RectTransform,
    pub _cursorRadius: f32,
    pub _normalizedValue: f32,
    pub normalizedValueDidChangeEvent: *mut crate::System::Action_2<
        *mut crate::HMUI::CircleSlider,
        f32,
    >,
    pub _containerRect: *mut crate::UnityEngine::RectTransform,
    pub _handleGraphic: *mut crate::UnityEngine::UI::Graphic,
    pub _tracker: crate::UnityEngine::DrivenRectTransformTracker,
}
#[cfg(feature = "HMUI+CircleSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::CircleSlider => "HMUI"."CircleSlider"
);
#[cfg(feature = "HMUI+CircleSlider")]
impl std::ops::Deref for crate::HMUI::CircleSlider {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl std::ops::DerefMut for crate::HMUI::CircleSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl crate::HMUI::CircleSlider {
    pub fn GraphicUpdateComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GraphicUpdateComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MayDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MayDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeginDrag", (eventData))?;
        Ok(__cordl_ret.into())
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
    pub fn OnDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrag", (eventData))?;
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
    pub fn OnInitializePotentialDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInitializePotentialDrag", (eventData))?;
        Ok(__cordl_ret.into())
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
    pub fn OnRectTransformDimensionsChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRectTransformDimensionsChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Rebuild(
        &mut self,
        executing: crate::UnityEngine::UI::CanvasUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", (executing))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormalizedValue__cordl_bool1(
        &mut self,
        input: f32,
        sendCallback: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormalizedValue", (input, sendCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormalizedValue_f32_0(
        &mut self,
        input: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormalizedValue", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UI_ICanvasElement_get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("UnityEngine.UI.ICanvasElement.get_transform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCachedReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCachedReferences", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisuals", ())?;
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
    pub fn add_normalizedValueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut crate::HMUI::CircleSlider, f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_normalizedValueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_handleRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_handleRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalizedValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalizedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_normalizedValueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut crate::HMUI::CircleSlider, f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_normalizedValueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handleColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handleColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handleRect(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handleRect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normalizedValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normalizedValue", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::CircleSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::HMUI::CircleSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::HMUI::CircleSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IDragHandler>
for crate::HMUI::CircleSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IDragHandler>
for crate::HMUI::CircleSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::CircleSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::CircleSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::HMUI::CircleSlider {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::HMUI::CircleSlider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsRef<crate::UnityEngine::UI::ICanvasElement> for crate::HMUI::CircleSlider {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ICanvasElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+CircleSlider")]
impl AsMut<crate::UnityEngine::UI::ICanvasElement> for crate::HMUI::CircleSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ICanvasElement {
        unsafe { std::mem::transmute(self) }
    }
}
