#[cfg(feature = "System+Net+FtpLoginState")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FtpLoginState {
    #[default]
    LoggedIn = 1u8,
    LoggedInButNeedsRelogin = 2u8,
    NotLoggedIn = 0u8,
    ReloginFailed = 3u8,
}
#[cfg(feature = "System+Net+FtpLoginState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpLoginState => "System.Net"
    ."FtpLoginState"
);
