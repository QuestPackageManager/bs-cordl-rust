#[cfg(feature = "System+MemoryExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+MemoryExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::MemoryExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "MemoryExtensions";
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
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::Deref for crate::System::MemoryExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::DerefMut for crate::System::MemoryExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl crate::System::MemoryExtensions {
    pub fn AsSpan_Il2CppArray_i32_0<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>, i32),
                crate::System::Span_1<T>,
                2usize,
            >("AsSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsSpan", 2usize
                )
            });
        let __cordl_ret: crate::System::Span_1<T> = unsafe {
            method.invoke_unchecked((), (array, start))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_Il2CppArray_i32_i32_4<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                    i32,
                    i32,
                ),
                crate::System::Span_1<T>,
                3usize,
            >("AsSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsSpan", 3usize
                )
            });
        let __cordl_ret: crate::System::Span_1<T> = unsafe {
            method.invoke_unchecked((), (array, start, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("AsSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsSpan", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (text))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_Il2CppString_i32_2(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                crate::System::ReadOnlySpan_1<char>,
                2usize,
            >("AsSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsSpan", 2usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (text, start))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_Il2CppString_i32_i32_3(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                ),
                crate::System::ReadOnlySpan_1<char>,
                3usize,
            >("AsSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsSpan", 3usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (text, start, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        source: crate::System::ReadOnlySpan_1<char>,
        value: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>, char),
                bool,
                2usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (source, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        destination: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                    crate::System::Span_1<T>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (source, destination))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_ReadOnlySpan_1_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<T>, crate::System::ReadOnlySpan_1<T>),
                bool,
                2usize,
            >("EndsWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndsWith", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_StringComparison0(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        comparisonType: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::StringComparison,
                ),
                bool,
                3usize,
            >("EndsWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndsWith", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (span, value, comparisonType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EqualsOrdinal(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                ),
                bool,
                2usize,
            >("EqualsOrdinal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EqualsOrdinal", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn EqualsOrdinalIgnoreCase(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                ),
                bool,
                2usize,
            >("EqualsOrdinalIgnoreCase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EqualsOrdinalIgnoreCase", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<T>, T),
                i32,
                2usize,
            >("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOf", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (span, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        values: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<T>, crate::System::ReadOnlySpan_1<T>),
                i32,
                2usize,
            >("IndexOfAny")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOfAny", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (span, values)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeComparableAsBytes<T>(
        _cordl_size: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u64>),
                bool,
                1usize,
            >("IsTypeComparableAsBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsTypeComparableAsBytes", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_size)) };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        other: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<T>, crate::System::ReadOnlySpan_1<T>),
                bool,
                2usize,
            >("SequenceEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SequenceEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, other)) };
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<T>, crate::System::ReadOnlySpan_1<T>),
                bool,
                2usize,
            >("StartsWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartsWith", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperInvariant(
        source: crate::System::ReadOnlySpan_1<char>,
        destination: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>, crate::System::Span_1<char>),
                i32,
                2usize,
            >("ToUpperInvariant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUpperInvariant", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (source, destination))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("Trim")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Trim", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (span))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("TrimEnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrimEnd", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (span))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                crate::System::ReadOnlySpan_1<char>,
                1usize,
            >("TrimStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrimStart", 1usize
                )
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = unsafe {
            method.invoke_unchecked((), (span))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MemoryExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
