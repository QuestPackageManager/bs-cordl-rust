#[cfg(feature = "LiteNetLib+INatPunchListener")]
#[repr(C)]
#[derive(Debug)]
pub struct INatPunchListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+INatPunchListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::INatPunchListener => "LiteNetLib"
    ."INatPunchListener"
);
#[cfg(feature = "LiteNetLib+INatPunchListener")]
impl std::ops::Deref for crate::LiteNetLib::INatPunchListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INatPunchListener")]
impl std::ops::DerefMut for crate::LiteNetLib::INatPunchListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INatPunchListener")]
impl crate::LiteNetLib::INatPunchListener {
    pub fn OnNatIntroductionRequest(
        &mut self,
        localEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNatIntroductionRequest", (localEndPoint, remoteEndPoint, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNatIntroductionSuccess(
        &mut self,
        targetEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        _cordl_type: crate::LiteNetLib::NatAddressType,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNatIntroductionSuccess", (targetEndPoint, _cordl_type, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "LiteNetLib+INatPunchListener")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::INatPunchListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
