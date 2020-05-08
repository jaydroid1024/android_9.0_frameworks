/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "shared.rsh"

static bool test_clamp_vector() {
    bool failed = false;

    float2 src2 = { 2.0f, 2.0f};
    float2 min2 = { 0.5f, -3.0f};
    float2 max2 = { 1.0f, 9.0f};

    float2 res2 = clamp(src2, min2, max2);
    _RS_ASSERT(res2.x == 1.0f);
    _RS_ASSERT(res2.y == 2.0f);


    float3 src3 = { 2.0f, 2.0f, 1.0f};
    float3 min3 = { 0.5f, -3.0f, 3.0f};
    float3 max3 = { 1.0f, 9.0f, 4.0f};

    float3 res3 = clamp(src3, min3, max3);
    _RS_ASSERT(res3.x == 1.0f);
    _RS_ASSERT(res3.y == 2.0f);
    _RS_ASSERT(res3.z == 3.0f);


    float4 src4 = { 2.0f, 2.0f, 1.0f, 4.0f };
    float4 min4 = { 0.5f, -3.0f, 3.0f, 4.0f };
    float4 max4 = { 1.0f, 9.0f, 4.0f, 4.0f };

    float4 res4 = clamp(src4, min4, max4);
    _RS_ASSERT(res4.x == 1.0f);
    _RS_ASSERT(res4.y == 2.0f);
    _RS_ASSERT(res4.z == 3.0f);
    _RS_ASSERT(res4.w == 4.0f);

    if (failed) {
        rsDebug("test_clamp_vector FAILED", 0);
    }
    else {
        rsDebug("test_clamp_vector PASSED", 0);
    }

    return failed;
}

void clamp_test() {
    bool failed = false;
    failed |= test_clamp_vector();

    if (failed) {
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
    else {
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    }
}

