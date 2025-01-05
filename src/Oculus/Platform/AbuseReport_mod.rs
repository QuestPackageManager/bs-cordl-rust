#[cfg(feature = "Oculus+Platform+AbuseReport")]
#[repr(C)]
#[derive(Debug)]
pub struct AbuseReport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AbuseReport =>
    "Oculus.Platform"."AbuseReport"
);
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl std::ops::Deref for crate::Oculus::Platform::AbuseReport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl std::ops::DerefMut for crate::Oculus::Platform::AbuseReport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl crate::Oculus::Platform::AbuseReport {
    pub fn ReportRequestHandled(
        response: crate::Oculus::Platform::ReportRequestResponse,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReportRequestHandled", (response))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReportButtonPressedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetReportButtonPressedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::AbuseReport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
