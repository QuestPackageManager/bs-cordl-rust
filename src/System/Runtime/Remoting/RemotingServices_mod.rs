#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingServices_CACD {
    __cordl_parent: crate::System::Object,
    pub d: *mut crate::System::Object,
    pub c: *mut crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::RemotingServices_CACD
    => "System.Runtime.Remoting"."RemotingServices/CACD"
);
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingServices_CACD {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingServices_CACD {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl crate::System::Runtime::Remoting::RemotingServices_CACD {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingServices_CACD {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingServices {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::RemotingServices =>
    "System.Runtime.Remoting"."RemotingServices"
);
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingServices {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl crate::System::Runtime::Remoting::RemotingServices {
    #[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
    pub type CACD = crate::System::Runtime::Remoting::RemotingServices_CACD;
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
