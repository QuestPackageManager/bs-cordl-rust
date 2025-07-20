#[cfg(feature = "OVRPose")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPose {
    pub position: crate::UnityEngine::Vector3,
    pub orientation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRPose {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRPose";
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
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRPose {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRPose {
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
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRPose {
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
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRPose {
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
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRPose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPose")]
impl crate::GlobalNamespace::OVRPose {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Inverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::GlobalNamespace::OVRPose, 0usize>("Inverse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "Inverse", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Rotate180AlongX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRPose,
                0usize,
            >("Rotate180AlongX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "Rotate180AlongX", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToPosef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRPlugin_Posef,
                0usize,
            >("ToPosef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "ToPosef", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToPosef_Legacy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRPlugin_Posef,
                0usize,
            >("ToPosef_Legacy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "ToPosef_Legacy", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn flipZ(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::GlobalNamespace::OVRPose, 0usize>("flipZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "flipZ", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPose,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::GlobalNamespace::OVRPose,
                0usize,
            >("get_identity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "get_identity", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::GlobalNamespace::OVRPose,
        y: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::OVRPose, crate::GlobalNamespace::OVRPose),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        x: crate::GlobalNamespace::OVRPose,
        y: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::OVRPose, crate::GlobalNamespace::OVRPose),
                bool,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        lhs: crate::GlobalNamespace::OVRPose,
        rhs: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::OVRPose, crate::GlobalNamespace::OVRPose),
                crate::GlobalNamespace::OVRPose,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRPose as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
}
