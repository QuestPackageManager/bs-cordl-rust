#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
#[repr(C)]
#[derive(Debug)]
pub struct LifetimeServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Lifetime";
    const CLASS_NAME: &'static str = "LifetimeServices";
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
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    pub fn TrackLifetime(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ServerIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrackLifetime", (identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LeaseManagerPollTime() -> quest_hook::libil2cpp::Result<
        crate::System::TimeSpan,
    > {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LeaseManagerPollTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LeaseTime() -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LeaseTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RenewOnCallTime() -> quest_hook::libil2cpp::Result<
        crate::System::TimeSpan,
    > {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RenewOnCallTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SponsorshipTimeout() -> quest_hook::libil2cpp::Result<
        crate::System::TimeSpan,
    > {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SponsorshipTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LeaseManagerPollTime(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LeaseManagerPollTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LeaseTime(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LeaseTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RenewOnCallTime(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_RenewOnCallTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SponsorshipTimeout(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_SponsorshipTimeout", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
