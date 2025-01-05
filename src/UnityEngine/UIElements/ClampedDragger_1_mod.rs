#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ClampedDragger_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::Clickable,
    pub dragging: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub draggingEnded: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _dragDirection_k__BackingField: crate::UnityEngine::UIElements::ClampedDragger_1_DragDirection<
        T,
    >,
    pub _slider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseSlider_1<T>,
    >,
    pub _startMousePosition_k__BackingField: crate::UnityEngine::Vector2,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ClampedDragger_1 < T >
    => "UnityEngine.UIElements"."ClampedDragger`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::ClampedDragger_1<T> {
    type Target = crate::UnityEngine::UIElements::Clickable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::ClampedDragger_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::ClampedDragger_1<T> {
    #[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1+DragDirection")]
    pub type DragDirection = crate::UnityEngine::UIElements::ClampedDragger_1_DragDirection;
    pub fn New(
        slider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseSlider_1<T>,
        >,
        clickHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
        dragHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (slider, clickHandler, dragHandler))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessDownEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        localPosition: crate::UnityEngine::Vector2,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDownEvent", (evt, localPosition, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMoveEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        localPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMoveEvent", (evt, localPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessUpEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        localPosition: crate::UnityEngine::Vector2,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessUpEvent", (evt, localPosition, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseSlider_1<T>,
        >,
        clickHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
        dragHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (slider, clickHandler, dragHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_dragging(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_dragging", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_draggingEnded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_draggingEnded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_delta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dragDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ClampedDragger_1_DragDirection<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ClampedDragger_1_DragDirection<
            T,
        > = __cordl_object.invoke("get_dragDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startMousePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_startMousePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_dragging(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_dragging", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_draggingEnded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_draggingEnded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dragDirection(
        &mut self,
        value: crate::UnityEngine::UIElements::ClampedDragger_1_DragDirection<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dragDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_slider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseSlider_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_slider", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startMousePosition(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startMousePosition", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ClampedDragger_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1+DragDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClampedDragger_1_DragDirection {
    #[default]
    Free = 4i32,
    HighToLow = 2i32,
    LowToHigh = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+ClampedDragger_1+DragDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ClampedDragger_1_DragDirection => "UnityEngine.UIElements"
    ."ClampedDragger`1/DragDirection<T>"
);
