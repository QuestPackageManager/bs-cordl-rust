#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
#[repr(C)]
#[derive(Debug)]
pub struct QuotedStringFormatReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::QuotedStringFormatReader =>
    "System.Net.Mail"."QuotedStringFormatReader"
);
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::Deref for crate::System::Net::Mail::QuotedStringFormatReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::QuotedStringFormatReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl crate::System::Net::Mail::QuotedStringFormatReader {
    pub fn IsValidQtext(
        allowUnicode: bool,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidQtext", (allowUnicode, ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadReverseQuoted(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        permitUnicode: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadReverseQuoted", (data, index, permitUnicode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadReverseUnQuoted(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        permitUnicode: bool,
        expectCommaDelimiter: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReadReverseUnQuoted",
                (data, index, permitUnicode, expectCommaDelimiter),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Mail::QuotedStringFormatReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
