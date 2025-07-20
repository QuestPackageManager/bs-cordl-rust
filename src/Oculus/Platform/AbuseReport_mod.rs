#[cfg(feature = "Oculus+Platform+AbuseReport")]
#[repr(C)]
#[derive(Debug)]
pub struct AbuseReport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::AbuseReport {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "AbuseReport";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::AbuseReport as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::ReportRequestResponse),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("ReportRequestHandled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::AbuseReport as quest_hook::libil2cpp::Type
                    > ::class(), "ReportRequestHandled", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (response))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetReportButtonPressedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::AbuseReport as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetReportButtonPressedNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::AbuseReport as quest_hook::libil2cpp::Type
                    > ::class(), "SetReportButtonPressedNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))?
        };
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
