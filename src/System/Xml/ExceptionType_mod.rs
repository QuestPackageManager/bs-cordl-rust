#[cfg(feature = "System+Xml+ExceptionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    ArgumentException = 0i32,
    XmlException = 1i32,
}
#[cfg(feature = "System+Xml+ExceptionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ExceptionType => "System.Xml"
    ."ExceptionType"
);
