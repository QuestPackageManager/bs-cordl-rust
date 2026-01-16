#[cfg(feature = "cordl_class_UnityOpus+Library")]
#[repr(C)]
#[derive(Debug)]
pub struct Library {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityOpus+Library")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityOpus::Library {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityOpus";
    const CLASS_NAME: &'static str = "Library";
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
#[cfg(feature = "UnityOpus+Library")]
impl std::ops::Deref for crate::UnityOpus::Library {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Library")]
impl std::ops::DerefMut for crate::UnityOpus::Library {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Library")]
impl crate::UnityOpus::Library {
    pub const dllName: &'static str = "unityopus";
    pub const maximumPacketDuration: i32 = 5760i32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OpusDecode(
        decoder: crate::System::IntPtr,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        len: i32,
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        frameSize: i32,
        decodeFec: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                        i32,
                        i32,
                    ), i32, 6usize>("OpusDecode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusDecode",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (decoder, data, len, pcm, frameSize, decodeFec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusDecodeFloat(
        decoder: crate::System::IntPtr,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        len: i32,
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        frameSize: i32,
        decodeFec: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                        i32,
                        i32,
                    ), i32, 6usize>("OpusDecodeFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusDecodeFloat",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (decoder, data, len, pcm, frameSize, decodeFec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusDecoderCreate(
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
        error: quest_hook::libil2cpp::ByRefMut<crate::UnityOpus::ErrorCode>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityOpus::SamplingFrequency,
                        crate::UnityOpus::NumChannels,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityOpus::ErrorCode>,
                    ), crate::System::IntPtr, 3usize>("OpusDecoderCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusDecoderCreate",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), (samplingFrequency, channels, error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusDecoderDestroy(
        decoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OpusDecoderDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpusDecoderDestroy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (decoder))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncode(
        encoder: crate::System::IntPtr,
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        frameSize: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        maxDataBytes: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                    ), i32, 5usize>("OpusEncode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusEncode",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (encoder, pcm, frameSize, data, maxDataBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncodeFloat(
        encoder: crate::System::IntPtr,
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        frameSize: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        maxDataBytes: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                    ), i32, 5usize>("OpusEncodeFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusEncodeFloat",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (encoder, pcm, frameSize, data, maxDataBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncoderCreate(
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
        application: crate::UnityOpus::OpusApplication,
        error: quest_hook::libil2cpp::ByRefMut<crate::UnityOpus::ErrorCode>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityOpus::SamplingFrequency,
                        crate::UnityOpus::NumChannels,
                        crate::UnityOpus::OpusApplication,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityOpus::ErrorCode>,
                    ), crate::System::IntPtr, 4usize>("OpusEncoderCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusEncoderCreate",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info
                .invoke_unchecked((), (samplingFrequency, channels, application, error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncoderDestroy(
        encoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OpusEncoderDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpusEncoderDestroy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (encoder))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncoderSetBitrate(
        encoder: crate::System::IntPtr,
        bitrate: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::IntPtr, i32), i32, 2usize>(
                        "OpusEncoderSetBitrate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusEncoderSetBitrate",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (encoder, bitrate))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncoderSetComplexity(
        encoder: crate::System::IntPtr,
        complexity: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::IntPtr, i32), i32, 2usize>(
                        "OpusEncoderSetComplexity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusEncoderSetComplexity",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (encoder, complexity))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpusEncoderSetSignal(
        encoder: crate::System::IntPtr,
        signal: crate::UnityOpus::OpusSignal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::UnityOpus::OpusSignal),
                        i32,
                        2usize,
                    >("OpusEncoderSetSignal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpusEncoderSetSignal", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (encoder, signal))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpusPcmSoftClip(
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        frameSize: i32,
        channels: crate::UnityOpus::NumChannels,
        softclipMem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                        i32,
                        crate::UnityOpus::NumChannels,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 4usize>("OpusPcmSoftClip")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OpusPcmSoftClip",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (pcm, frameSize, channels, softclipMem))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityOpus+Library")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityOpus::Library {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
