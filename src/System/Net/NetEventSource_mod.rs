#[cfg(feature = "System+Net+NetEventSource")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEventSource {
    __cordl_parent: crate::System::Diagnostics::Tracing::EventSource,
}
#[cfg(feature = "System+Net+NetEventSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetEventSource => "System.Net"
    ."NetEventSource"
);
#[cfg(feature = "System+Net+NetEventSource")]
impl std::ops::Deref for crate::System::Net::NetEventSource {
    type Target = crate::System::Diagnostics::Tracing::EventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource")]
impl std::ops::DerefMut for crate::System::Net::NetEventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource")]
impl crate::System::Net::NetEventSource {
    #[cfg(feature = "System+Net+NetEventSource+Keywords")]
    pub type Keywords = crate::System::Net::NetEventSource_Keywords;
    pub fn Associate_Il2CppObject_Il2CppObject0(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Associate", (first, second, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Associate_Il2CppString_Il2CppString_Il2CppString1(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Associate", (thisOrContextObject, memberName, first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn CriticalFailure(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CriticalFailure", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enter_Il2CppObject_FormattableString_Il2CppString0(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        formattableString: quest_hook::libil2cpp::Gc<crate::System::FormattableString>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enter", (thisOrContextObject, formattableString, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enter_Il2CppObject_Il2CppObject_Il2CppObject_Il2CppObject_Il2CppString2(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enter", (thisOrContextObject, arg0, arg1, arg2, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enter_Il2CppObject_Il2CppObject_Il2CppString1(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enter", (thisOrContextObject, arg0, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enter_Il2CppString_Il2CppString_Il2CppString3(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enter", (thisOrContextObject, memberName, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Error(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Error", (thisOrContextObject, message, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ErrorMessage(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ErrorMessage", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exit_Il2CppObject_FormattableString0(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        formattableString: quest_hook::libil2cpp::Gc<crate::System::FormattableString>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exit", (thisOrContextObject, formattableString, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exit_Il2CppObject_Il2CppObject1(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exit", (thisOrContextObject, arg0, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exit_Il2CppString_Il2CppString2(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Exit", (thisOrContextObject, memberName, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fail(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fail", (thisOrContextObject, message, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_FormattableString1(
        s: quest_hook::libil2cpp::Gc<crate::System::FormattableString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Format", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Format", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IdOf(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("IdOf", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Info_Il2CppObject_FormattableString0(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        formattableString: quest_hook::libil2cpp::Gc<crate::System::FormattableString>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Info", (thisOrContextObject, formattableString, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Info_Il2CppObject_Il2CppObject1(
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Info", (thisOrContextObject, message, memberName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Info_Il2CppString_Il2CppString2(
        &mut self,
        thisOrContextObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Info", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn WriteEvent(
        &mut self,
        eventId: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg4: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsEnabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetEventSource")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetEventSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEventSource_Keywords {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetEventSource_Keywords =>
    "System.Net"."NetEventSource/Keywords"
);
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl std::ops::Deref for crate::System::Net::NetEventSource_Keywords {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl std::ops::DerefMut for crate::System::Net::NetEventSource_Keywords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl crate::System::Net::NetEventSource_Keywords {}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetEventSource_Keywords {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
