#[cfg(feature = "System+IPv6AddressHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IPv6AddressHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IPv6AddressHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IPv6AddressHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "IPv6AddressHelper";
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
#[cfg(feature = "System+IPv6AddressHelper")]
impl std::ops::Deref for crate::System::IPv6AddressHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl std::ops::DerefMut for crate::System::IPv6AddressHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl crate::System::IPv6AddressHelper {
    pub fn FindCompressionRange(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, i32>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<u16>),
                crate::System::ValueTuple_2<i32, i32>,
                1usize,
            >("FindCompressionRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "FindCompressionRange", 1usize
                )
            });
        let __cordl_ret: crate::System::ValueTuple_2<i32, i32> = unsafe {
            method.invoke_unchecked((), (numbers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        validateStrictAddress: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    bool,
                ),
                bool,
                4usize,
            >("InternalIsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "InternalIsValid", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, start, end, validateStrictAddress))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLoopback(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<u16>),
                bool,
                1usize,
            >("IsLoopback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "IsLoopback", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (numbers))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "IsValid", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, start, end))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidStrict(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("IsValidStrict")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "IsValidStrict", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, start, end))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        address: crate::System::ReadOnlySpan_1<char>,
        numbers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        scopeId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "Parse", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (address, numbers, start, scopeId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseCanonicalName(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        isLoopback: quest_hook::libil2cpp::ByRefMut<bool>,
        scopeId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("ParseCanonicalName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "ParseCanonicalName", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (str, start, isLoopback, scopeId))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldHaveIpv4Embedded(
        numbers: crate::System::ReadOnlySpan_1<u16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::IPv6AddressHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<u16>),
                bool,
                1usize,
            >("ShouldHaveIpv4Embedded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::IPv6AddressHelper as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldHaveIpv4Embedded", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (numbers))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IPv6AddressHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IPv6AddressHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
