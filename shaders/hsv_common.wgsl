struct HSV { h: f32, s: f32, v: f32 };
fn srgb_to_hsv(r_: f32, g_: f32, b_: f32) -> HSV {
    let r = r_ / 255;
    let g = g_ / 255;
    let b = b_ / 255;

    var max: f32 = max(max(r, g), b);
    var min: f32 = min(min(r, g), b);
    var h: f32 = max;
    var s: f32 = max;
    var v: f32 = max;

    var d: f32 = max - min;
    if max == 0 {
        s = 0.;
    } else {
        s = d / max;
    }

    if (max == min) {
        h = 0.; // achromatic
    } else {
        if max == r {
            h = (g - b) / d;
            if g < b {
                h = h + 6;
            }
        } else if max == g {
            h = (b - r) / d + 2;
        } else if max == b {
            h = (r - g) / d + 4;
        }

        h = h / 6;
    }

    return HSV(h, s, v);
}

fn hsv_to_srgb(h: f32, s: f32, v: f32) -> vec3<f32>{
  var r = 0.;
  var g = 0.;
  var b = 0.;

  let i = floor(h * 6);
  let f = h * 6 - i;
  let p = v * (1 - s);
  let q = v * (1 - f * s);
  let t = v * (1 - (1 - f) * s);

  switch (i32(i) % 6) {
    case 0: { r = v; g = t; b = p; break; }
    case 1: { r = q; g = v; b = p; break; }
    case 2: { r = p; g = v; b = t; break; }
    case 3: { r = p; g = q; b = v; break; }
    case 4: { r = t; g = p; b = v; break; }
    case 5: { r = v; g = p; b = q; break; }
    default: {}
  }

  return vec3<f32>(r, g, b);
}
