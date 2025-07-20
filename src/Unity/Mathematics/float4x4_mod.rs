#[cfg(feature = "Unity+Mathematics+float4x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct float4x4 {
    pub c0: crate::Unity::Mathematics::float4,
    pub c1: crate::Unity::Mathematics::float4,
    pub c2: crate::Unity::Mathematics::float4,
    pub c3: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::float4x4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "float4x4";
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
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::float4x4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::float4x4 {
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
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::float4x4 {
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
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::float4x4 {
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
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float4x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl crate::Unity::Mathematics::float4x4 {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("AxisAngle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "AxisAngle", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (axis, angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_float4x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::Unity::Mathematics::float4x4), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerXYZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerXYZ", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerXYZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerXYZ", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerXZY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerXZY", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerXZY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerXZY", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerYXZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerYXZ", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerYXZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerYXZ", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerYZX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerYZX", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerYZX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerYZX", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerZXY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerZXY", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerZXY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerZXY", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("EulerZYX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerZYX", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("EulerZYX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "EulerZYX", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, crate::Unity::Mathematics::math_RotationOrder),
                crate::Unity::Mathematics::float4x4,
                4usize,
            >("Euler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Euler", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float3,
                    crate::Unity::Mathematics::math_RotationOrder,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("Euler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Euler", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (xyz, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LookAt(
        eye: crate::Unity::Mathematics::float3,
        target: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float3,
                    crate::Unity::Mathematics::float3,
                    crate::Unity::Mathematics::float3,
                ),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("LookAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "LookAt", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (eye, target, up))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Ortho(
        width: f32,
        height: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                4usize,
            >("Ortho")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Ortho", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (width, height, near, far))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrthoOffCenter(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                6usize,
            >("OrthoOffCenter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "OrthoOffCenter", 6usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (left, right, bottom, top, near, far))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PerspectiveFov(
        verticalFov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                4usize,
            >("PerspectiveFov")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "PerspectiveFov", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (verticalFov, aspect, near, far))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PerspectiveOffCenter(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                6usize,
            >("PerspectiveOffCenter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "PerspectiveOffCenter", 6usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (left, right, bottom, top, near, far))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("RotateX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "RotateX", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("RotateY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "RotateY", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("RotateZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "RotateZ", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_0(
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("Scale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Scale", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("Scale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Scale", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Scale_float3_2(
        scales: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("Scale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Scale", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (scales))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TRS(
        translation: crate::Unity::Mathematics::float3,
        rotation: crate::Unity::Mathematics::quaternion,
        scale: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float3,
                    crate::Unity::Mathematics::quaternion,
                    crate::Unity::Mathematics::float3,
                ),
                crate::Unity::Mathematics::float4x4,
                3usize,
            >("TRS")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "TRS", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (translation, rotation, scale))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format, formatProvider))? };
        Ok(__cordl_ret.into())
    }
    pub fn Translate(
        vector: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float3),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("Translate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "Translate", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (vector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RigidTransform13(
        &mut self,
        transform: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::RigidTransform),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (transform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool4x4_4(
        &mut self,
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::bool4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double4x4_10(
        &mut self,
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::double4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_2(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                16usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 16usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        m00,
                        m01,
                        m02,
                        m03,
                        m10,
                        m11,
                        m12,
                        m13,
                        m20,
                        m21,
                        m22,
                        m23,
                        m30,
                        m31,
                        m32,
                        m33,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3x3_float3_11(
        &mut self,
        rotation: crate::Unity::Mathematics::float3x3,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::float3x3, crate::Unity::Mathematics::float3),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rotation, translation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_float4_float4_float4_0(
        &mut self,
        c0: crate::Unity::Mathematics::float4,
        c1: crate::Unity::Mathematics::float4,
        c2: crate::Unity::Mathematics::float4,
        c3: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Unity::Mathematics::float4,
                    crate::Unity::Mathematics::float4,
                    crate::Unity::Mathematics::float4,
                    crate::Unity::Mathematics::float4,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c0, c1, c2, c3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_5(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int4x4_6(
        &mut self,
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_quaternion_float3_12(
        &mut self,
        rotation: crate::Unity::Mathematics::quaternion,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Unity::Mathematics::quaternion,
                    crate::Unity::Mathematics::float3,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rotation, translation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_7(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint4x4_8(
        &mut self,
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::uint4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
                1usize,
            >("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "get_Item", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float4,
        > = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Decrement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Decrement", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4x4_1(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double4x4_3(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::double4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Matrix4x4_5(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Matrix4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (m))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4x4_6(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked((), (m))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int4x4_2(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint4x4_4(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::uint4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_Increment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Increment", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4, f32),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float4x4,
                    crate::Unity::Mathematics::float4x4,
                ),
                crate::Unity::Mathematics::float4x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_UnaryNegation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_UnaryNegation", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float4x4),
                crate::Unity::Mathematics::float4x4,
                1usize,
            >("op_UnaryPlus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::float4x4 as quest_hook::libil2cpp::Type >
                    ::class(), "op_UnaryPlus", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4>>
for crate::Unity::Mathematics::float4x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4>>
for crate::Unity::Mathematics::float4x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float4x4 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float4x4 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
