//
// Copyright (C) 2019 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

//! Re-exports WebRTC JNI interfaces

use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;

extern "C" {
    /// Export the nativeCreatepeerconnection() call from the
    /// org.webrtc.PeerConnectionFactory class.
    pub fn Java_org_webrtc_PeerConnectionFactory_nativeCreatePeerConnection(
        env: JNIEnv,
        class: JClass,
        factory: jlong,
        rtcConfig: JObject,
        constraints: JObject,
        nativeObserver: jlong,
        sslCertificateVerifier: JObject,
    ) -> jlong;
}
