#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNatPunchListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub NatIntroductionRequest: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
    >,
    pub NatIntroductionSuccess: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
    >,
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::EventBasedNatPunchListener {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "EventBasedNatPunchListener";
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
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNatPunchListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        localEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INatPunchListener.OnNatIntroductionRequest",
                (localEndPoint, remoteEndPoint, token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INatPunchListener_OnNatIntroductionSuccess(
        &mut self,
        targetEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INatPunchListener.OnNatIntroductionSuccess",
                (targetEndPoint, _cordl_type, token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NatIntroductionRequest(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NatIntroductionRequest", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NatIntroductionSuccess(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NatIntroductionSuccess", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NatIntroductionRequest(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NatIntroductionRequest", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NatIntroductionSuccess(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NatIntroductionSuccess", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl AsRef<crate::LiteNetLib::INatPunchListener>
for crate::LiteNetLib::EventBasedNatPunchListener {
    fn as_ref(&self) -> &crate::LiteNetLib::INatPunchListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener")]
impl AsMut<crate::LiteNetLib::INatPunchListener>
for crate::LiteNetLib::EventBasedNatPunchListener {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::INatPunchListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNatPunchListener_OnNatIntroductionRequest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "LiteNetLib+EventBasedNatPunchListener+OnNatIntroductionRequest")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionRequest {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "OnNatIntroductionRequest";
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
        localEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                (localEndPoint, remoteEndPoint, token, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        localEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (localEndPoint, remoteEndPoint, token))?;
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
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::EventBasedNatPunchListener_OnNatIntroductionSuccess {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "OnNatIntroductionSuccess";
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
        targetEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                (targetEndPoint, _cordl_type, token, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        targetEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (targetEndPoint, _cordl_type, token))?;
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
