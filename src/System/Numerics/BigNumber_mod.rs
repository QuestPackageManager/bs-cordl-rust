#[cfg(feature = "System+Numerics+BigNumber")]
#[repr(C)]
#[derive(Debug)]
pub struct BigNumber {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Numerics+BigNumber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::BigNumber => "System.Numerics"
    ."BigNumber"
);
#[cfg(feature = "System+Numerics+BigNumber")]
impl std::ops::Deref for crate::System::Numerics::BigNumber {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl std::ops::DerefMut for crate::System::Numerics::BigNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl crate::System::Numerics::BigNumber {
    #[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
    pub type BigNumberBuffer = crate::System::Numerics::BigNumber_BigNumberBuffer;
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::BigNumber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BigNumber_BigNumberBuffer {
    pub digits: *mut crate::System::Text::StringBuilder,
    pub precision: i32,
    pub scale: i32,
    pub sign: bool,
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::BigNumber_BigNumberBuffer =>
    "System.Numerics"."BigNumber/BigNumberBuffer"
);
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Numerics::BigNumber_BigNumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
impl crate::System::Numerics::BigNumber_BigNumberBuffer {}
