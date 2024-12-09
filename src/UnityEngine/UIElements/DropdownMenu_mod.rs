#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownMenu {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MenuItems: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::DropdownMenuItem,
    >,
    pub m_DropdownMenuEventInfo: *mut crate::UnityEngine::UIElements::DropdownMenuEventInfo,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DropdownMenu =>
    "UnityEngine.UIElements"."DropdownMenu"
);
#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DropdownMenu {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DropdownMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
impl crate::UnityEngine::UIElements::DropdownMenu {
    #[cfg(feature = "UnityEngine+UIElements+DropdownMenu+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::UnityEngine::UIElements::DropdownMenu___c__DisplayClass4_0;
    pub fn AppendAction_DropdownMenuAction_Status1(
        &mut self,
        actionName: *mut quest_hook::libil2cpp::Il2CppString,
        action: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
        >,
        status: crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendAction", (actionName, action, status))?;
        Ok(__cordl_ret)
    }
    pub fn AppendAction_Func_2_Il2CppObject0(
        &mut self,
        actionName: *mut quest_hook::libil2cpp::Il2CppString,
        action: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
        >,
        actionStatusCallback: *mut crate::System::Func_2<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
            crate::UnityEngine::UIElements::DropdownMenuAction_Status,
        >,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AppendAction",
                (actionName, action, actionStatusCallback, userData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AppendSeparator(
        &mut self,
        subMenuPath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendSeparator", (subMenuPath))?;
        Ok(__cordl_ret)
    }
    pub fn InsertSeparator(
        &mut self,
        subMenuPath: *mut quest_hook::libil2cpp::Il2CppString,
        atIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertSeparator", (subMenuPath, atIndex))?;
        Ok(__cordl_ret)
    }
    pub fn MenuItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuItem,
        > = __cordl_object.invoke("MenuItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PrepareForDisplay(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareForDisplay", (e))?;
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
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenu")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::DropdownMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
