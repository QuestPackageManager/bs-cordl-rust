#[cfg(feature = "HMUI+MouseBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseBinder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _enabled_k__BackingField: bool,
    pub _scrollBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Events::UnityAction_1<f32>,
        >,
    >,
    pub _buttonBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::ValueTuple_3<
                crate::HMUI::MouseBinder_ButtonType,
                crate::HMUI::MouseBinder_MouseEventType,
                *mut crate::UnityEngine::Events::UnityAction,
            >,
        >,
    >,
}
#[cfg(feature = "HMUI+MouseBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::MouseBinder => "HMUI"."MouseBinder"
);
#[cfg(feature = "HMUI+MouseBinder")]
impl std::ops::Deref for crate::HMUI::MouseBinder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+MouseBinder")]
impl std::ops::DerefMut for crate::HMUI::MouseBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+MouseBinder")]
impl crate::HMUI::MouseBinder {
    #[cfg(feature = "HMUI+MouseBinder+ButtonType")]
    pub type ButtonType = crate::HMUI::MouseBinder_ButtonType;
    #[cfg(feature = "HMUI+MouseBinder+MouseEventType")]
    pub type MouseEventType = crate::HMUI::MouseBinder_MouseEventType;
    pub fn AddButtonBinding(
        &mut self,
        buttonType: crate::HMUI::MouseBinder_ButtonType,
        keyBindingType: crate::HMUI::MouseBinder_MouseEventType,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddButtonBinding", (buttonType, keyBindingType, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddButtonBindings(
        &mut self,
        bindingData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Tuple_3<
                    crate::HMUI::MouseBinder_ButtonType,
                    crate::HMUI::MouseBinder_MouseEventType,
                    *mut crate::UnityEngine::Events::UnityAction,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddButtonBindings", (bindingData))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddScrollBinding(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScrollBinding", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddScrollBindings(
        &mut self,
        bindingData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Events::UnityAction_1<f32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScrollBindings", (bindingData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveButtonBinding(
        &mut self,
        buttonType: crate::HMUI::MouseBinder_ButtonType,
        keyBindingType: crate::HMUI::MouseBinder_MouseEventType,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveButtonBinding", (buttonType, keyBindingType, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveScrollBinding(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveScrollBinding", (action))?;
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
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enabled", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+MouseBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::MouseBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+MouseBinder+ButtonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseBinder_ButtonType {
    Middle = 2i32,
    Primary = 0i32,
    Secondary = 1i32,
}
#[cfg(feature = "HMUI+MouseBinder+ButtonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::MouseBinder_ButtonType => "HMUI"
    ."MouseBinder/ButtonType"
);
#[cfg(feature = "HMUI+MouseBinder+MouseEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseBinder_MouseEventType {
    ButtonDown = 0i32,
    ButtonPress = 2i32,
    ButtonUp = 1i32,
}
#[cfg(feature = "HMUI+MouseBinder+MouseEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::MouseBinder_MouseEventType => "HMUI"
    ."MouseBinder/MouseEventType"
);
