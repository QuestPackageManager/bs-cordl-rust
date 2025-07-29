#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct quaternion {
    pub value: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::quaternion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "quaternion";
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
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::quaternion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::quaternion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::quaternion {
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
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::quaternion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::quaternion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl crate::Unity::Mathematics::quaternion {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3, f32),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("AxisAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AxisAngle", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (axis, angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_quaternion0(
        &mut self,
        x: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::quaternion),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerXYZ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerXYZ", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerXYZ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerXYZ", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerXZY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerXZY", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerXZY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerXZY", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerYXZ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerYXZ", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerYXZ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerYXZ", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerYZX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerYZX", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerYZX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerYZX", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerZXY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerZXY", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerZXY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerZXY", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("EulerZYX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerZYX", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("EulerZYX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EulerZYX", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32, crate::Unity::Mathematics::math_RotationOrder),
                        crate::Unity::Mathematics::quaternion,
                        4usize,
                    >("Euler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Euler",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y, z, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::math_RotationOrder,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("Euler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Euler",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (xyz, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LookRotation(
        forward: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("LookRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LookRotation", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, up))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LookRotationSafe(
        forward: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("LookRotationSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LookRotationSafe", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, up))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("RotateX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RotateX",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("RotateY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RotateY",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("RotateZ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RotateZ",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (format, formatProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_0(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y, z, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3x3_2(
        &mut self,
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3x3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (m))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_1(
        &mut self,
        value: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float4),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x4_3(
        &mut self,
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float4x4),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (m))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Quaternion1(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Quaternion),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4_2(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_quaternion0(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::quaternion),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::quaternion {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::quaternion {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
