#[cfg(feature = "HMUI+KeyboardBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyboardBinder {
    __cordl_parent: crate::System::Object,
    pub _enabled_k__BackingField: bool,
    pub _shouldClearBindings: bool,
    pub _newBindings: *mut crate::System::Collections::Generic::List_1<
        crate::System::ValueTuple_3<
            crate::UnityEngine::KeyCode,
            crate::HMUI::KeyboardBinder_KeyBindingType,
            *mut crate::UnityEngine::Events::UnityAction_1<bool>,
        >,
    >,
    pub _bindings: *mut crate::System::Collections::Generic::List_1<
        crate::System::ValueTuple_3<
            crate::UnityEngine::KeyCode,
            crate::HMUI::KeyboardBinder_KeyBindingType,
            *mut crate::UnityEngine::Events::UnityAction_1<bool>,
        >,
    >,
}
#[cfg(feature = "HMUI+KeyboardBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::KeyboardBinder => "HMUI"."KeyboardBinder"
);
#[cfg(feature = "HMUI+KeyboardBinder")]
impl std::ops::Deref for crate::HMUI::KeyboardBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+KeyboardBinder")]
impl std::ops::DerefMut for crate::HMUI::KeyboardBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+KeyboardBinder")]
impl crate::HMUI::KeyboardBinder {
    #[cfg(feature = "HMUI+KeyboardBinder+KeyBindingType")]
    pub type KeyBindingType = crate::HMUI::KeyboardBinder_KeyBindingType;
    pub fn AddBinding(
        &mut self,
        keyCode: crate::UnityEngine::KeyCode,
        keyBindingType: crate::HMUI::KeyboardBinder_KeyBindingType,
        action: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBinding", (keyCode, keyBindingType, action))?;
        Ok(__cordl_ret)
    }
    pub fn AddBindings(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_3<
                crate::UnityEngine::KeyCode,
                crate::HMUI::KeyboardBinder_KeyBindingType,
                *mut crate::System::Action_1<bool>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBindings", (bindingData))?;
        Ok(__cordl_ret)
    }
    pub fn ClearBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_KeyCode_KeyboardBinder_KeyBindingType_Action_1_1(
        keycode: crate::UnityEngine::KeyCode,
        keyBindingType: crate::HMUI::KeyboardBinder_KeyBindingType,
        action: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keycode, keyBindingType, action))?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_2(
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_3<
                crate::UnityEngine::KeyCode,
                crate::HMUI::KeyboardBinder_KeyBindingType,
                *mut crate::System::Action_1<bool>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindingData))?;
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
    pub fn _ctor_KeyCode_KeyboardBinder_KeyBindingType_Action_1_1(
        &mut self,
        keycode: crate::UnityEngine::KeyCode,
        keyBindingType: crate::HMUI::KeyboardBinder_KeyBindingType,
        action: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keycode, keyBindingType, action))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_2(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_3<
                crate::UnityEngine::KeyCode,
                crate::HMUI::KeyboardBinder_KeyBindingType,
                *mut crate::System::Action_1<bool>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindingData))?;
        Ok(__cordl_ret)
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+KeyboardBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::KeyboardBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+KeyboardBinder+KeyBindingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardBinder_KeyBindingType {
    KeyDown = 0i32,
    KeyPress = 2i32,
    KeyUp = 1i32,
}
#[cfg(feature = "HMUI+KeyboardBinder+KeyBindingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::KeyboardBinder_KeyBindingType => "HMUI"
    ."KeyboardBinder/KeyBindingType"
);
