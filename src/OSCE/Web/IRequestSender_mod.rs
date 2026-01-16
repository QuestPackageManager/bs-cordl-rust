#[cfg(feature = "cordl_class_OSCE+Web+IRequestSender")]
#[repr(C)]
#[derive(Debug)]
pub struct IRequestSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OSCE+Web+IRequestSender")]
unsafe impl quest_hook::libil2cpp::Type for crate::OSCE::Web::IRequestSender {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OSCE.Web";
    const CLASS_NAME: &'static str = "IRequestSender";
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
#[cfg(feature = "OSCE+Web+IRequestSender")]
impl std::ops::Deref for crate::OSCE::Web::IRequestSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OSCE+Web+IRequestSender")]
impl std::ops::DerefMut for crate::OSCE::Web::IRequestSender {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OSCE+Web+IRequestSender")]
impl crate::OSCE::Web::IRequestSender {
    pub fn SendBatchRequest(
        &mut self,
        oculusGraphURL: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        endpoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        batch: quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::LoggerAnalyticsBatch>,
        onSucceed: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::LoggerAnalyticsBatch>,
            >,
        >,
        onFail: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
                crate::OSCE::Web::FailedPostRequest,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::OSCE::Analytics::LoggerAnalyticsBatch,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::OSCE::Analytics::LoggerAnalyticsBatch,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                                    crate::OSCE::Web::FailedPostRequest,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SendBatchRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendBatchRequest", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (oculusGraphURL, endpoint, batch, onSucceed, onFail),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendIdRequest(
        &mut self,
        oculusGraphURL: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        endpoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idRequest: crate::OSCE::Analytics::GetIdParams,
        onSucceed: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::LoggerAnalyticsBatch>,
            >,
        >,
        onFail: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
                crate::OSCE::Web::FailedPostRequest,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::OSCE::Analytics::GetIdParams,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::OSCE::Analytics::LoggerAnalyticsBatch,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                                    crate::OSCE::Web::FailedPostRequest,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SendIdRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendIdRequest", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (oculusGraphURL, endpoint, idRequest, onSucceed, onFail),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_OSCE+Web+IRequestSender")]
impl quest_hook::libil2cpp::ObjectType for crate::OSCE::Web::IRequestSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
