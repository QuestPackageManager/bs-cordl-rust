#[cfg(feature = "cordl_class_Oculus+Platform+Models+ChallengeEntryList")]
#[repr(C)]
#[derive(Debug)]
pub struct ChallengeEntryList {
    __cordl_parent: crate::Oculus::Platform::Models::DeserializableList_1<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeEntry>,
    >,
    pub TotalCount: u64,
}
#[cfg(feature = "cordl_class_Oculus+Platform+Models+ChallengeEntryList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Models::ChallengeEntryList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "ChallengeEntryList";
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
#[cfg(feature = "Oculus+Platform+Models+ChallengeEntryList")]
impl std::ops::Deref for crate::Oculus::Platform::Models::ChallengeEntryList {
    type Target = crate::Oculus::Platform::Models::DeserializableList_1<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeEntry>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeEntryList")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::ChallengeEntryList {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeEntryList")]
impl crate::Oculus::Platform::Models::ChallengeEntryList {
    pub fn New(
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (a))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+Models+ChallengeEntryList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::ChallengeEntryList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
