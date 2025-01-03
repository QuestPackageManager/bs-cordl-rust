#[cfg(feature = "OVRSpace")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpace {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpace => ""."OVRSpace"
);
#[cfg(feature = "OVRSpace")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRSpace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpace")]
impl crate::GlobalNamespace::OVRSpace {
    #[cfg(feature = "OVRSpace+StorageLocation")]
    pub type StorageLocation = crate::GlobalNamespace::OVRSpace_StorageLocation;
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_OVRSpace0(
        &mut self,
        other: crate::GlobalNamespace::OVRSpace,
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
    pub fn TryGetUuid(
        &mut self,
        uuid: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetUuid",
            (uuid),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRSpace,
        rhs: crate::GlobalNamespace::OVRSpace,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_OVRSpace1(
        space: crate::GlobalNamespace::OVRSpace,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (space))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u64_0(
        handle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSpace> {
        let __cordl_ret: crate::GlobalNamespace::OVRSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRSpace,
        rhs: crate::GlobalNamespace::OVRSpace,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSpace")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRSpace>>
for crate::GlobalNamespace::OVRSpace {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRSpace> {
        todo!()
    }
}
#[cfg(feature = "OVRSpace")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRSpace>>
for crate::GlobalNamespace::OVRSpace {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRSpace> {
        todo!()
    }
}
#[cfg(feature = "OVRSpace+StorageLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRSpace_StorageLocation {
    Cloud = 1i32,
    Local = 0i32,
}
#[cfg(feature = "OVRSpace+StorageLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpace_StorageLocation => ""
    ."OVRSpace/StorageLocation"
);
