#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRNotifications {
    pub CreateNotification: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRNotifications__CreateNotification,
    >,
    pub RemoveNotification: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRNotifications__RemoveNotification,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRNotifications {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRNotifications";
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRNotifications {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRNotifications {
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRNotifications {
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRNotifications {
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::IVRNotifications {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
impl crate::OVR::OpenVR::IVRNotifications {
    #[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
    pub type _CreateNotification = crate::OVR::OpenVR::IVRNotifications__CreateNotification;
    #[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
    pub type _RemoveNotification = crate::OVR::OpenVR::IVRNotifications__RemoveNotification;
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRNotifications__CreateNotification {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRNotifications/_CreateNotification";
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        ulUserValue: u64,
        _cordl_type: crate::OVR::OpenVR::EVRNotificationType,
        pchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::OVR::OpenVR::EVRNotificationStyle,
        pImage: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::NotificationBitmap_t,
        >,
        pNotificationId: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u64,
                            crate::OVR::OpenVR::EVRNotificationType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::OVR::OpenVR::EVRNotificationStyle,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::NotificationBitmap_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        9usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        ulUserValue,
                        _cordl_type,
                        pchText,
                        style,
                        pImage,
                        pNotificationId,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pImage: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::NotificationBitmap_t,
        >,
        pNotificationId: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::NotificationBitmap_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRNotificationError,
                        3usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pImage, pNotificationId, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        ulUserValue: u64,
        _cordl_type: crate::OVR::OpenVR::EVRNotificationType,
        pchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::OVR::OpenVR::EVRNotificationStyle,
        pImage: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::NotificationBitmap_t,
        >,
        pNotificationId: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u64,
                            crate::OVR::OpenVR::EVRNotificationType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::OVR::OpenVR::EVRNotificationStyle,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::NotificationBitmap_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRNotificationError,
                        7usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            7usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        ulUserValue,
                        _cordl_type,
                        pchText,
                        style,
                        pImage,
                        pNotificationId,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRNotifications__RemoveNotification {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRNotifications/_RemoveNotification";
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
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    pub fn BeginInvoke(
        &mut self,
        notificationId: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (notificationId, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        crate::OVR::OpenVR::EVRNotificationError,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        notificationId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        crate::OVR::OpenVR::EVRNotificationError,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = unsafe {
            cordl_method_info.invoke_unchecked(self, (notificationId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
