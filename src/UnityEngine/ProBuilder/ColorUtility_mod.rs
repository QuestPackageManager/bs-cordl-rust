#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ColorUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ColorUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ColorUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ColorUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl crate::UnityEngine::ProBuilder::ColorUtility {
    pub fn CIELabFromRGB(
        R: f32,
        G: f32,
        B: f32,
        Scale: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
                4usize,
            >("CIELabFromRGB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CIELabFromRGB", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::CIELabColor,
        > = unsafe { method.invoke_unchecked((), (R, G, B, Scale)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeltaE(
        lhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
        rhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::CIELabColor,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::CIELabColor,
                    >,
                ),
                f32,
                2usize,
            >("DeltaE")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeltaE", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetColor(
        vec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Color,
                1usize,
            >("GetColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetColor", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (vec))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorName(
        InColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetColorName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetColorName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (InColor)) };
        Ok(__cordl_ret.into())
    }
    pub fn HSVtoRGB_HSVColor0(
        hsv: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>),
                crate::UnityEngine::Color,
                1usize,
            >("HSVtoRGB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HSVtoRGB", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (hsv))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HSVtoRGB_f32_f32_f32_1(
        h: f32,
        s: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::UnityEngine::Color,
                3usize,
            >("HSVtoRGB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HSVtoRGB", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), (h, s, v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RGBToXYZ_Color0(
        col: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
                1usize,
            >("RGBToXYZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RGBToXYZ", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::XYZColor,
        > = unsafe { method.invoke_unchecked((), (col)) };
        Ok(__cordl_ret.into())
    }
    pub fn RGBToXYZ_f32_f32_f32_1(
        r: f32,
        g: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
                3usize,
            >("RGBToXYZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RGBToXYZ", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::XYZColor,
        > = unsafe { method.invoke_unchecked((), (r, g, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn RGBtoHSV(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>,
                1usize,
            >("RGBtoHSV")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RGBtoHSV", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::HSVColor,
        > = unsafe { method.invoke_unchecked((), (color)) };
        Ok(__cordl_ret.into())
    }
    pub fn XYZToCIE_Lab(
        xyz: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
                1usize,
            >("XYZToCIE_Lab")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "XYZToCIE_Lab", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::CIELabColor,
        > = unsafe { method.invoke_unchecked((), (xyz)) };
        Ok(__cordl_ret.into())
    }
    pub fn approx(lhs: f32, rhs: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), bool, 2usize>("approx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "approx", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ColorUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
