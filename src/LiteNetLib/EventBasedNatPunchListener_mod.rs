#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNatPunchListener {
    __cordl_parent: crate::System::Object,
    pub NatIntroductionRequest: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
    pub NatIntroductionSuccess: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::EventBasedNatPunchListener =>
    "LiteNetLib"."EventBasedNatPunchListener"
);
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNatPunchListener {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNatPunchListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl crate::LiteNetLib::EventBasedNatPunchListener {
    #[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
    pub type OnNatIntroductionRequest = crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest;
    #[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
    pub type OnNatIntroductionSuccess = crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess;
    pub fn LiteNetLib_INatPunchListener_OnNatIntroductionRequest(
        &mut self,
        localEndPoint: *mut crate::System::Net::IPEndPoint,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        token: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INatPunchListener.OnNatIntroductionRequest",
                (localEndPoint, remoteEndPoint, token),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INatPunchListener_OnNatIntroductionSuccess(
        &mut self,
        targetEndPoint: *mut crate::System::Net::IPEndPoint,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INatPunchListener.OnNatIntroductionSuccess",
                (targetEndPoint, _cordl_type, token),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn add_NatIntroductionRequest(
        &mut self,
        value: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NatIntroductionRequest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_NatIntroductionSuccess(
        &mut self,
        value: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NatIntroductionSuccess", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_NatIntroductionRequest(
        &mut self,
        value: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NatIntroductionRequest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_NatIntroductionSuccess(
        &mut self,
        value: *mut crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NatIntroductionSuccess", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNatPunchListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNatPunchListener_OnNatIntroductionRequest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest => "LiteNetLib"
    ."EventBasedNatPunchListener/OnNatIntroductionRequest"
);
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
impl std::ops::Deref
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
impl std::ops::DerefMut
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
impl crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest {
    pub fn BeginInvoke(
        &mut self,
        localEndPoint: *mut crate::System::Net::IPEndPoint,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        token: *mut crate::System::String,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (localEndPoint, remoteEndPoint, token, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        localEndPoint: *mut crate::System::Net::IPEndPoint,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        token: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (localEndPoint, remoteEndPoint, token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNatPunchListener_OnNatIntroductionSuccess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess => "LiteNetLib"
    ."EventBasedNatPunchListener/OnNatIntroductionSuccess"
);
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
impl std::ops::Deref
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
impl std::ops::DerefMut
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
impl crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess {
    pub fn BeginInvoke(
        &mut self,
        targetEndPoint: *mut crate::System::Net::IPEndPoint,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: *mut crate::System::String,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (targetEndPoint, _cordl_type, token, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        targetEndPoint: *mut crate::System::Net::IPEndPoint,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (targetEndPoint, _cordl_type, token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionSuccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
