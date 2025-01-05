#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
#[repr(C)]
#[derive(Debug)]
pub struct ApplicationLifecycle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ApplicationLifecycle =>
    "Oculus.Platform"."ApplicationLifecycle"
);
#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
impl std::ops::Deref for crate::Oculus::Platform::ApplicationLifecycle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
impl std::ops::DerefMut for crate::Oculus::Platform::ApplicationLifecycle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
impl crate::Oculus::Platform::ApplicationLifecycle {
    pub fn GetLaunchDetails() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LaunchDetails>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchDetails,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLaunchDetails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LogDeeplinkResult(
        trackingID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: crate::Oculus::Platform::LaunchResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogDeeplinkResult", (trackingID, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLaunchIntentChangedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLaunchIntentChangedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+ApplicationLifecycle")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::ApplicationLifecycle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
