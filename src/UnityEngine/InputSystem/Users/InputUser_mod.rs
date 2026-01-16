#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser {
    pub m_Id: u32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputSystem::Users::InputUser {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::InputSystem::Users::InputUser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::InputSystem::Users::InputUser {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::InputSystem::Users::InputUser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::InputSystem::Users::InputUser {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl crate::UnityEngine::InputSystem::Users::InputUser {
    pub const InvalidId: u32 = 0u32;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
    pub type CompareDevicesByUserAccount =
        crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
    pub type ControlSchemeChangeSyntax =
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::Users::InputUser_GlobalState;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
    pub type OngoingAccountSelection =
        crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
    pub type UserData = crate::UnityEngine::InputSystem::Users::InputUser_UserData;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserFlags")]
    pub type UserFlags = crate::UnityEngine::InputSystem::Users::InputUser_UserFlags;
    pub fn ActivateControlSchemeInternal(
        &mut self,
        userIndex: i32,
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::InputSystem::InputControlScheme),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ActivateControlSchemeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActivateControlSchemeInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (userIndex, scheme))? };
        Ok(__cordl_ret.into())
    }
    pub fn ActivateControlScheme_Il2CppString0(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
                        1usize,
                    >("ActivateControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActivateControlScheme", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = unsafe {
            cordl_method_info.invoke_unchecked(self, (schemeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ActivateControlScheme_InputControlScheme1(
        &mut self,
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::InputControlScheme),
                        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
                        1usize,
                    >("ActivateControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActivateControlScheme", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = unsafe {
            cordl_method_info.invoke_unchecked(self, (scheme))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDeviceToUser(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        asLostDevice: bool,
        dontUpdateControlScheme: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("AddDeviceToUser")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddDeviceToUser",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (userIndex, device, asLostDevice, dontUpdateControlScheme),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddUser() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("AddUser")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddUser",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn AssociateActionsWithUser(
        &mut self,
        actions: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::IInputActionCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::IInputActionCollection,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "AssociateActionsWithUser"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AssociateActionsWithUser",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (actions))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUserWithoutPairedDevices(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Users::InputUser> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Users::InputUser,
                        0usize,
                    >("CreateUserWithoutPairedDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUserWithoutPairedDevices", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAndResetGlobalState() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "DisposeAndResetGlobalState",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisposeAndResetGlobalState",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputUser0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Users::InputUser),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindControlScheme(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::InputControlScheme,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("FindControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindControlScheme",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schemeName, scheme))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindLostDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        i32,
                    ), i32, 2usize>("FindLostDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindLostDevice",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (device, startIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindUserByAccount(
        platformUserAccountHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Users::InputUserAccountHandle),
                        crate::System::Nullable_1<
                            crate::UnityEngine::InputSystem::Users::InputUser,
                        >,
                        1usize,
                    >("FindUserByAccount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindUserByAccount", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = unsafe { cordl_method_info.invoke_unchecked((), (platformUserAccountHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindUserPairedToDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        crate::System::Nullable_1<
                            crate::UnityEngine::InputSystem::Users::InputUser,
                        >,
                        1usize,
                    >("FindUserPairedToDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindUserPairedToDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = unsafe { cordl_method_info.invoke_unchecked((), (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnpairedInputDevices_0() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::InputSystem::InputControlList_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    >, 0usize>("GetUnpairedInputDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUnpairedInputDevices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnpairedInputDevices_ByRefMut1(
        list: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputControlList_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        >,
                    >), i32, 1usize>("GetUnpairedInputDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUnpairedInputDevices",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoActionChange() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "HookIntoActionChange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HookIntoActionChange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoDeviceChange() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "HookIntoDeviceChange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HookIntoDeviceChange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoEvents() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("HookIntoEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HookIntoEvents",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InitiateUserAccountSelection(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        options: crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
                    ), bool, 3usize>("InitiateUserAccountSelection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitiateUserAccountSelection",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex, device, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitiateUserAccountSelectionAtPlatformLevel(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        bool,
                        1usize,
                    >("InitiateUserAccountSelectionAtPlatformLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitiateUserAccountSelectionAtPlatformLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn Notify(
        userIndex: i32,
        change: crate::UnityEngine::InputSystem::Users::InputUserChange,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        crate::UnityEngine::InputSystem::Users::InputUserChange,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Notify")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Notify",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex, change, device))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnActionChange(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        change: crate::UnityEngine::InputSystem::InputActionChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::UnityEngine::InputSystem::InputActionChange,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnActionChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnActionChange",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (obj, change))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnDeviceChange(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        crate::UnityEngine::InputSystem::InputDeviceChange,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnDeviceChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnDeviceChange",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (device, change))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEvent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventPtr, device))? };
        Ok(__cordl_ret.into())
    }
    pub fn PerformPairingWithDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        user: crate::UnityEngine::InputSystem::Users::InputUser,
        options: crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Users::InputUser> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        crate::UnityEngine::InputSystem::Users::InputUser,
                        crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
                    ), crate::UnityEngine::InputSystem::Users::InputUser, 3usize>(
                        "PerformPairingWithDevice",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PerformPairingWithDevice",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser =
            unsafe { cordl_method_info.invoke_unchecked((), (device, user, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn QueryPairedPlatformUserAccount(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        platformAccountHandle: quest_hook::libil2cpp::ByRefMut<
            crate::System::Nullable_1<
                crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
            >,
        >,
        platformAccountName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        platformAccountId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Nullable_1<
                                crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    ), i64, 4usize>("QueryPairedPlatformUserAccount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QueryPairedPlatformUserAccount",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    device,
                    platformAccountHandle,
                    platformAccountName,
                    platformAccountId,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDeviceFromUser(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        asLostDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RemoveDeviceFromUser"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RemoveDeviceFromUser",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex, device, asLostDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLostDevicesForUser(
        userIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "RemoveLostDevicesForUser",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RemoveLostDevicesForUser",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveUser(
        userIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("RemoveUser")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RemoveUser",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetGlobals() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetGlobals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetGlobals",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAndResetState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Utilities::ISavedState>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Utilities::ISavedState,
                    >, 0usize>("SaveAndResetState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveAndResetState",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControlScheme(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::InputControlScheme,
                        >,
                    ), bool, 2usize>("TryFindControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryFindControlScheme",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (schemeName, scheme))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_InputDevice2(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        i32,
                        1usize,
                    >("TryFindUserIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFindUserIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_InputUserAccountHandle1(
        platformHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Users::InputUserAccountHandle),
                        i32,
                        1usize,
                    >("TryFindUserIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFindUserIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (platformHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_u32_0(userId: u32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), i32, 1usize>("TryFindUserIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryFindUserIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (userId))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromActionChange() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnhookFromActionChange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnhookFromActionChange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromDeviceChange() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnhookFromDeviceChange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnhookFromDeviceChange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromDeviceStateChange(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnhookFromDeviceStateChange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnhookFromDeviceStateChange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnpairDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnpairDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevices(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UnpairDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnpairDevices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevicesAndRemoveUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnpairDevicesAndRemoveUser",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnpairDevicesAndRemoveUser",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateControlSchemeMatch(
        userIndex: i32,
        autoPairMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, bool), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateControlSchemeMatch",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateControlSchemeMatch",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex, autoPairMissing))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePlatformUserAccount(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), i64, 2usize>("UpdatePlatformUserAccount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdatePlatformUserAccount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i64 =
            unsafe { cordl_method_info.invoke_unchecked((), (userIndex, device))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_3<
                            crate::UnityEngine::InputSystem::Users::InputUser,
                            crate::UnityEngine::InputSystem::Users::InputUserChange,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("add_onChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_onChange",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onPrefilterUnpairedDeviceActivity(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            bool,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_onPrefilterUnpairedDeviceActivity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_onPrefilterUnpairedDeviceActivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onUnpairedDeviceUsed(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_onUnpairedDeviceUsed"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_onUnpairedDeviceUsed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_actions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::IInputActionCollection>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::IInputActionCollection,
                    >, 0usize>("get_actions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_actions",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_all() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                            crate::UnityEngine::InputSystem::Users::InputUser,
                        >,
                        0usize,
                    >("get_all")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_all",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputControlScheme>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<
                            crate::UnityEngine::InputSystem::InputControlScheme,
                        >,
                        0usize,
                    >("get_controlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_controlScheme", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlSchemeMatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
                        0usize,
                    >("get_controlSchemeMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_controlSchemeMatch", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme_MatchResult =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasMissingRequiredDevices(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasMissingRequiredDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hasMissingRequiredDevices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_id")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_id",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_index",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_listenForUnpairedDeviceActivity() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_listenForUnpairedDeviceActivity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_listenForUnpairedDeviceActivity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lostDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    >, 0usize>("get_lostDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_lostDevices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pairedDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    >, 0usize>("get_pairedDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pairedDevices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUserAccountHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Nullable_1<
                        crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
                    >, 0usize>("get_platformUserAccountHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_platformUserAccountHandle",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_platformUserAccountId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_platformUserAccountId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_platformUserAccountName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_platformUserAccountName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_valid",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Users::InputUser,
        right: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::Users::InputUser,
                        crate::UnityEngine::InputSystem::Users::InputUser,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Users::InputUser,
        right: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::Users::InputUser,
                        crate::UnityEngine::InputSystem::Users::InputUser,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_3<
                            crate::UnityEngine::InputSystem::Users::InputUser,
                            crate::UnityEngine::InputSystem::Users::InputUserChange,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("remove_onChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_onChange",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onPrefilterUnpairedDeviceActivity(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            bool,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_onPrefilterUnpairedDeviceActivity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_onPrefilterUnpairedDeviceActivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onUnpairedDeviceUsed(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_onUnpairedDeviceUsed"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_onUnpairedDeviceUsed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_listenForUnpairedDeviceActivity(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_listenForUnpairedDeviceActivity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_listenForUnpairedDeviceActivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser>>
    for crate::UnityEngine::InputSystem::Users::InputUser
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser>>
    for crate::UnityEngine::InputSystem::Users::InputUser
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser_CompareDevicesByUserAccount {
    pub platformUserAccountHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/CompareDevicesByUserAccount";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount {
    pub fn Compare(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), i32, 2usize>("Compare")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Compare",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUserAccountHandleForDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUserAccountHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        crate::System::Nullable_1<
                            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
                        >,
                        1usize,
                    >("GetUserAccountHandleForDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUserAccountHandleForDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        > = unsafe { cordl_method_info.invoke_unchecked((), (device))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl
    AsRef<
        crate::System::Collections::Generic::IComparer_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl
    AsMut<
        crate::System::Collections::Generic::IComparer_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser_ControlSchemeChangeSyntax {
    pub m_UserIndex: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/ControlSchemeChangeSyntax";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
impl crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax {
    pub fn AndPairRemainingDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
                        0usize,
                    >("AndPairRemainingDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AndPairRemainingDevices", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser_GlobalState {
    pub pairingStateVersion: i32,
    pub lastUserId: u32,
    pub allUserCount: i32,
    pub allPairedDeviceCount: i32,
    pub allLostDeviceCount: i32,
    pub allUsers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::InputSystem::Users::InputUser>,
    >,
    pub allUserData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Users::InputUser_UserData,
        >,
    >,
    pub allPairedDevices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub allLostDevices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub ongoingAccountSelections: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection,
    >,
    pub onChange: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    >,
    pub onUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    >,
    pub onPreFilterUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    >,
    pub actionChangeDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            crate::UnityEngine::InputSystem::InputActionChange,
        >,
    >,
    pub onDeviceChangeDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    >,
    pub onEventDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub onActionChangeHooked: bool,
    pub onDeviceChangeHooked: bool,
    pub onEventHooked: bool,
    pub listenForUnpairedDeviceActivity: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/GlobalState";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
impl crate::UnityEngine::InputSystem::Users::InputUser_GlobalState {}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser_OngoingAccountSelection {
    pub device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    pub userId: u32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/OngoingAccountSelection";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
impl crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection {}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputUser_UserData {
    pub platformUserAccountHandle:
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUserAccountHandle>,
    pub platformUserAccountName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformUserAccountId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceCount: i32,
    pub deviceStartIndex: i32,
    pub actions: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::IInputActionCollection>,
    pub controlScheme:
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputControlScheme>,
    pub controlSchemeMatch: crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
    pub lostDeviceCount: i32,
    pub lostDeviceStartIndex: i32,
    pub flags: crate::UnityEngine::InputSystem::Users::InputUser_UserFlags,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/UserData";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Users::InputUser_UserData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
impl crate::UnityEngine::InputSystem::Users::InputUser_UserData {}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum InputUser_UserFlags {
    #[default]
    BindToAllDevices = 1i32,
    UserAccountSelectionInProgress = 2i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Users::InputUser_UserFlags
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Users";
    const CLASS_NAME: &'static str = "InputUser/UserFlags";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Users::InputUser_UserFlags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Users::InputUser_UserFlags
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Users::InputUser_UserFlags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Users+InputUser+UserFlags")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Users::InputUser_UserFlags
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
