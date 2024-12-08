#[cfg(feature = "System+Net+WebHeaderCollectionType")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebHeaderCollectionType {
    FileWebRequest = 9u16,
    FileWebResponse = 10u16,
    FtpWebRequest = 7u16,
    FtpWebResponse = 8u16,
    HttpListenerRequest = 5u16,
    HttpListenerResponse = 6u16,
    HttpWebRequest = 3u16,
    HttpWebResponse = 4u16,
    Unknown = 0u16,
    WebRequest = 1u16,
    WebResponse = 2u16,
}
#[cfg(feature = "System+Net+WebHeaderCollectionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebHeaderCollectionType =>
    "System.Net"."WebHeaderCollectionType"
);
