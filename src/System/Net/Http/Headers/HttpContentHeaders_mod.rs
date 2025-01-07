#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpContentHeaders {
    __cordl_parent: crate::System::Net::Http::Headers::HttpHeaders,
    pub content: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpContent>,
}
#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Http::Headers::HttpContentHeaders {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http.Headers";
    const CLASS_NAME: &'static str = "HttpContentHeaders";
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
#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HttpContentHeaders {
    type Target = crate::System::Net::Http::Headers::HttpHeaders;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HttpContentHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
impl crate::System::Net::Http::Headers::HttpContentHeaders {
    pub fn New(
        content: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpContent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        content: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpContent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i64> = __cordl_object
            .invoke("get_ContentLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::MediaTypeHeaderValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::MediaTypeHeaderValue,
        > = __cordl_object.invoke("get_ContentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ContentType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::MediaTypeHeaderValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpContentHeaders")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HttpContentHeaders {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
