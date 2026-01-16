#[cfg(feature = "cordl_class_Oculus+Platform+RichPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct RichPresence {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+RichPresence")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::RichPresence {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "RichPresence";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::Deref for crate::Oculus::Platform::RichPresence {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::DerefMut for crate::Oculus::Platform::RichPresence {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl crate::Oculus::Platform::RichPresence {
    pub fn Clear(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                        0usize,
                    >("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDestinations() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Oculus::Platform::Request_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::DestinationList,
                            >,
                        >,
                    >, 0usize>("GetDestinations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDestinations",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextDestinationListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::DestinationList,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::DestinationList,
                                >,
                            >,
                        >,
                        1usize,
                    >("GetNextDestinationListPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextDestinationListPage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        richPresenceOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::RichPresenceOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::RichPresenceOptions,
                        >),
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                        1usize,
                    >("Set")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Set",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> =
            unsafe { cordl_method_info.invoke_unchecked((), (richPresenceOptions))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+RichPresence")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::RichPresence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
