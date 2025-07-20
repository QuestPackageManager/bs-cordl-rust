#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
#[repr(C)]
#[derive(Debug)]
pub struct Unsafe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::Unsafe {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "Unsafe";
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
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::Unsafe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::Unsafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl crate::System::Runtime::CompilerServices::Unsafe {
    pub fn AddByteOffset_IntPtr0<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        byteOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, crate::System::IntPtr),
                quest_hook::libil2cpp::ByRefMut<T>,
                2usize,
            >("AddByteOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AddByteOffset", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source, byteOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddByteOffset_u64_1<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        byteOffset: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, u64),
                quest_hook::libil2cpp::ByRefMut<T>,
                2usize,
            >("AddByteOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AddByteOffset", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source, byteOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut_IntPtr1<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, crate::System::IntPtr),
                quest_hook::libil2cpp::ByRefMut<T>,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source, elementOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut_i32_0<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, i32),
                quest_hook::libil2cpp::ByRefMut<T>,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source, elementOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppObject_i32_2<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (source, elementOffset))? };
        Ok(__cordl_ret.into())
    }
    pub fn AreSame<T>(
        left: quest_hook::libil2cpp::ByRefMut<T>,
        right: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, quest_hook::libil2cpp::ByRefMut<T>),
                bool,
                2usize,
            >("AreSame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AreSame", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsPointer<T>(
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("AsPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AsPointer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsRef_ByRefMut1<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>),
                quest_hook::libil2cpp::ByRefMut<T>,
                1usize,
            >("AsRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AsRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsRef_Il2CppObject0<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::ByRefMut<T>,
                1usize,
            >("AsRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "AsRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = unsafe {
            method.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn As_ByRefMut1<TFrom, TTo>(
        source: quest_hook::libil2cpp::ByRefMut<TFrom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<TTo>>
    where
        TFrom: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TTo: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<TFrom>),
                quest_hook::libil2cpp::ByRefMut<TTo>,
                1usize,
            >("As")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "As", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<TTo> = unsafe {
            method.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn As_Il2CppObject0<T>(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                T,
                1usize,
            >("As")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "As", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitBlockUnaligned(
        startAddress: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u8>, u8, u32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("InitBlockUnaligned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "InitBlockUnaligned", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (startAddress, value, byteCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAddressLessThan<T>(
        left: quest_hook::libil2cpp::ByRefMut<T>,
        right: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>, quest_hook::libil2cpp::ByRefMut<T>),
                bool,
                2usize,
            >("IsAddressLessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "IsAddressLessThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn Read<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                T,
                1usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnaligned<T>(
        source: quest_hook::libil2cpp::ByRefMut<u8>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u8>),
                T,
                1usize,
            >("ReadUnaligned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "ReadUnaligned", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("SizeOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "SizeOf", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnaligned<T>(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::Unsafe as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u8>, T),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteUnaligned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::Unsafe as
                    quest_hook::libil2cpp::Type > ::class(), "WriteUnaligned", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (destination, value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::Unsafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
