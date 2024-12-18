#[cfg(feature = "OVRAnchor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRAnchor {
    pub _Handle_k__BackingField: u64,
    pub _Uuid_k__BackingField: crate::System::Guid,
}
#[cfg(feature = "OVRAnchor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRAnchor => ""."OVRAnchor"
);
#[cfg(feature = "OVRAnchor")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor")]
impl crate::GlobalNamespace::OVRAnchor {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn Equals_OVRAnchor0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetComponent",
            (),
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
    pub fn SupportsComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SupportsComponent",
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
    pub fn TryGetComponent<T>(
        &mut self,
        component: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetComponent",
            (component),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: u64,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle, uuid),
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
    pub fn get_Uuid(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uuid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
for crate::GlobalNamespace::OVRAnchor {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
for crate::GlobalNamespace::OVRAnchor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
