#[cfg(feature = "System+Net+Mail+MailAddressParser")]
#[repr(C)]
#[derive(Debug)]
pub struct MailAddressParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Mail::MailAddressParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Mail";
    const CLASS_NAME: &'static str = "MailAddressParser";
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
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::Deref for crate::System::Net::Mail::MailAddressParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::DerefMut for crate::System::Net::Mail::MailAddressParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl crate::System::Net::Mail::MailAddressParser {
    pub fn NormalizeOrThrow(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("NormalizeOrThrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "NormalizeOrThrow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseAddress_Il2CppString0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
                1usize,
            >("ParseAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "ParseAddress", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Mail::MailAddress,
        > = unsafe { method.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseAddress__cordl_bool_ByRefMut1(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectMultipleAddresses: bool,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
                3usize,
            >("ParseAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "ParseAddress", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Mail::MailAddress,
        > = unsafe {
            method.invoke_unchecked((), (data, expectMultipleAddresses, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDisplayName(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        expectMultipleAddresses: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("ParseDisplayName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "ParseDisplayName", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked((), (data, index, expectMultipleAddresses))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDomain(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ParseDomain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "ParseDomain", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (data, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseLocalPart(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        expectAngleBracket: bool,
        expectMultipleAddresses: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("ParseLocalPart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(), "ParseLocalPart", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (data, index, expectAngleBracket, expectMultipleAddresses),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCfwsAndThrowIfIncomplete(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Mail::MailAddressParser as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                i32,
                2usize,
            >("ReadCfwsAndThrowIfIncomplete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Mail::MailAddressParser as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ReadCfwsAndThrowIfIncomplete", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (data, index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::MailAddressParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
