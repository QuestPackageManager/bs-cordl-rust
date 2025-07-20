#[cfg(feature = "System+Net+FtpMethodInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpMethodInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Operation: crate::System::Net::FtpOperation,
    pub Flags: crate::System::Net::FtpMethodFlags,
    pub HttpCommand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::FtpMethodInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "FtpMethodInfo";
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
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl std::ops::Deref for crate::System::Net::FtpMethodInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl std::ops::DerefMut for crate::System::Net::FtpMethodInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl crate::System::Net::FtpMethodInfo {
    pub fn GetMethodInfo(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::FtpMethodInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Net::FtpMethodInfo>,
                1usize,
            >("GetMethodInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "GetMethodInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::FtpMethodInfo> = unsafe {
            method.invoke_unchecked((), (method))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag(
        &mut self,
        flags: crate::System::Net::FtpMethodFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::Net::FtpMethodFlags), bool, 1usize>("HasFlag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "HasFlag", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operation: crate::System::Net::FtpOperation,
        flags: crate::System::Net::FtpMethodFlags,
        httpCommand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, operation, flags, httpCommand))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operation: crate::System::Net::FtpOperation,
        flags: crate::System::Net::FtpMethodFlags,
        httpCommand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Net::FtpOperation,
                    crate::System::Net::FtpMethodFlags,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (method, operation, flags, httpCommand))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCommandOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsCommandOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsCommandOnly", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDownload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsDownload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsDownload", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUpload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsUpload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsUpload", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldParseForResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ShouldParseForResponseUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::FtpMethodInfo as quest_hook::libil2cpp::Type >
                    ::class(), "get_ShouldParseForResponseUri", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpMethodInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
