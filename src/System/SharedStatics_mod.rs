#[cfg(feature = "System+SharedStatics")]
#[repr(C)]
#[derive(Debug)]
pub struct SharedStatics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _maker: quest_hook::libil2cpp::Gc<
        crate::System::Security::Util::Tokenizer_StringMaker,
    >,
}
#[cfg(feature = "System+SharedStatics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::SharedStatics => "System"
    ."SharedStatics"
);
#[cfg(feature = "System+SharedStatics")]
impl std::ops::Deref for crate::System::SharedStatics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+SharedStatics")]
impl std::ops::DerefMut for crate::System::SharedStatics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+SharedStatics")]
impl crate::System::SharedStatics {
    pub fn GetSharedStringMaker() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Util::Tokenizer_StringMaker>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Util::Tokenizer_StringMaker,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSharedStringMaker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseSharedStringMaker(
        maker: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Security::Util::Tokenizer_StringMaker,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseSharedStringMaker", (maker))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+SharedStatics")]
impl quest_hook::libil2cpp::ObjectType for crate::System::SharedStatics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
