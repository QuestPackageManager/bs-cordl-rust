#[cfg(feature = "System+Version")]
#[repr(C)]
#[derive(Debug)]
pub struct Version {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Major: i32,
    pub _Minor: i32,
    pub _Build: i32,
    pub _Revision: i32,
}
#[cfg(feature = "System+Version")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Version => "System"."Version"
);
#[cfg(feature = "System+Version")]
impl std::ops::Deref for crate::System::Version {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Version")]
impl std::ops::DerefMut for crate::System::Version {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Version")]
impl crate::System::Version {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Version1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Version1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_4() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString3(
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Version5(
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_2(
        major: i32,
        minor: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (major, minor))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_1(
        major: i32,
        minor: i32,
        build: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (major, minor, build))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_i32_0(
        major: i32,
        minor: i32,
        build: i32,
        revision: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (major, minor, build, revision))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseVersion(
        input: crate::System::ReadOnlySpan_1<char>,
        throwOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseVersion", (input, throwOnFailure))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ISpanFormattable_TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.ISpanFormattable.TryFormat",
                (destination, charsWritten, format, provider),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCachedStringBuilder(
        &mut self,
        fieldCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = __cordl_object
            .invoke("ToCachedStringBuilder", (fieldCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_1(
        &mut self,
        fieldCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (fieldCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat_ByRefMut0(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryFormat", (destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat_i32_ByRefMut1(
        &mut self,
        destination: crate::System::Span_1<char>,
        fieldCount: i32,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryFormat", (destination, fieldCount, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseComponent(
        component: crate::System::ReadOnlySpan_1<char>,
        componentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwOnFailure: bool,
        parsedComponent: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseComponent",
                (component, componentName, throwOnFailure, parsedComponent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString3(
        &mut self,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Version5(
        &mut self,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_2(
        &mut self,
        major: i32,
        minor: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (major, minor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_1(
        &mut self,
        major: i32,
        minor: i32,
        build: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (major, minor, build))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_0(
        &mut self,
        major: i32,
        minor: i32,
        build: i32,
        revision: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (major, minor, build, revision))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Build(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultFormatFieldCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DefaultFormatFieldCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Major(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Major", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Minor(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Minor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Revision(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Revision", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        v1: quest_hook::libil2cpp::Gc<crate::System::Version>,
        v2: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (v1, v2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Version")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Version {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Version")]
impl AsRef<crate::System::ICloneable> for crate::System::Version {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsMut<crate::System::ICloneable> for crate::System::Version {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsRef<crate::System::IComparable> for crate::System::Version {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsMut<crate::System::IComparable> for crate::System::Version {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsRef<crate::System::IComparable_1<*mut crate::System::Version>>
for crate::System::Version {
    fn as_ref(&self) -> &crate::System::IComparable_1<*mut crate::System::Version> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsMut<crate::System::IComparable_1<*mut crate::System::Version>>
for crate::System::Version {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<*mut crate::System::Version> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsRef<crate::System::IEquatable_1<*mut crate::System::Version>>
for crate::System::Version {
    fn as_ref(&self) -> &crate::System::IEquatable_1<*mut crate::System::Version> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsMut<crate::System::IEquatable_1<*mut crate::System::Version>>
for crate::System::Version {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<*mut crate::System::Version> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::Version {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Version")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::Version {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        unsafe { std::mem::transmute(self) }
    }
}
