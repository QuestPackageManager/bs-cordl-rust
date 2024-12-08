#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct UnexceptionalStreamWriter {
    __cordl_parent: crate::System::IO::StreamWriter,
}
#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::UnexceptionalStreamWriter =>
    "System.IO"."UnexceptionalStreamWriter"
);
#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
impl std::ops::Deref for crate::System::IO::UnexceptionalStreamWriter {
    type Target = crate::System::IO::StreamWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
impl std::ops::DerefMut for crate::System::IO::UnexceptionalStreamWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
impl crate::System::IO::UnexceptionalStreamWriter {
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
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn Write__cordl_char1(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Write_Il2CppArray2(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Write_String3(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
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
#[cfg(feature = "System+IO+UnexceptionalStreamWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::UnexceptionalStreamWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
