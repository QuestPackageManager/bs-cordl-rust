#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexCharClass {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _rangelist: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
        >,
    >,
    pub _categories: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _canonical: bool,
    pub _negate: bool,
    pub _subtractor: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexCharClass,
    >,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexCharClass
    => "System.Text.RegularExpressions"."RegexCharClass"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexCharClass {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCategory", (category))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCategoryFromName(
        &mut self,
        categoryName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        invert: bool,
        caseInsensitive: bool,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddCategoryFromName",
                (categoryName, invert, caseInsensitive, pattern),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn AddCharClass(
        &mut self,
        cc: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCharClass", (cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDigit(
        &mut self,
        ecma: bool,
        negate: bool,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDigit", (ecma, negate, pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLowercase(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLowercase", (culture))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLowercaseRange(
        &mut self,
        chMin: char,
        chMax: char,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLowercaseRange", (chMin, chMax, culture))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn AddSet(
        &mut self,
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSet", (set))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn AddSubtraction(
        &mut self,
        sub: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSubtraction", (sub))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Canonicalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Canonicalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CharInCategory(
        ch: char,
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        mySetLength: i32,
        myCategoryLength: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CharInCategory", (ch, set, start, mySetLength, myCategoryLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn CharInCategoryGroup(
        ch: char,
        chcategory: crate::System::Globalization::UnicodeCategory,
        category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CharInCategoryGroup", (ch, chcategory, category, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CharInClass(
        ch: char,
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CharInClass", (ch, set))?;
        Ok(__cordl_ret.into())
    }
    pub fn CharInClassInternal(
        ch: char,
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        mySetLength: i32,
        myCategoryLength: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CharInClassInternal",
                (ch, set, start, mySetLength, myCategoryLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CharInClassRecursive(
        ch: char,
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CharInClassRecursive", (ch, set, start))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IsECMAWordChar(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsECMAWordChar", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty(
        charClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEmpty", (charClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMergeable(
        charClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMergeable", (charClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNegated(
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNegated", (set))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSingleton(
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSingleton", (set))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSingletonInverse(
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSingletonInverse", (set))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSubtraction(
        charClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSubtraction", (charClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWordChar(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWordChar", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn NegateCategory(
        category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NegateCategory", (category))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_List_1_StringBuilder_RegexCharClass1(
        negate: bool,
        ranges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
            >,
        >,
        categories: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        subtraction: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (negate, ranges, categories, subtraction))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        charClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Parse", (charClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseRecursive(
        charClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseRecursive", (charClass, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn RangeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RangeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFromProperty(
        capname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        invert: bool,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFromProperty", (capname, invert, pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn SingletonChar(
        set: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SingletonChar", (set))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStringClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToStringClass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_List_1_StringBuilder_RegexCharClass1(
        &mut self,
        negate: bool,
        ranges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
            >,
        >,
        categories: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        subtraction: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCharClass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (negate, ranges, categories, subtraction))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanMerge(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanMerge", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRange")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexCharClass_SingleRangeComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    >,
> for crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCharClass+SingleRangeComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    >,
> for crate::System::Text::RegularExpressions::RegexCharClass_SingleRangeComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        crate::System::Text::RegularExpressions::RegexCharClass_SingleRange,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
