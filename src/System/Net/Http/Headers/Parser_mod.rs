#[cfg(feature = "System+Net+Http+Headers+Parser")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser =>
    "System.Net.Http.Headers"."Parser"
);
#[cfg(feature = "System+Net+Http+Headers+Parser")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser")]
impl crate::System::Net::Http::Headers::Parser {
    #[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
    pub type DateTime = crate::System::Net::Http::Headers::Parser_DateTime;
    #[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
    pub type EmailAddress = crate::System::Net::Http::Headers::Parser_EmailAddress;
    #[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
    pub type Host = crate::System::Net::Http::Headers::Parser_Host;
    #[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
    pub type Int = crate::System::Net::Http::Headers::Parser_Int;
    #[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
    pub type Long = crate::System::Net::Http::Headers::Parser_Long;
    #[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
    pub type MD5 = crate::System::Net::Http::Headers::Parser_MD5;
    #[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
    pub type TimeSpanSeconds = crate::System::Net::Http::Headers::Parser_TimeSpanSeconds;
    #[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
    pub type Token = crate::System::Net::Http::Headers::Parser_Token;
    #[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
    pub type Uri = crate::System::Net::Http::Headers::Parser_Uri;
}
#[cfg(feature = "System+Net+Http+Headers+Parser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::Headers::Parser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_DateTime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_DateTime =>
    "System.Net.Http.Headers"."Parser/DateTime"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_DateTime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
impl crate::System::Net::Http::Headers::Parser_DateTime {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+DateTime")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_DateTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_EmailAddress {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_EmailAddress
    => "System.Net.Http.Headers"."Parser/EmailAddress"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_EmailAddress {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_EmailAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
impl crate::System::Net::Http::Headers::Parser_EmailAddress {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+EmailAddress")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_EmailAddress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_Host {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_Host =>
    "System.Net.Http.Headers"."Parser/Host"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_Host {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_Host {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
impl crate::System::Net::Http::Headers::Parser_Host {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Host")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_Host {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_Int {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_Int =>
    "System.Net.Http.Headers"."Parser/Int"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_Int {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_Int {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
impl crate::System::Net::Http::Headers::Parser_Int {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Int")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_Int {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_Long {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_Long =>
    "System.Net.Http.Headers"."Parser/Long"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_Long {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_Long {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
impl crate::System::Net::Http::Headers::Parser_Long {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Long")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_Long {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_MD5 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_MD5 =>
    "System.Net.Http.Headers"."Parser/MD5"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_MD5 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_MD5 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
impl crate::System::Net::Http::Headers::Parser_MD5 {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+MD5")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_MD5 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_TimeSpanSeconds {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Http::Headers::Parser_TimeSpanSeconds => "System.Net.Http.Headers"
    ."Parser/TimeSpanSeconds"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_TimeSpanSeconds {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_TimeSpanSeconds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
impl crate::System::Net::Http::Headers::Parser_TimeSpanSeconds {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+TimeSpanSeconds")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_TimeSpanSeconds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_Token {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_Token =>
    "System.Net.Http.Headers"."Parser/Token"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_Token {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
impl crate::System::Net::Http::Headers::Parser_Token {
    pub fn Check(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Check", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCheck(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCheck", (s))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Token")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_Token {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser_Uri {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Parser_Uri =>
    "System.Net.Http.Headers"."Parser/Uri"
);
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Parser_Uri {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Parser_Uri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
impl crate::System::Net::Http::Headers::Parser_Uri {
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Parser+Uri")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::Parser_Uri {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
