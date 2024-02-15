use crate::{imp::LottieAnimation, Animation};
use gtk::{
    gio::File,
    glib::{self, translate::*},
};

#[no_mangle]
pub extern "C" fn lottie_animation_get_type() -> glib::ffi::GType {
    <crate::Animation as glib::types::StaticType>::static_type().into_glib()
}

/// lottie_animation_request_draw:
/// @animation: A #LottieAnimation
/// @frame_num: the frame number to draw
///
/// Request draw for a specific frame.
#[no_mangle]
pub extern "C" fn lottie_animation_request_draw(animation: *mut LottieAnimation, frame_num: usize) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.request_draw(frame_num);
}

/// lottie_animation_open:
/// @animation: A #LottieAnimation
/// @file: (transfer full): the lottie file to open
///
/// Open the lottie file.
#[no_mangle]
pub extern "C" fn lottie_animation_open(
    animation: *mut LottieAnimation,
    file: *mut gtk::gio::ffi::GFile,
) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    let file: File = unsafe { from_glib_full(file) };
    this.open(file)
}

/// lottie_animation_set_use_cache:
/// @animation: A #LottieAnimation
/// @value: whether the cache should be used
///
/// Set to use the cache or not.
///
/// By default animation have the cache
/// it uses ram to reduse cpu usage
///
/// and you can disable it when animation
/// plays once and don't need a cache
#[no_mangle]
pub extern "C" fn lottie_animation_set_use_cache(animation: *mut LottieAnimation, value: bool) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.set_use_cache(value);
}

/// lottie_animation_is_reversed:
/// @animation: A #LottieAnimation
///
/// Reversed frame order.
#[no_mangle]
pub extern "C" fn lottie_animation_is_reversed(animation: *mut LottieAnimation) -> bool {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.is_reversed()
}

/// lottie_animation_set_reversed:
/// @animation: A #LottieAnimation
/// @value: whether the frame order should be reversed
///
/// Sets reversed or default frame order.
#[no_mangle]
pub extern "C" fn lottie_animation_set_reversed(animation: *mut LottieAnimation, value: bool) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.set_reversed(value);
}

/// lottie_animation_progress:
/// @animation: A #LottieAnimation
///
/// Returns current progress.
#[no_mangle]
pub extern "C" fn lottie_animation_progress(animation: *mut LottieAnimation) -> f64 {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.progress()
}

/// lottie_animation_set_progress:
/// @animation: A #LottieAnimation
/// @value: the progress to set
///
/// Sets current progress.
#[no_mangle]
pub extern "C" fn lottie_animation_set_progress(animation: *mut LottieAnimation, value: f64) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.set_progress(value);
}

// Media functions

/// lottie_animation_is_playing:
/// @animation: A #LottieAnimation
///
/// Return whether the animation is currently playing.
#[no_mangle]
pub extern "C" fn lottie_animation_is_playing(animation: *mut LottieAnimation) -> bool {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.is_playing()
}

/// lottie_animation_play:
/// @animation: A #LottieAnimation
///
/// Play the animation.
#[no_mangle]
pub extern "C" fn lottie_animation_play(animation: *mut LottieAnimation) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.play()
}

/// lottie_animation_pause:
/// @animation: A #LottieAnimation
///
/// Pause the animation.
#[no_mangle]
pub extern "C" fn lottie_animation_pause(animation: *mut LottieAnimation) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.pause()
}

/// lottie_animation_is_loop:
/// @animation: A #LottieAnimation
///
/// Returns whether the animation is set to loop.
#[no_mangle]
pub extern "C" fn lottie_animation_is_loop(animation: *mut LottieAnimation) -> bool {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.is_loop()
}

/// lottie_animation_set_loop:
/// @animation: A #LottieAnimation
/// @loop_: whether the animation should loop
///
/// Sets whether the animation should loop.
#[no_mangle]
pub extern "C" fn lottie_animation_set_loop(animation: *mut LottieAnimation, loop_: bool) {
    let this: &Animation = unsafe { &from_glib_borrow(animation) };
    this.set_loop(loop_);
}
