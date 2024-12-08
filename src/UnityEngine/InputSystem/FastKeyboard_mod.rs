#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct FastKeyboard {
    __cordl_parent: crate::UnityEngine::InputSystem::Keyboard,
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::FastKeyboard =>
    "UnityEngine.InputSystem"."FastKeyboard"
);
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastKeyboard {
    type Target = crate::UnityEngine::InputSystem::Keyboard;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl crate::UnityEngine::InputSystem::FastKeyboard {
    pub const metadata: &'static str = ";AnyKey;Button;Axis;Key;DiscreteButton;Keyboard";
    pub fn Initialize_ctrlKeyboardf9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf9", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardslash(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardslash", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardOEM4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardOEM4", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardm(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardm", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftCtrl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftCtrl", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftAlt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftAlt", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard0(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard0", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightMeta(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightMeta", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad0(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad0", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardl", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf6", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf10(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf10", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardprintScreen(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardprintScreen", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard2", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardenter(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardenter", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardpageDown(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardpageDown", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardhome(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardhome", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardscrollLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardscrollLock", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad8", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboards(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboards", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad1", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardn(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardn", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardj(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardj", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboarda(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboarda", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad3", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardtab(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardtab", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardminus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardminus", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardk(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardk", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardinsert(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardinsert", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardOEM3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardOEM3", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardq(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardq", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboarddownArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboarddownArrow", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard1", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightBracket(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightBracket", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardctrl(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardctrl", (kDiscreteButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardb(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardb", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardi(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardi", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardbackquote(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardbackquote", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboarde(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboarde", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardg(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardg", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardbackspace(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardbackspace", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadMinus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadMinus", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad7", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardspace(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardspace", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad9", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadMultiply(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadMultiply", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardpause(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardpause", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad4", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf7", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftArrow", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardw(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardw", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardbackslash(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardbackslash", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardOEM5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardOEM5", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardalt(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardalt", (kDiscreteButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard7", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardo(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardo", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardend(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardend", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardt", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardy(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardy", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardOEM2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardOEM2", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftMeta(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftMeta", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad6", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftShift(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftShift", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightArrow", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardpageUp(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardpageUp", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboarddelete(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboarddelete", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardz(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardz", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardv(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardv", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardIMESelected(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardIMESelected", (kButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf2", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardc(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardc", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard6", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf5", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardu(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardu", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightShift(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightShift", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard9", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf4", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardshift(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardshift", (kDiscreteButtonLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightAlt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightAlt", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardupArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardupArrow", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardh(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardh", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf11(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf11", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard3", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardr(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardr", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardleftBracket(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardleftBracket", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf3", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardsemicolon(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardsemicolon", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardd(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardd", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadPeriod(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadPeriod", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardOEM1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardOEM1", (kKeyLayout, parent))?;
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
    pub fn Initialize_ctrlKeyboardx(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardx", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardrightCtrl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardrightCtrl", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardcapsLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardcapsLock", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadDivide(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadDivide", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf1", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardp(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardp", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumLock", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard8", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad2", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard5", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboard4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboard4", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardequals(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardequals", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardperiod(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardperiod", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadPlus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadPlus", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf12(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf12", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardanyKey(
        &mut self,
        kAnyKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AnyKeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AnyKeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardanyKey", (kAnyKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadEquals(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadEquals", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardcontextMenu(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardcontextMenu", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpadEnter(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpadEnter", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardcomma(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardcomma", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardquote(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardquote", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardf8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardf8", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardescape(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardescape", (kKeyLayout, parent))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_ctrlKeyboardnumpad5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::KeyControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::KeyControl = __cordl_object
            .invoke("Initialize_ctrlKeyboardnumpad5", (kKeyLayout, parent))?;
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
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::FastKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
