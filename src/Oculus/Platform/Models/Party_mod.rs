#[cfg(feature = "Oculus+Platform+Models+Party")]
#[repr(C)]
#[derive(Debug)]
pub struct Party {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_ID: u64,
    pub InvitedUsersOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub InvitedUsers: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub LeaderOptional: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
    pub Leader: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
    pub UsersOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub Users: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
}
#[cfg(feature = "Oculus+Platform+Models+Party")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Models::Party {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "Party";
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
#[cfg(feature = "Oculus+Platform+Models+Party")]
impl std::ops::Deref for crate::Oculus::Platform::Models::Party {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Party")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::Party {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Party")]
impl crate::Oculus::Platform::Models::Party {
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+Party")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::Party {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
