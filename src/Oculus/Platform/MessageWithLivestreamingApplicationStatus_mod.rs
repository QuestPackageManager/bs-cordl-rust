#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithLivestreamingApplicationStatus {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::MessageWithLivestreamingApplicationStatus => "Oculus.Platform"
    ."MessageWithLivestreamingApplicationStatus"
);
#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
impl std::ops::Deref
for crate::Oculus::Platform::MessageWithLivestreamingApplicationStatus {
    type Target = crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
impl std::ops::DerefMut
for crate::Oculus::Platform::MessageWithLivestreamingApplicationStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
impl crate::Oculus::Platform::MessageWithLivestreamingApplicationStatus {
    pub fn GetLivestreamingApplicationStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus = __cordl_object
            .invoke("GetLivestreamingApplicationStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus = __cordl_object
            .invoke("GetDataFromMessage", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithLivestreamingApplicationStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithLivestreamingApplicationStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
