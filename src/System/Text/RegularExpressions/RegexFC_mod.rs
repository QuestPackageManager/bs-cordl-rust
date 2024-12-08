#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexFC {
    __cordl_parent: crate::System::Object,
    pub _cc: *mut crate::System::Text::RegularExpressions::RegexCharClass,
    pub _nullable: bool,
    pub _CaseInsensitive_k__BackingField: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexFC =>
    "System.Text.RegularExpressions"."RegexFC"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexFC {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexFC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
impl crate::System::Text::RegularExpressions::RegexFC {
    pub fn get_CaseInsensitive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CaseInsensitive", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CaseInsensitive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CaseInsensitive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetFirstChars(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetFirstChars", (culture))?;
        Ok(__cordl_ret)
    }
    pub fn AddFC(
        &mut self,
        fc: *mut crate::System::Text::RegularExpressions::RegexFC,
        concatenate: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddFC", (fc, concatenate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nullable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_char__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        ch: char,
        _cordl_not: bool,
        nullable: bool,
        caseInsensitive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ch, _cordl_not, nullable, caseInsensitive))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_bool__cordl_bool2(
        &mut self,
        charClass: *mut crate::System::String,
        nullable: bool,
        caseInsensitive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (charClass, nullable, caseInsensitive))?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool0(
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullable))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_char__cordl_bool__cordl_bool__cordl_bool1(
        ch: char,
        _cordl_not: bool,
        nullable: bool,
        caseInsensitive: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ch, _cordl_not, nullable, caseInsensitive))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_bool__cordl_bool2(
        charClass: *mut crate::System::String,
        nullable: bool,
        caseInsensitive: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (charClass, nullable, caseInsensitive))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFC")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexFC {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
