#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::BufferUtils =>
    "Newtonsoft.Json.Utilities"."BufferUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::BufferUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::BufferUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl crate::Newtonsoft::Json::Utilities::BufferUtils {
    pub fn EnsureBufferSize(
        bufferPool: quest_hook::libil2cpp::Gc<char>,
        _cordl_size: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureBufferSize", (bufferPool, _cordl_size, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RentBuffer(
        bufferPool: quest_hook::libil2cpp::Gc<char>,
        minSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RentBuffer", (bufferPool, minSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnBuffer(
        bufferPool: quest_hook::libil2cpp::Gc<char>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReturnBuffer", (bufferPool, buffer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::BufferUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
