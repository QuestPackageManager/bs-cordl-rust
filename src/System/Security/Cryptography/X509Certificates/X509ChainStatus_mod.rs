#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatus")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct X509ChainStatus {
    pub status: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    pub info: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ChainStatus =>
    "System.Security.Cryptography.X509Certificates"."X509ChainStatus"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatus")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Security::Cryptography::X509Certificates::X509ChainStatus {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatus")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainStatus {
    pub fn GetInformation(
        flags: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInformation", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (flag),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Status",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Status(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Status",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StatusInformation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_StatusInformation",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
