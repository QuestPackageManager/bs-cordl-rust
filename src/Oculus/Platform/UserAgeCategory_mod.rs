#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
#[repr(C)]
#[derive(Debug)]
pub struct UserAgeCategory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::UserAgeCategory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "UserAgeCategory";
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
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl std::ops::Deref for crate::Oculus::Platform::UserAgeCategory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl std::ops::DerefMut for crate::Oculus::Platform::UserAgeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl crate::Oculus::Platform::UserAgeCategory {
    pub fn Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserAccountAgeCategory,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserAccountAgeCategory,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Report(
        age_category: crate::Oculus::Platform::AppAgeCategory,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Report", (age_category))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::UserAgeCategory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
