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

package com.android.rs.unittest;

import android.content.Context;
import android.renderscript.RenderScript;

public class UT_struct extends UnitTest {

    public UT_struct(Context ctx) {
        super("Struct", ctx);
    }

    public void run() {
        RenderScript pRS = createRenderScript(true);
        ScriptC_struct s = new ScriptC_struct(pRS);

        ScriptField_Point2 p = new ScriptField_Point2(pRS, 1);
        ScriptField_Point2.Item i = new ScriptField_Point2.Item();
        int val = 100;
        i.x = val;
        i.y = val;
        p.set(i, 0, true);
        s.bind_point2(p);
        s.invoke_struct_test(val);
        pRS.finish();

        val = 200;
        p.set_x(0, val, true);
        p.set_y(0, val, true);
        s.invoke_struct_test(val);
        pRS.finish();
        p.getAllocation().destroy();
        p.getElement().destroy();
        s.destroy();
        pRS.destroy();
    }
}
