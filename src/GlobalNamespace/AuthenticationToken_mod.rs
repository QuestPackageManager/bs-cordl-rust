#[cfg(feature = "AuthenticationToken")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AuthenticationToken {
    pub platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub userId: *mut quest_hook::libil2cpp::Il2CppString,
    pub userName: *mut quest_hook::libil2cpp::Il2CppString,
    pub sessionToken: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "AuthenticationToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AuthenticationToken => ""
    ."AuthenticationToken"
);
#[cfg(feature = "AuthenticationToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AuthenticationToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AuthenticationToken")]
impl crate::GlobalNamespace::AuthenticationToken {
    #[cfg(feature = "AuthenticationToken+Platform")]
    pub type Platform = crate::GlobalNamespace::AuthenticationToken_Platform;
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::AuthenticationToken> {
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sessionToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (platform, userId, userName, sessionToken),
        )?;
        Ok(__cordl_ret.into())
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
