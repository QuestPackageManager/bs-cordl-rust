#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
#[repr(C)]
#[derive(Debug)]
pub struct FormattableStringFactory_ConcreteFormattableString {
    __cordl_parent: crate::System::FormattableString,
    pub _format: *mut crate::System::String,
    pub _arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
}
#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString
    => "System.Runtime.CompilerServices"
    ."FormattableStringFactory/ConcreteFormattableString"
);
#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString {
    type Target = crate::System::FormattableString;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
impl crate::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString {
    pub fn _ctor(
        &mut self,
        format: *mut crate::System::String,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (format, arguments))?;
        Ok(__cordl_ret)
    }
    pub fn GetArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Format", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetArgument(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetArgument", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (formatProvider))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        format: *mut crate::System::String,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (format, arguments))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct FormattableStringFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::FormattableStringFactory =>
    "System.Runtime.CompilerServices"."FormattableStringFactory"
);
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::FormattableStringFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::FormattableStringFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
impl crate::System::Runtime::CompilerServices::FormattableStringFactory {
    #[cfg(
        feature = "System+Runtime+CompilerServices+FormattableStringFactory+ConcreteFormattableString"
    )]
    pub type ConcreteFormattableString = crate::System::Runtime::CompilerServices::FormattableStringFactory_ConcreteFormattableString;
}
#[cfg(feature = "System+Runtime+CompilerServices+FormattableStringFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::FormattableStringFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
