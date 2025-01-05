#[cfg(feature = "System+Net+HttpStatusDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpStatusDescription {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+HttpStatusDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpStatusDescription =>
    "System.Net"."HttpStatusDescription"
);
#[cfg(feature = "System+Net+HttpStatusDescription")]
impl std::ops::Deref for crate::System::Net::HttpStatusDescription {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpStatusDescription")]
impl std::ops::DerefMut for crate::System::Net::HttpStatusDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpStatusDescription")]
impl crate::System::Net::HttpStatusDescription {
    pub fn Get_HttpStatusCode0(
        code: crate::System::Net::HttpStatusCode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_i32_1(
        code: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (code))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpStatusDescription")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpStatusDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
