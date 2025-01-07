#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownMenuAction {
    __cordl_parent: crate::UnityEngine::UIElements::DropdownMenuItem,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _status_k__BackingField: crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    pub _eventInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::DropdownMenuEventInfo,
    >,
    pub _userData_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub actionCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
        >,
    >,
    pub actionStatusCallback: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::DropdownMenuAction,
            >,
            crate::UnityEngine::UIElements::DropdownMenuAction_Status,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::DropdownMenuAction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "DropdownMenuAction";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn AlwaysDisabled(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DropdownMenuAction_Status = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlwaysDisabled", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlwaysEnabled(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DropdownMenuAction_Status = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlwaysEnabled", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        actionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actionCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::DropdownMenuAction,
                >,
            >,
        >,
        actionStatusCallback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::DropdownMenuAction,
                >,
                crate::UnityEngine::UIElements::DropdownMenuAction_Status,
            >,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (actionName, actionCallback, actionStatusCallback, userData),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateActionStatus(
        &mut self,
        eventInfo: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DropdownMenuEventInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateActionStatus", (eventInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        actionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actionCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::DropdownMenuAction,
                >,
            >,
        >,
        actionStatusCallback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::DropdownMenuAction,
                >,
                crate::UnityEngine::UIElements::DropdownMenuAction_Status,
            >,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (actionName, actionCallback, actionStatusCallback, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eventInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DropdownMenuEventInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventInfo", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_userData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userData", (value))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuAction_Status {
    #[default]
    Checked = 4i32,
    Disabled = 2i32,
    Hidden = 8i32,
    None = 0i32,
    Normal = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuAction+Status")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::DropdownMenuAction_Status {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "Status";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::DropdownMenuAction_Status {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::DropdownMenuAction_Status {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::DropdownMenuAction_Status {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::DropdownMenuAction_Status {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
