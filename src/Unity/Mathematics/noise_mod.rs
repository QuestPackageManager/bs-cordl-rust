#[cfg(feature = "cordl_class_Unity+Mathematics+noise")]
#[repr(C)]
#[derive(Debug)]
pub struct noise {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Mathematics+noise")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::noise {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "noise";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl std::ops::Deref for crate::Unity::Mathematics::noise {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl std::ops::DerefMut for crate::Unity::Mathematics::noise {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl crate::Unity::Mathematics::noise {
    pub fn cellular2x2(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("cellular2x2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "cellular2x2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cellular2x2x2(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("cellular2x2x2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "cellular2x2x2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cellular_float2_0(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("cellular")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "cellular", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cellular_float3_1(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("cellular")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "cellular", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float2_0(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float2), f32, 1usize>(
                        "cnoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "cnoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float3_1(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float3), f32, 1usize>(
                        "cnoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "cnoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float4_2(
        P: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float4), f32, 1usize>(
                        "cnoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "cnoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P))? };
        Ok(__cordl_ret.into())
    }
    pub fn fade_float2_0(
        t: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("fade")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "fade",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn fade_float3_1(
        t: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("fade")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "fade",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn fade_float4_2(
        t: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("fade")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "fade",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn grad4(
        j: f32,
        ip: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        2usize,
                    >("grad4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "grad4",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (j, ip))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod289_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("mod289")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "mod289",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("mod289")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "mod289",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("mod289")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "mod289",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("mod289")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "mod289",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod7_float3_0(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("mod7")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "mod7",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn mod7_float4_1(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("mod7")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "mod7",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn permute_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("permute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "permute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn permute_float3_1(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("permute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "permute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn permute_float4_2(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("permute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "permute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float2_float2_0(
        P: crate::Unity::Mathematics::float2,
        rep: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float2,
                        crate::Unity::Mathematics::float2,
                    ), f32, 2usize>("pnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "pnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P, rep))? };
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float3_float3_1(
        P: crate::Unity::Mathematics::float3,
        rep: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float3,
                        crate::Unity::Mathematics::float3,
                    ), f32, 2usize>("pnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "pnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P, rep))? };
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float4_float4_2(
        P: crate::Unity::Mathematics::float4,
        rep: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float4,
                        crate::Unity::Mathematics::float4,
                    ), f32, 2usize>("pnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "pnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (P, rep))? };
        Ok(__cordl_ret.into())
    }
    pub fn psrdnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float2,
                        crate::Unity::Mathematics::float2,
                        f32,
                    ), crate::Unity::Mathematics::float3, 3usize>("psrdnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "psrdnoise",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (pos, per, rot))? };
        Ok(__cordl_ret.into())
    }
    pub fn psrdnoise_float2_float2_1(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float2,
                        crate::Unity::Mathematics::float2,
                    ), crate::Unity::Mathematics::float3, 2usize>("psrdnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "psrdnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (pos, per))? };
        Ok(__cordl_ret.into())
    }
    pub fn psrnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float2,
                        crate::Unity::Mathematics::float2,
                        f32,
                    ), f32, 3usize>("psrnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "psrnoise",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (pos, per, rot))? };
        Ok(__cordl_ret.into())
    }
    pub fn psrnoise_float2_float2_1(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float2,
                        crate::Unity::Mathematics::float2,
                    ), f32, 2usize>("psrnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "psrnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (pos, per))? };
        Ok(__cordl_ret.into())
    }
    pub fn rgrad2(
        p: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2, f32),
                        crate::Unity::Mathematics::float2,
                        2usize,
                    >("rgrad2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rgrad2",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 =
            unsafe { cordl_method_info.invoke_unchecked((), (p, rot))? };
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float2_0(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float2), f32, 1usize>(
                        "snoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "snoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float3_1(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float3), f32, 1usize>(
                        "snoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "snoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float3_ByRefMut2(
        v: crate::Unity::Mathematics::float3,
        gradient: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float3,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
                    ), f32, 2usize>("snoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "snoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (v, gradient))? };
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float4_3(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float4), f32, 1usize>(
                        "snoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "snoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn srdnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2, f32),
                        crate::Unity::Mathematics::float3,
                        2usize,
                    >("srdnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "srdnoise", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (pos, rot))? };
        Ok(__cordl_ret.into())
    }
    pub fn srdnoise_float2_1(
        pos: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("srdnoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "srdnoise", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 =
            unsafe { cordl_method_info.invoke_unchecked((), (pos))? };
        Ok(__cordl_ret.into())
    }
    pub fn srnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float2, f32), f32, 2usize>(
                        "srnoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "srnoise",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (pos, rot))? };
        Ok(__cordl_ret.into())
    }
    pub fn srnoise_float2_1(
        pos: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Mathematics::float2), f32, 1usize>(
                        "srnoise",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "srnoise",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (pos))? };
        Ok(__cordl_ret.into())
    }
    pub fn taylorInvSqrt_f32_0(r: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("taylorInvSqrt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "taylorInvSqrt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (r))? };
        Ok(__cordl_ret.into())
    }
    pub fn taylorInvSqrt_float4_1(
        r: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("taylorInvSqrt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "taylorInvSqrt", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 =
            unsafe { cordl_method_info.invoke_unchecked((), (r))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+noise")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::noise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
