#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OSPlatform {
    pub _osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::OSPlatform =>
    "System.Runtime.InteropServices"."OSPlatform"
);
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::InteropServices::OSPlatform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl crate::System::Runtime::InteropServices::OSPlatform {
    pub fn Create(
        osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (osPlatform))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppString1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OSPlatform0(
        &mut self,
        other: crate::System::Runtime::InteropServices::OSPlatform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (osPlatform),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Linux() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Linux", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OSX() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OSX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Windows() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Windows", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Runtime::InteropServices::OSPlatform,
        right: crate::System::Runtime::InteropServices::OSPlatform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl AsRef<
    crate::System::IEquatable_1<crate::System::Runtime::InteropServices::OSPlatform>,
> for crate::System::Runtime::InteropServices::OSPlatform {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl AsMut<
    crate::System::IEquatable_1<crate::System::Runtime::InteropServices::OSPlatform>,
> for crate::System::Runtime::InteropServices::OSPlatform {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        todo!()
    }
}
