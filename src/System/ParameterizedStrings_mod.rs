#[cfg(feature = "System+ParameterizedStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct ParameterizedStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ParameterizedStrings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ParameterizedStrings => "System"
    ."ParameterizedStrings"
);
#[cfg(feature = "System+ParameterizedStrings")]
impl std::ops::Deref for crate::System::ParameterizedStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings")]
impl std::ops::DerefMut for crate::System::ParameterizedStrings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings")]
impl crate::System::ParameterizedStrings {
    #[cfg(feature = "System+ParameterizedStrings+FormatParam")]
    pub type FormatParam = crate::System::ParameterizedStrings_FormatParam;
    #[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
    pub type LowLevelStack = crate::System::ParameterizedStrings_LowLevelStack;
}
#[cfg(feature = "System+ParameterizedStrings")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ParameterizedStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ParameterizedStrings_FormatParam {
    pub _int32: i32,
    pub _string: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParameterizedStrings_FormatParam =>
    "System"."ParameterizedStrings/FormatParam"
);
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ParameterizedStrings_FormatParam {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
impl crate::System::ParameterizedStrings_FormatParam {
    pub fn _ctor_Il2CppString1(
        &mut self,
        intValue: i32,
        stringValue: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (intValue, stringValue),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Int32(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Int32",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Object",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_String(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_String",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
#[repr(C)]
#[derive(Debug)]
pub struct ParameterizedStrings_LowLevelStack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _arr: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::ParameterizedStrings_FormatParam,
    >,
    pub _count: i32,
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ParameterizedStrings_LowLevelStack =>
    "System"."ParameterizedStrings/LowLevelStack"
);
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl std::ops::Deref for crate::System::ParameterizedStrings_LowLevelStack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl std::ops::DerefMut for crate::System::ParameterizedStrings_LowLevelStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl crate::System::ParameterizedStrings_LowLevelStack {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParameterizedStrings_FormatParam> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ParameterizedStrings_FormatParam = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        item: crate::System::ParameterizedStrings_FormatParam,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (item))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ParameterizedStrings_LowLevelStack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
