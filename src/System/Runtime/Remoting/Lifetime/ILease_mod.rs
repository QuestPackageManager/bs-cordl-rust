#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
#[repr(C)]
#[derive(Debug)]
pub struct ILease {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Lifetime::ILease =>
    "System.Runtime.Remoting.Lifetime"."ILease"
);
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Lifetime::ILease {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Lifetime::ILease {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
impl crate::System::Runtime::Remoting::Lifetime::ILease {
    pub fn get_RenewOnCallTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_RenewOnCallTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn Renew(
        &mut self,
        renewalTime: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("Renew", (renewalTime))?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::Lifetime::LeaseState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Remoting::Lifetime::LeaseState = __cordl_object
            .invoke("get_CurrentState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentLeaseTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_CurrentLeaseTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+ILease")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::ILease {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
