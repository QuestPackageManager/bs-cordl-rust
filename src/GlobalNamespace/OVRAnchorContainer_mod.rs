#[cfg(feature = "OVRAnchorContainer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRAnchorContainer {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRAnchorContainer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRAnchorContainer => ""
    ."OVRAnchorContainer"
);
#[cfg(feature = "OVRAnchorContainer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRAnchorContainer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchorContainer")]
impl crate::GlobalNamespace::OVRAnchorContainer {
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
    pub fn Equals_OVRAnchorContainer0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchorContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchChildrenAsync(
        &mut self,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FetchChildrenAsync",
            (anchors),
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
    pub fn IOVRAnchorComponent_OVRAnchorContainer__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRAnchorContainer> {
        let __cordl_ret: crate::GlobalNamespace::OVRAnchorContainer = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRAnchorContainer>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRAnchorContainer__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRAnchorContainer>.SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRAnchorContainer__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRAnchorContainer>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRAnchorContainer__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRAnchorContainer>.get_Type",
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
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
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
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Uuids", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRAnchorContainer,
        rhs: crate::GlobalNamespace::OVRAnchorContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRAnchorContainer,
        rhs: crate::GlobalNamespace::OVRAnchorContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchorContainer")]
impl AsRef<
    crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRAnchorContainer,
    >,
> for crate::GlobalNamespace::OVRAnchorContainer {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRAnchorContainer,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRAnchorContainer")]
impl AsMut<
    crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRAnchorContainer,
    >,
> for crate::GlobalNamespace::OVRAnchorContainer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRAnchorContainer,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRAnchorContainer")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchorContainer>>
for crate::GlobalNamespace::OVRAnchorContainer {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchorContainer> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchorContainer")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchorContainer>>
for crate::GlobalNamespace::OVRAnchorContainer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchorContainer> {
        todo!()
    }
}
