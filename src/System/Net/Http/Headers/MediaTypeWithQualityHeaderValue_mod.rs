#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
#[repr(C)]
#[derive(Debug)]
pub struct MediaTypeWithQualityHeaderValue {
    __cordl_parent: crate::System::Net::Http::Headers::MediaTypeHeaderValue,
}
#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http.Headers";
    const CLASS_NAME: &'static str = "MediaTypeWithQualityHeaderValue";
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
#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
impl std::ops::Deref
for crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue {
    type Target = crate::System::Net::Http::Headers::MediaTypeHeaderValue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
impl std::ops::DerefMut
for crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
impl crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minimalCount: i32,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue,
                                >,
                            >,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue
                    as quest_hook::libil2cpp::Type > ::class(), "TryParse", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (input, minimalCount, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseElement(
        lexer: quest_hook::libil2cpp::Gc<crate::System::Net::Http::Headers::Lexer>,
        parsedValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue,
            >,
        >,
        t: quest_hook::libil2cpp::ByRefMut<crate::System::Net::Http::Headers::Token>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::Http::Headers::Lexer>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Net::Http::Headers::Token,
                    >,
                ),
                bool,
                3usize,
            >("TryParseElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue
                    as quest_hook::libil2cpp::Type > ::class(), "TryParseElement", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (lexer, parsedValue, t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+MediaTypeWithQualityHeaderValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::MediaTypeWithQualityHeaderValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
