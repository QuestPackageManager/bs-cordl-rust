#[cfg(feature = "System+TermInfoReader")]
#[repr(C)]
#[derive(Debug)]
pub struct TermInfoReader {
    __cordl_parent: crate::System::Object,
    pub boolSize: i32,
    pub numSize: i32,
    pub strOffsets: i32,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub booleansOffset: i32,
    pub intOffset: i32,
}
#[cfg(feature = "System+TermInfoReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TermInfoReader => "System"
    ."TermInfoReader"
);
#[cfg(feature = "System+TermInfoReader")]
impl std::ops::Deref for crate::System::TermInfoReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TermInfoReader")]
impl std::ops::DerefMut for crate::System::TermInfoReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TermInfoReader")]
impl crate::System::TermInfoReader {
    pub fn DetermineVersion(
        &mut self,
        magic: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DetermineVersion", (magic))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt16(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("GetInt16", (buffer, offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (buffer, offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringBytes_Il2CppArray_i32_1(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetStringBytes", (buffer, offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringBytes_TermInfoStrings0(
        &mut self,
        tstr: crate::System::TermInfoStrings,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetStringBytes", (tstr))?;
        Ok(__cordl_ret)
    }
    pub fn Get_TermInfoNumbers0(
        &mut self,
        number: crate::System::TermInfoNumbers,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Get", (number))?;
        Ok(__cordl_ret)
    }
    pub fn Get_TermInfoStrings1(
        &mut self,
        tstr: crate::System::TermInfoStrings,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Get", (tstr))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray1(
        term: *mut crate::System::String,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (term, buffer))?;
        Ok(__cordl_object)
    }
    pub fn New_String0(
        term: *mut crate::System::String,
        filename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (term, filename))?;
        Ok(__cordl_object)
    }
    pub fn ReadHeader(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadHeader", (buffer, position))?;
        Ok(__cordl_ret)
    }
    pub fn ReadNames(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadNames", (buffer, position))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        term: *mut crate::System::String,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (term, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        term: *mut crate::System::String,
        filename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (term, filename))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+TermInfoReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TermInfoReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
