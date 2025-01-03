#[cfg(feature = "UnityEngine+Cache")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Cache {
    pub m_Handle: i32,
}
#[cfg(feature = "UnityEngine+Cache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Cache => "UnityEngine"."Cache"
);
#[cfg(feature = "UnityEngine+Cache")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Cache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Cache")]
impl crate::UnityEngine::Cache {
    pub fn Cache_GetPath(
        handle: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cache_GetPath", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cache_IsValid(handle: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cache_IsValid", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cache_SetExpirationDelay(
        handle: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cache_SetExpirationDelay", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cache_SetMaximumDiskSpaceAvailable(
        handle: i32,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cache_SetMaximumDiskSpaceAvailable", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Cache1(
        &mut self,
        other: crate::UnityEngine::Cache,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    pub fn get_handle(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_path", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_expirationDelay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_expirationDelay",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumAvailableStorageSpace(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maximumAvailableStorageSpace",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Cache")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Cache>>
for crate::UnityEngine::Cache {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Cache> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Cache")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Cache>>
for crate::UnityEngine::Cache {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Cache> {
        todo!()
    }
}
