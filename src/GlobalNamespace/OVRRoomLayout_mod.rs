#[cfg(feature = "OVRRoomLayout")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRRoomLayout {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRRoomLayout {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRRoomLayout";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRRoomLayout {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRRoomLayout {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRRoomLayout {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRRoomLayout {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRRoomLayout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl crate::GlobalNamespace::OVRRoomLayout {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRRoomLayout0(
        &mut self,
        other: crate::GlobalNamespace::OVRRoomLayout,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRRoomLayout),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other)) };
        Ok(__cordl_ret.into())
    }
    pub fn FetchLayoutAnchorsAsync(
        &mut self,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::GlobalNamespace::OVRAnchor,
                    >,
                >),
                crate::GlobalNamespace::OVRTask_1<bool>,
                1usize,
            >("FetchLayoutAnchorsAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FetchLayoutAnchorsAsync", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            method.invoke_unchecked(self, (anchors))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRRoomLayout> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRAnchor),
                crate::GlobalNamespace::OVRRoomLayout,
                1usize,
            >("IOVRAnchorComponent<OVRRoomLayout>.FromAnchor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IOVRAnchorComponent<OVRRoomLayout>.FromAnchor", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRRoomLayout = unsafe {
            method.invoke_unchecked(self, (anchor))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, f64),
                crate::GlobalNamespace::OVRTask_1<bool>,
                2usize,
            >("IOVRAnchorComponent<OVRRoomLayout>.SetEnabledAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IOVRAnchorComponent<OVRRoomLayout>.SetEnabledAsync", 2usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            method.invoke_unchecked(self, (enabled, timeout))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                u64,
                0usize,
            >("IOVRAnchorComponent<OVRRoomLayout>.get_Handle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IOVRAnchorComponent<OVRRoomLayout>.get_Handle", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                0usize,
            >("IOVRAnchorComponent<OVRRoomLayout>.get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IOVRAnchorComponent<OVRRoomLayout>.get_Type", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetRoomLayout(
        &mut self,
        ceiling: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        floor: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        walls: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("TryGetRoomLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetRoomLayout", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ceiling, floor, walls))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRAnchor),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (anchor))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u64, 0usize>("get_Handle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Handle", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsEnabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsNull", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Type", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRRoomLayout,
        rhs: crate::GlobalNamespace::OVRRoomLayout,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::OVRRoomLayout,
                    crate::GlobalNamespace::OVRRoomLayout,
                ),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRRoomLayout,
        rhs: crate::GlobalNamespace::OVRRoomLayout,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::OVRRoomLayout,
                    crate::GlobalNamespace::OVRRoomLayout,
                ),
                bool,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl AsRef<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRRoomLayout>,
> for crate::GlobalNamespace::OVRRoomLayout {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRRoomLayout,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl AsMut<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRRoomLayout>,
> for crate::GlobalNamespace::OVRRoomLayout {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRRoomLayout,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRRoomLayout>>
for crate::GlobalNamespace::OVRRoomLayout {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRRoomLayout> {
        todo!()
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRRoomLayout>>
for crate::GlobalNamespace::OVRRoomLayout {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRRoomLayout> {
        todo!()
    }
}
