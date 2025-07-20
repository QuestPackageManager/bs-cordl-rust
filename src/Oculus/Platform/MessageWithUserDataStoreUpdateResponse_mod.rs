#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithUserDataStoreUpdateResponse {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        >,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "MessageWithUserDataStoreUpdateResponse";
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
#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
impl std::ops::Deref
for crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse {
    type Target = crate::Oculus::Platform::Message_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
impl std::ops::DerefMut
for crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
impl crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse {
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
                1usize,
            >("GetDataFromMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as
                    quest_hook::libil2cpp::Type > ::class(), "GetDataFromMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        > = unsafe { method.invoke_unchecked(self, (c_message))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDataStoreUpdateResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
                0usize,
            >("GetUserDataStoreUpdateResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetUserDataStoreUpdateResponse", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::MessageWithUserDataStoreUpdateResponse as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c_message))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithUserDataStoreUpdateResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithUserDataStoreUpdateResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
