#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRNotifications {
    pub CreateNotification: *mut crate::OVR::OpenVR::IVRNotifications__CreateNotification,
    pub RemoveNotification: *mut crate::OVR::OpenVR::IVRNotifications__RemoveNotification,
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRNotifications => "OVR.OpenVR"
    ."IVRNotifications"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRNotifications__CreateNotification => "OVR.OpenVR"
    ."IVRNotifications/_CreateNotification"
);
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_CreateNotification")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRNotifications__CreateNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
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
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke("EndInvoke", (pImage, pNotificationId, result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke(
                "Invoke",
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRNotifications__RemoveNotification => "OVR.OpenVR"
    ."IVRNotifications/_RemoveNotification"
);
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRNotifications+_RemoveNotification")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRNotifications__RemoveNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (notificationId, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        notificationId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRNotificationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRNotificationError = __cordl_object
            .invoke("Invoke", (notificationId))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
