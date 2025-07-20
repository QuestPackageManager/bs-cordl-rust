#[cfg(feature = "IBitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct IBitMaskUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBitMaskUtil")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IBitMaskUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBitMaskUtil";
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
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::Deref for crate::GlobalNamespace::IBitMaskUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl crate::GlobalNamespace::IBitMaskUtil {
    pub fn FromBytes<T>(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>, i32),
                T,
                2usize,
            >("FromBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "FromBytes", 2usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (bytes, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBase64Char(digit: u64) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), char, 1usize>("GetBase64Char")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "GetBase64Char", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (digit))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBase64Digit(c: char) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u32, 1usize>("GetBase64Digit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "GetBase64Digit", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHexDigit(c: char) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u32, 1usize>("GetHexDigit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "GetHexDigit", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn NumberOfSetBits<T>(bitMask: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(T), i32, 1usize>("NumberOfSetBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "NumberOfSetBits", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (bitMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToBytes<T>(
        bitMask: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (T),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("ToBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "ToBytes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (bitMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToShortString<T>(
        bitMask: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (T),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToShortString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "ToShortString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (bitMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut0<T>(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bitMask: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<T>,
                ),
                bool,
                2usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "TryParse", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (stringSerializedMask, bitMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_i32_i32_ByRefMut1<T>(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        offset: i32,
        length: i32,
        bitMask: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<T>,
                ),
                bool,
                4usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBitMaskUtil as quest_hook::libil2cpp::Type
                    > ::class(), "TryParse", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (stringSerializedMask, offset, length, bitMask))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
