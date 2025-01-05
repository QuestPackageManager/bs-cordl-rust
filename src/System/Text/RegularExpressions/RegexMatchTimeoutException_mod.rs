#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexMatchTimeoutException {
    __cordl_parent: crate::System::TimeoutException,
    pub _Input_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _Pattern_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _MatchTimeout_k__BackingField: crate::System::TimeSpan,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexMatchTimeoutException =>
    "System.Text.RegularExpressions"."RegexMatchTimeoutException"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl std::ops::Deref
for crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    type Target = crate::System::TimeoutException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl std::ops::DerefMut
for crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_TimeSpan0(
        regexInput: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        regexPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (regexInput, regexPattern, matchTimeout))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_TimeSpan0(
        &mut self,
        regexInput: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        regexPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (regexInput, regexPattern, matchTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Input", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MatchTimeout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_MatchTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Pattern", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexMatchTimeoutException")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Text::RegularExpressions::RegexMatchTimeoutException {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
