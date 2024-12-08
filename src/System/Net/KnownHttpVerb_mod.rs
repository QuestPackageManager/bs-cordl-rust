#[cfg(feature = "System+Net+KnownHttpVerb")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownHttpVerb {
    __cordl_parent: crate::System::Object,
    pub Name: *mut crate::System::String,
    pub RequireContentBody: bool,
    pub ContentBodyNotAllowed: bool,
    pub ConnectRequest: bool,
    pub ExpectNoContentResponse: bool,
}
#[cfg(feature = "System+Net+KnownHttpVerb")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::KnownHttpVerb => "System.Net"
    ."KnownHttpVerb"
);
#[cfg(feature = "System+Net+KnownHttpVerb")]
impl std::ops::Deref for crate::System::Net::KnownHttpVerb {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+KnownHttpVerb")]
impl std::ops::DerefMut for crate::System::Net::KnownHttpVerb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+KnownHttpVerb")]
impl crate::System::Net::KnownHttpVerb {
    pub fn New(
        name: *mut crate::System::String,
        requireContentBody: bool,
        contentBodyNotAllowed: bool,
        connectRequest: bool,
        expectNoContentResponse: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    name,
                    requireContentBody,
                    contentBodyNotAllowed,
                    connectRequest,
                    expectNoContentResponse,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        requireContentBody: bool,
        contentBodyNotAllowed: bool,
        connectRequest: bool,
        expectNoContentResponse: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    name,
                    requireContentBody,
                    contentBodyNotAllowed,
                    connectRequest,
                    expectNoContentResponse,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+KnownHttpVerb")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::KnownHttpVerb {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
