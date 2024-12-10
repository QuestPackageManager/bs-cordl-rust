#[cfg(feature = "System+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct Number {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Number")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Number => "System"."Number"
);
#[cfg(feature = "System+Number")]
impl std::ops::Deref for crate::System::Number {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl std::ops::DerefMut for crate::System::Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl crate::System::Number {
    #[cfg(feature = "System+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Number_NumberBuffer;
}
#[cfg(feature = "System+Number")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::NumberBuffer_Number_DigitsAndNullTerminator => "System"
    ."Number/NumberBuffer/DigitsAndNullTerminator"
);
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
impl crate::System::NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Number_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub _sign: i32,
    pub _digits: crate::System::NumberBuffer_Number_DigitsAndNullTerminator,
    pub _allDigits: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Number+NumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Number_NumberBuffer => "System"
    ."Number/NumberBuffer"
);
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Number_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer")]
impl crate::System::Number_NumberBuffer {
    #[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
    pub type DigitsAndNullTerminator = crate::System::NumberBuffer_Number_DigitsAndNullTerminator;
    pub fn get_digits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_digits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sign(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sign",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sign(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sign",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
