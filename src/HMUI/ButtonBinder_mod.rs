#[cfg(feature = "HMUI+ButtonBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ButtonBinder {
    __cordl_parent: crate::System::Object,
    pub _bindings: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Tuple_2<
            *mut crate::UnityEngine::UI::Button,
            *mut crate::UnityEngine::Events::UnityAction,
        >,
    >,
}
#[cfg(feature = "HMUI+ButtonBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ButtonBinder => "HMUI"."ButtonBinder"
);
#[cfg(feature = "HMUI+ButtonBinder")]
impl std::ops::Deref for crate::HMUI::ButtonBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ButtonBinder")]
impl std::ops::DerefMut for crate::HMUI::ButtonBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ButtonBinder")]
impl crate::HMUI::ButtonBinder {
    pub fn AddBinding(
        &mut self,
        button: *mut crate::UnityEngine::UI::Button,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBinding", (button, action))?;
        Ok(__cordl_ret)
    }
    pub fn AddBindings(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_2<
                *mut crate::UnityEngine::UI::Button,
                *mut crate::System::Action,
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Button_Action1(
        button: *mut crate::UnityEngine::UI::Button,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (button, action))?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_2(
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_2<
                *mut crate::UnityEngine::UI::Button,
                *mut crate::System::Action,
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
    pub fn _ctor_Button_Action1(
        &mut self,
        button: *mut crate::UnityEngine::UI::Button,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (button, action))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_2(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_2<
                *mut crate::UnityEngine::UI::Button,
                *mut crate::System::Action,
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
}
#[cfg(feature = "HMUI+ButtonBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ButtonBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
