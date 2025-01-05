#[cfg(feature = "ENet+ArrayPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayPool {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ENet+ArrayPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::ArrayPool => "ENet"."ArrayPool"
);
#[cfg(feature = "ENet+ArrayPool")]
impl std::ops::Deref for crate::ENet::ArrayPool {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+ArrayPool")]
impl std::ops::DerefMut for crate::ENet::ArrayPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+ArrayPool")]
impl crate::ENet::ArrayPool {
    pub fn GetByteBuffer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByteBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPointerBuffer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPointerBuffer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+ArrayPool")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::ArrayPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
