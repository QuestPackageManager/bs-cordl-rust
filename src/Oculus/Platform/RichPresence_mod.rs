#[cfg(feature = "Oculus+Platform+RichPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct RichPresence {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::RichPresence =>
    "Oculus.Platform"."RichPresence"
);
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::Deref for crate::Oculus::Platform::RichPresence {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::DerefMut for crate::Oculus::Platform::RichPresence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl crate::Oculus::Platform::RichPresence {
    pub fn Clear() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDestinations() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::DestinationList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::DestinationList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDestinations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextDestinationListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::DestinationList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::DestinationList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextDestinationListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        richPresenceOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::RichPresenceOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Set", (richPresenceOptions))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::RichPresence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
