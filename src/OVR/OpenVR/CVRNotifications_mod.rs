#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRNotifications {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub FnTable: crate::OVR::OpenVR::IVRNotifications,
}
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRNotifications => "OVR.OpenVR"
    ."CVRNotifications"
);
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRNotifications {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRNotifications {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke(
                "CreateNotification",
                (
                    ulOverlayHandle,
                    ulUserValue,
                    _cordl_type,
                    pchText,
                    style,
                    pImage,
                    pNotificationId,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke("RemoveNotification", (notificationId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRNotifications")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRNotifications {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
