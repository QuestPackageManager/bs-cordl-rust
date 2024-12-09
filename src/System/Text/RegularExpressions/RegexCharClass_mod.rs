#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexCharClass {
    __cordl_parent: crate::System::Object,
    pub _rangelist: *mut crate::System::Collections::Generic::List_1<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    >,
    pub _categories: *mut crate::System::Text::StringBuilder,
    pub _canonical: bool,
    pub _negate: bool,
    pub _subtractor: *mut crate::System::Text::RegularExpressions::RegexCharClass,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexCharClass
    => "System.Text.RegularExpressions"."RegexCharClass"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexCharClass {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexCharClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
impl crate::System::Text::RegularExpressions::RegexCharClass {
    #[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+LowerCaseMapping")]
    pub type LowerCaseMapping = crate::System::Text::RegularExpressions::RegexCharClass_LowerCaseMapping;
    #[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
    pub type SingleRange = crate::System::Text::RegularExpressions::RegexCharClass_SingleRange;
    #[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
    pub type SingleRangeComparer = crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer;
    pub fn AddCategory(
        &mut self,
        category: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCategory", (category))?;
        Ok(__cordl_ret)
    }
    pub fn AddCategoryFromName(
        &mut self,
        categoryName: *mut crate::System::String,
        invert: bool,
        caseInsensitive: bool,
        pattern: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddCategoryFromName",
                (categoryName, invert, caseInsensitive, pattern),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddChar(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChar", (c))?;
        Ok(__cordl_ret)
    }
    pub fn AddCharClass(
        &mut self,
        cc: *mut crate::System::Text::RegularExpressions::RegexCharClass,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCharClass", (cc))?;
        Ok(__cordl_ret)
    }
    pub fn AddDigit(
        &mut self,
        ecma: bool,
        negate: bool,
        pattern: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDigit", (ecma, negate, pattern))?;
        Ok(__cordl_ret)
    }
    pub fn AddLowercase(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLowercase", (culture))?;
        Ok(__cordl_ret)
    }
    pub fn AddLowercaseRange(
        &mut self,
        chMin: char,
        chMax: char,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLowercaseRange", (chMin, chMax, culture))?;
        Ok(__cordl_ret)
    }
    pub fn AddRange(
        &mut self,
        first: char,
        last: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRange", (first, last))?;
        Ok(__cordl_ret)
    }
    pub fn AddSet(
        &mut self,
        set: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSet", (set))?;
        Ok(__cordl_ret)
    }
    pub fn AddSpace(
        &mut self,
        ecma: bool,
        negate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpace", (ecma, negate))?;
        Ok(__cordl_ret)
    }
    pub fn AddSubtraction(
        &mut self,
        sub: *mut crate::System::Text::RegularExpressions::RegexCharClass,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSubtraction", (sub))?;
        Ok(__cordl_ret)
    }
    pub fn AddWord(
        &mut self,
        ecma: bool,
        negate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWord", (ecma, negate))?;
        Ok(__cordl_ret)
    }
    pub fn Canonicalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Canonicalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRangeAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Text::RegularExpressions::RegexCharClass_SingleRange = __cordl_object
            .invoke("GetRangeAt", (i))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_List_1_StringBuilder_RegexCharClass1(
        negate: bool,
        ranges: *mut crate::System::Collections::Generic::List_1<
            crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
        >,
        categories: *mut crate::System::Text::StringBuilder,
        subtraction: *mut crate::System::Text::RegularExpressions::RegexCharClass,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (negate, ranges, categories, subtraction))?;
        Ok(__cordl_object)
    }
    pub fn RangeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RangeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToStringClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToStringClass", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_List_1_StringBuilder_RegexCharClass1(
        &mut self,
        negate: bool,
        ranges: *mut crate::System::Collections::Generic::List_1<
            crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
        >,
        categories: *mut crate::System::Text::StringBuilder,
        subtraction: *mut crate::System::Text::RegularExpressions::RegexCharClass,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (negate, ranges, categories, subtraction))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanMerge(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanMerge", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Negate(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Negate", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexCharClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+LowerCaseMapping")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RegexCharClass_LowerCaseMapping {
    pub ChMin: char,
    pub ChMax: char,
    pub LcOp: i32,
    pub Data: i32,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+LowerCaseMapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexCharClass_LowerCaseMapping =>
    "System.Text.RegularExpressions"."RegexCharClass/LowerCaseMapping"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+LowerCaseMapping")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::RegexCharClass_LowerCaseMapping {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+LowerCaseMapping")]
impl crate::System::Text::RegularExpressions::RegexCharClass_LowerCaseMapping {
    pub fn _ctor(
        &mut self,
        chMin: char,
        chMax: char,
        lcOp: i32,
        data: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (chMin, chMax, lcOp, data),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RegexCharClass_SingleRange {
    pub First: char,
    pub Last: char,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexCharClass_SingleRange =>
    "System.Text.RegularExpressions"."RegexCharClass/SingleRange"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::RegexCharClass_SingleRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
impl crate::System::Text::RegularExpressions::RegexCharClass_SingleRange {
    pub fn _ctor(
        &mut self,
        first: char,
        last: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (first, last),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexCharClass_SingleRangeComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer =>
    "System.Text.RegularExpressions"."RegexCharClass/SingleRangeComparer"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl std::ops::Deref
for crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl std::ops::DerefMut
for crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    pub fn Compare(
        &mut self,
        x: crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
        y: crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
