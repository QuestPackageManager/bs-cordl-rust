#[cfg(feature = "System+Net+KnownHttpVerb")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownHttpVerb {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        requireContentBody: bool,
        contentBodyNotAllowed: bool,
        connectRequest: bool,
        expectNoContentResponse: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::KnownHttpVerb>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::KnownHttpVerb> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
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
