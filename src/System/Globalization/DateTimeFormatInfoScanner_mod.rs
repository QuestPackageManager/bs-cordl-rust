#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfoScanner {
    __cordl_parent: crate::System::Object,
    pub m_dateWords: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _ymdFlags: crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DateTimeFormatInfoScanner
    => "System.Globalization"."DateTimeFormatInfoScanner"
);
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl std::ops::Deref for crate::System::Globalization::DateTimeFormatInfoScanner {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl std::ops::DerefMut for crate::System::Globalization::DateTimeFormatInfoScanner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl crate::System::Globalization::DateTimeFormatInfoScanner {
    #[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
    pub type FoundDatePattern = crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern;
    pub fn AddDateWordOrPostfix(
        &mut self,
        formatPostfix: *mut crate::System::String,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDateWordOrPostfix", (formatPostfix, str))?;
        Ok(__cordl_ret)
    }
    pub fn AddDateWords(
        &mut self,
        pattern: *mut crate::System::String,
        index: i32,
        formatPostfix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("AddDateWords", (pattern, index, formatPostfix))?;
        Ok(__cordl_ret)
    }
    pub fn AddIgnorableSymbols(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddIgnorableSymbols", (text))?;
        Ok(__cordl_ret)
    }
    pub fn GetDateWordsOfDTFI(
        &mut self,
        dtfi: *mut crate::System::Globalization::DateTimeFormatInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDateWordsOfDTFI", (dtfi))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ScanDateWord(
        &mut self,
        pattern: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanDateWord", (pattern))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::DateTimeFormatInfoScanner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeFormatInfoScanner_FoundDatePattern {
    FoundDayPatternFlag = 4i32,
    FoundMonthPatternFlag = 2i32,
    FoundYMDPatternFlag = 7i32,
    FoundYearPatternFlag = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern =>
    "System.Globalization"."DateTimeFormatInfoScanner/FoundDatePattern"
);
