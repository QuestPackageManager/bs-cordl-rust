#[cfg(feature = "System+Globalization+EncodingTable")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+EncodingTable")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::EncodingTable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "EncodingTable";
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
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::Deref for crate::System::Globalization::EncodingTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::DerefMut for crate::System::Globalization::EncodingTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl crate::System::Globalization::EncodingTable {
    pub fn ENC(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cp: u16,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::InternalEncodingDataItem,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, u16),
                crate::System::Globalization::InternalEncodingDataItem,
                2usize,
            >("ENC")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ENC", 2usize
                )
            });
        let __cordl_ret: crate::System::Globalization::InternalEncodingDataItem = unsafe {
            method.invoke_unchecked((), (name, cp))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCodePageDataItem(
        codepage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CodePageDataItem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::System::Globalization::CodePageDataItem,
                >,
                1usize,
            >("GetCodePageDataItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCodePageDataItem", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CodePageDataItem,
        > = unsafe { method.invoke_unchecked((), (codepage)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCodePageFromName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetCodePageFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCodePageFromName", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNumEncodingItems() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("GetNumEncodingItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNumEncodingItems", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn MapCodePageDataItem(
        cp: u16,
        fcp: u16,
        names: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::InternalCodePageDataItem,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u16,
                    u16,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    u32,
                ),
                crate::System::Globalization::InternalCodePageDataItem,
                4usize,
            >("MapCodePageDataItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MapCodePageDataItem", 4usize
                )
            });
        let __cordl_ret: crate::System::Globalization::InternalCodePageDataItem = unsafe {
            method.invoke_unchecked((), (cp, fcp, names, flags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn internalGetCodePageFromName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("internalGetCodePageFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "internalGetCodePageFromName", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (name)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::EncodingTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
