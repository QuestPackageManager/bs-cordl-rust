#[cfg(feature = "cordl_class_OVR+OpenVR+CVRNotifications")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRNotifications {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRNotifications,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRNotifications")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRNotifications {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRNotifications";
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
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRNotifications {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRNotifications {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl crate::OVR::OpenVR::CVRNotifications {
    pub fn CreateNotification(
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
                    >("CreateNotification")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateNotification", 7usize
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
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveNotification(
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
                    >("RemoveNotification")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveNotification", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = unsafe {
            cordl_method_info.invoke_unchecked(self, (notificationId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRNotifications")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRNotifications {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
