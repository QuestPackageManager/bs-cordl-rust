#[cfg(feature = "AuthenticationToken")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AuthenticationToken {
    pub platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
    pub sessionToken: *mut crate::System::String,
}
#[cfg(feature = "AuthenticationToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for AuthenticationToken => ""."AuthenticationToken"
);
#[cfg(feature = "AuthenticationToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument for AuthenticationToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AuthenticationToken")]
impl AuthenticationToken {
    #[cfg(feature = "AuthenticationToken+Platform")]
    pub type Platform = crate::GlobalNamespace::AuthenticationToken_Platform;
    pub fn _ctor(
        &mut self,
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        sessionToken: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (platform, userId, userName, sessionToken),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<AuthenticationToken> {
        let __cordl_ret: AuthenticationToken = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AuthenticationToken+Platform")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationToken_Platform {
    Oculus = 2u8,
    OculusRift = 1u8,
    PS4 = 4u8,
    PS4Cert = 6u8,
    PS4Dev = 5u8,
    PS5 = 7u8,
    PS5Cert = 9u8,
    PS5Dev = 8u8,
    Steam = 3u8,
    Test = 0u8,
}
#[cfg(feature = "AuthenticationToken+Platform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AuthenticationToken_Platform =>
    ""."AuthenticationToken/Platform"
);
