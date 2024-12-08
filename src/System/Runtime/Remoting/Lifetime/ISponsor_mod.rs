#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
#[repr(C)]
#[derive(Debug)]
pub struct ISponsor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Lifetime::ISponsor =>
    "System.Runtime.Remoting.Lifetime"."ISponsor"
);
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Lifetime::ISponsor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Lifetime::ISponsor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
impl crate::System::Runtime::Remoting::Lifetime::ISponsor {
    pub fn Renewal(
        &mut self,
        lease: *mut crate::System::Runtime::Remoting::Lifetime::ILease,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("Renewal", (lease))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ISponsor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::ISponsor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
