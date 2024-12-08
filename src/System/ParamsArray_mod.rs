#[cfg(feature = "System+ParamsArray")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ParamsArray {
    pub _arg0: *mut crate::System::Object,
    pub _arg1: *mut crate::System::Object,
    pub _arg2: *mut crate::System::Object,
    pub _args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
}
#[cfg(feature = "System+ParamsArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParamsArray => "System"."ParamsArray"
);
#[cfg(feature = "System+ParamsArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::ParamsArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ParamsArray")]
impl crate::System::ParamsArray {
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object0(
        &mut self,
        arg0: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (arg0),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_Object1(
        &mut self,
        arg0: *mut crate::System::Object,
        arg1: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (arg0, arg1),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_Object_Object2(
        &mut self,
        arg0: *mut crate::System::Object,
        arg1: *mut crate::System::Object,
        arg2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (arg0, arg1, arg2),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray3(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (args),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetAtSlow(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAtSlow",
            (index),
        )?;
        Ok(__cordl_ret)
    }
}
