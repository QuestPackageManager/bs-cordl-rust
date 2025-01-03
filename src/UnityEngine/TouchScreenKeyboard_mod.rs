#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchScreenKeyboard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchScreenKeyboard =>
    "UnityEngine"."TouchScreenKeyboard"
);
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
impl std::ops::Deref for crate::UnityEngine::TouchScreenKeyboard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
impl std::ops::DerefMut for crate::UnityEngine::TouchScreenKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
impl crate::UnityEngine::TouchScreenKeyboard {
    #[cfg(feature = "UnityEngine+TouchScreenKeyboard+Status")]
    pub type Status = crate::UnityEngine::TouchScreenKeyboard_Status;
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelection(
        start: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSelection", (start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Destroy(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Destroy", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInPlaceEditingAllowed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInPlaceEditingAllowed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        textPlaceholder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        characterLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    text,
                    keyboardType,
                    autocorrection,
                    multiline,
                    secure,
                    alert,
                    textPlaceholder,
                    characterLimit,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Open_Il2CppString_TouchScreenKeyboardType__cordl_bool__cordl_bool__cordl_bool1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TouchScreenKeyboard>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TouchScreenKeyboard,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (text, keyboardType, autocorrection, multiline, secure))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open__cordl_bool_Il2CppString_i32_0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        textPlaceholder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        characterLimit: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TouchScreenKeyboard>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TouchScreenKeyboard,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Open",
                (
                    text,
                    keyboardType,
                    autocorrection,
                    multiline,
                    secure,
                    alert,
                    textPlaceholder,
                    characterLimit,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelection(
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSelection", (start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn TouchScreenKeyboard_InternalConstructorHelper(
        arguments: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TouchScreenKeyboard_InternalConstructorHelperArguments,
        >,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        textPlaceholder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TouchScreenKeyboard_InternalConstructorHelper",
                (arguments, text, textPlaceholder),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        textPlaceholder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        characterLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    text,
                    keyboardType,
                    autocorrection,
                    multiline,
                    secure,
                    alert,
                    textPlaceholder,
                    characterLimit,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_active(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_active", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canGetSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canGetSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canSetSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canSetSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disableInPlaceEditing() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_disableInPlaceEditing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInPlaceEditingAllowed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isInPlaceEditingAllowed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RangeInt> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RangeInt = __cordl_object
            .invoke("get_selection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchScreenKeyboard_Status> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TouchScreenKeyboard_Status = __cordl_object
            .invoke("get_status", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_text", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_active(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_active", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_characterLimit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hideInput(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_hideInput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selection(
        &mut self,
        value: crate::UnityEngine::RangeInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_text(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TouchScreenKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard+Status")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchScreenKeyboard_Status {
    Canceled = 2i32,
    Done = 1i32,
    LostFocus = 3i32,
    Visible = 0i32,
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard+Status")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchScreenKeyboard_Status =>
    "UnityEngine"."TouchScreenKeyboard/Status"
);
