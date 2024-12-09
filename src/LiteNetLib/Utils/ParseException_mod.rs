#[cfg(feature = "LiteNetLib+Utils+ParseException")]
#[repr(C)]
#[derive(Debug)]
pub struct ParseException {
    __cordl_parent: crate::System::Exception,
}
#[cfg(feature = "LiteNetLib+Utils+ParseException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::ParseException =>
    "LiteNetLib.Utils"."ParseException"
);
#[cfg(feature = "LiteNetLib+Utils+ParseException")]
impl std::ops::Deref for crate::LiteNetLib::Utils::ParseException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+ParseException")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::ParseException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+ParseException")]
impl crate::LiteNetLib::Utils::ParseException {
    pub fn New(
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+Utils+ParseException")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::ParseException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
