#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryPrimitives {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Buffers::Binary::BinaryPrimitives {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Buffers.Binary";
    const CLASS_NAME: &'static str = "BinaryPrimitives";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl std::ops::Deref for crate::System::Buffers::Binary::BinaryPrimitives {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl std::ops::DerefMut for crate::System::Buffers::Binary::BinaryPrimitives {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl crate::System::Buffers::Binary::BinaryPrimitives {
    pub fn ReverseEndianness_i32_0(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("ReverseEndianness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReverseEndianness", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), i64, 1usize>("ReverseEndianness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReverseEndianness", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u16_2(value: u16) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), u16, 1usize>("ReverseEndianness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReverseEndianness", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u32_3(value: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u32, 1usize>("ReverseEndianness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReverseEndianness", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u64_4(value: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ReverseEndianness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReverseEndianness", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::Binary::BinaryPrimitives {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
