#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownMenuAction {
    __cordl_parent: crate::UnityEngine::UIElements::DropdownMenuItem,
    pub _name_k__BackingField: *mut crate::System::String,
    pub _status_k__BackingField: crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    pub _eventInfo_k__BackingField: *mut crate::UnityEngine::UIElements::DropdownMenuEventInfo,
    pub _userData_k__BackingField: *mut crate::System::Object,
    pub actionCallback: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::DropdownMenuAction,
    >,
    pub actionStatusCallback: *mut crate::System::Func_2<
        *mut crate::UnityEngine::UIElements::DropdownMenuAction,
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DropdownMenuAction =>
    "UnityEngine.UIElements"."DropdownMenuAction"
);
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DropdownMenuAction {
    type Target = crate::UnityEngine::UIElements::DropdownMenuItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DropdownMenuAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
impl crate::UnityEngine::UIElements::DropdownMenuAction {
    #[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction+Status")]
    pub type Status = crate::UnityEngine::UIElements::DropdownMenuAction_Status;
    pub fn New(
        actionName: *mut crate::System::String,
        actionCallback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
        >,
        actionStatusCallback: *mut crate::System::Func_2<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
            crate::UnityEngine::UIElements::DropdownMenuAction_Status,
        >,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (actionName, actionCallback, actionStatusCallback, userData),
            )?;
        Ok(__cordl_object)
    }
    pub fn UpdateActionStatus(
        &mut self,
        eventInfo: *mut crate::UnityEngine::UIElements::DropdownMenuEventInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateActionStatus", (eventInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        actionName: *mut crate::System::String,
        actionCallback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
        >,
        actionStatusCallback: *mut crate::System::Func_2<
            *mut crate::UnityEngine::UIElements::DropdownMenuAction,
            crate::UnityEngine::UIElements::DropdownMenuAction_Status,
        >,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (actionName, actionCallback, actionStatusCallback, userData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_eventInfo(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::DropdownMenuEventInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_status(
        &mut self,
        value: crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_status", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_userData(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userData", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DropdownMenuAction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction+Status")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownMenuAction_Status {
    Checked = 4i32,
    Disabled = 2i32,
    Hidden = 8i32,
    None = 0i32,
    Normal = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction+Status")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DropdownMenuAction_Status => "UnityEngine.UIElements"
    ."DropdownMenuAction/Status"
);
