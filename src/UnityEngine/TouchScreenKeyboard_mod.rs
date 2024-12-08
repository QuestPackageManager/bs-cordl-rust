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
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchScreenKeyboard {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchScreenKeyboard =>
    "UnityEngine"."TouchScreenKeyboard"
);
#[cfg(feature = "UnityEngine+TouchScreenKeyboard")]
impl std::ops::Deref for crate::UnityEngine::TouchScreenKeyboard {
    type Target = crate::System::Object;
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
    pub fn get_active(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_active", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_canSetSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canSetSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchScreenKeyboard_Status> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TouchScreenKeyboard_Status = __cordl_object
            .invoke("get_status", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RangeInt> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RangeInt = __cordl_object
            .invoke("get_selection", ())?;
        Ok(__cordl_ret)
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
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
    pub fn set_characterLimit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterLimit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_canGetSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canGetSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        text: *mut crate::System::String,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        textPlaceholder: *mut crate::System::String,
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
    pub fn set_active(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_active", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        text: *mut crate::System::String,
        keyboardType: crate::UnityEngine::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        textPlaceholder: *mut crate::System::String,
        characterLimit: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
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
