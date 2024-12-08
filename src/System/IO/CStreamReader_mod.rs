#[cfg(feature = "System+IO+CStreamReader")]
#[repr(C)]
#[derive(Debug)]
pub struct CStreamReader {
    __cordl_parent: crate::System::IO::StreamReader,
    pub driver: *mut crate::System::TermInfoDriver,
}
#[cfg(feature = "System+IO+CStreamReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::CStreamReader => "System.IO"
    ."CStreamReader"
);
#[cfg(feature = "System+IO+CStreamReader")]
impl std::ops::Deref for crate::System::IO::CStreamReader {
    type Target = crate::System::IO::StreamReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+CStreamReader")]
impl std::ops::DerefMut for crate::System::IO::CStreamReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+CStreamReader")]
impl crate::System::IO::CStreamReader {
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn ReadToEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadToEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn Peek(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Peek", ())?;
        Ok(__cordl_ret)
    }
    pub fn Read_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn Read_ByRefMut_i32_i32_1(
        &mut self,
        dest: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (dest, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn ReadLine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadLine", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+IO+CStreamReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::CStreamReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
