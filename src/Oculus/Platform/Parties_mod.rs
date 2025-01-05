#[cfg(feature = "Oculus+Platform+Parties")]
#[repr(C)]
#[derive(Debug)]
pub struct Parties {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Oculus+Platform+Parties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Parties => "Oculus.Platform"
    ."Parties"
);
#[cfg(feature = "Oculus+Platform+Parties")]
impl std::ops::Deref for crate::Oculus::Platform::Parties {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Parties")]
impl std::ops::DerefMut for crate::Oculus::Platform::Parties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Parties")]
impl crate::Oculus::Platform::Parties {
    pub fn GetCurrent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Party>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Party>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPartyUpdateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::PartyUpdateNotification,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPartyUpdateNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Parties")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Parties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
