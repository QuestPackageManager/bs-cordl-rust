#[cfg(feature = "System+Globalization+FormatProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Globalization+FormatProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::FormatProvider =>
    "System.Globalization"."FormatProvider"
);
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl crate::System::Globalization::FormatProvider {
    #[cfg(feature = "System+Globalization+FormatProvider+Number")]
    pub type Number = crate::System::Globalization::FormatProvider_Number;
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::FormatProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider_Number {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::FormatProvider_Number =>
    "System.Globalization"."FormatProvider/Number"
);
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider_Number {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider_Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl crate::System::Globalization::FormatProvider_Number {
    #[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Globalization::Number_FormatProvider_NumberBuffer;
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::FormatProvider_Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Number_FormatProvider_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub sign: bool,
    pub overrideDigits: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::Number_FormatProvider_NumberBuffer => "System.Globalization"
    ."FormatProvider/Number/NumberBuffer"
);
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
impl crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    pub fn get_digits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_digits",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
