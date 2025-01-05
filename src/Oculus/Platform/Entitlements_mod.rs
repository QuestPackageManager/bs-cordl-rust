#[cfg(feature = "Oculus+Platform+Entitlements")]
#[repr(C)]
#[derive(Debug)]
pub struct Entitlements {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Entitlements =>
    "Oculus.Platform"."Entitlements"
);
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::Deref for crate::Oculus::Platform::Entitlements {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::DerefMut for crate::Oculus::Platform::Entitlements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl crate::Oculus::Platform::Entitlements {
    pub fn IsUserEntitledToApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUserEntitledToApplication", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Entitlements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
