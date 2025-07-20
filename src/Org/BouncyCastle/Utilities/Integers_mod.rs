#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
#[repr(C)]
#[derive(Debug)]
pub struct Integers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Integers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities";
    const CLASS_NAME: &'static str = "Integers";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Integers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Integers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
impl crate::Org::BouncyCastle::Utilities::Integers {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NumberOfLeadingZeros(i: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("NumberOfLeadingZeros")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "NumberOfLeadingZeros",
                    1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (i))? };
        Ok(__cordl_ret.into())
    }
    pub fn NumberOfTrailingZeros(i: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("NumberOfTrailingZeros")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "NumberOfTrailingZeros",
                    1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (i))? };
        Ok(__cordl_ret.into())
    }
    pub fn RotateLeft_i32_0(
        i: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("RotateLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "RotateLeft", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (i, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RotateLeft_u32_1(
        i: u32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, i32), u32, 2usize>("RotateLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "RotateLeft", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (i, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RotateRight_i32_0(
        i: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("RotateRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "RotateRight", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (i, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RotateRight_u32_1(
        i: u32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, i32), u32, 2usize>("RotateRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), "RotateRight", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (i, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Utilities::Integers as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Utilities::Integers as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Integers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Integers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
